use crate::config;
use crate::error::Result;
use crate::sources::{SourceEntry, SourceType};
use crate::utils;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

/// Manages all APT source entries from sources.list and sources.list.d
pub struct SourcesList {
    /// All source entries loaded from files
    pub entries: Vec<SourceEntry>,
    /// Backup extension for file backups
    backup_ext: Option<String>,
    /// Track which files have been modified (need backup and rewrite)
    pub(crate) modified_files: HashSet<PathBuf>,
}

impl SourcesList {
    /// Create a new SourcesList and load all sources
    pub fn new() -> Result<Self> {
        let mut sources_list = Self {
            entries: Vec::new(),
            backup_ext: None,
            modified_files: HashSet::new(),
        };
        sources_list.load_all()?;
        Ok(sources_list)
    }

    /// Create an empty SourcesList without loading
    pub fn empty() -> Self {
        Self {
            entries: Vec::new(),
            backup_ext: None,
            modified_files: HashSet::new(),
        }
    }

    /// Load all sources from sources.list and sources.list.d
    pub fn load_all(&mut self) -> Result<()> {
        // Load main sources.list
        let main_file = Path::new(config::SOURCES_LIST_PATH);
        if main_file.exists() {
            self.load_file(main_file)?;
        }

        // Load sources.list.d/*.list files (one-line format)
        let list_files = utils::list_sources_list_d_files()?;
        for file in list_files {
            self.load_file(&file)?;
        }

        // Load sources.list.d/*.sources files (DEB822 format)
        let sources_files = utils::list_sources_list_d_deb822_files()?;
        for file in sources_files {
            self.load_deb822_file(&file)?;
        }

        Ok(())
    }

    /// Load sources from a single file (one-line format)
    pub fn load_file(&mut self, path: &Path) -> Result<()> {
        let content = utils::read_file_to_string(path)?;
        
        for line in content.lines() {
            if let Some(entry) = SourceEntry::from_line(line, path.to_path_buf()) {
                self.entries.push(entry);
            }
        }

        Ok(())
    }

    /// Load sources from a DEB822 format file
    pub fn load_deb822_file(&mut self, path: &Path) -> Result<()> {
        let content = utils::read_file_to_string(path)?;
        let stanzas = crate::deb822::parse_deb822_file(&content, path.to_path_buf())?;

        // Expand each stanza into individual SourceEntry objects
        for stanza in stanzas {
            if !stanza.enabled {
                continue; // Skip disabled stanzas
            }

            let oneline_entries = crate::deb822::expand_to_oneline(&stanza);
            for (source_type, uri, suite, components) in oneline_entries {
                let mut entry = SourceEntry::new(
                    source_type,
                    uri,
                    suite,
                    components,
                );
                entry.file = path.to_path_buf();
                entry.line = format!("{} {} {} {}", 
                    entry.entry_type.as_str(),
                    entry.uri,
                    entry.dist,
                    entry.components.join(" ")
                );
                self.entries.push(entry);
            }
        }

        Ok(())
    }

    /// Add a new source entry
    /// Returns the index of the added or modified entry
    pub fn add(
        &mut self,
        entry_type: SourceType,
        uri: String,
        dist: String,
        components: Vec<String>,
        disabled: bool,
        file: Option<PathBuf>,
    ) -> Result<usize> {
        let file = file.unwrap_or_else(|| PathBuf::from(config::SOURCES_LIST_PATH));

        // Check for exact match (including disabled state)
        for (idx, existing) in self.entries.iter_mut().enumerate() {
            if existing.entry_type == entry_type
                && existing.uri == uri
                && existing.dist == dist
                && existing.disabled == disabled
            {
                // Check if all components already exist
                let mut new_comps = components.clone();
                new_comps.retain(|c| !existing.components.contains(c));
                
                if new_comps.is_empty() {
                    // All components already present
                    return Ok(idx);
                }
                
                // Add missing components
                existing.components.extend(new_comps);
                existing.line = existing.to_line();
                // Mark file as modified
                self.modified_files.insert(existing.file.clone());
                return Ok(idx);
            }
        }

        // Check for disabled match that can be enabled
        for (idx, existing) in self.entries.iter_mut().enumerate() {
            if existing.entry_type == entry_type
                && existing.uri == uri
                && existing.dist == dist
                && existing.disabled
                && !disabled
                && existing.components == components
            {
                // Enable the existing disabled entry
                existing.set_enabled(true);
                existing.line = existing.to_line();
                // Mark file as modified
                self.modified_files.insert(existing.file.clone());
                return Ok(idx);
            }
        }

        // Create new entry
        let mut new_entry = SourceEntry::new(entry_type, uri, dist, components);
        new_entry.disabled = disabled;
        new_entry.file = file.clone();
        new_entry.line = new_entry.to_line();
        
        // Mark file as modified (new entry being added)
        self.modified_files.insert(file);
        
        self.entries.push(new_entry);
        Ok(self.entries.len() - 1)
    }

    /// Remove a source entry
    pub fn remove(&mut self, entry: &SourceEntry) -> bool {
        if let Some(pos) = self.entries.iter().position(|e| e.matches(entry)) {
            let removed_entry = self.entries.remove(pos);
            // Mark file as modified
            self.modified_files.insert(removed_entry.file);
            return true;
        }
        false
    }

    /// Find entries matching criteria
    pub fn find(
        &self,
        entry_type: Option<SourceType>,
        uri: Option<&str>,
        dist: Option<&str>,
    ) -> Vec<&SourceEntry> {
        self.entries
            .iter()
            .filter(|e| {
                entry_type.map_or(true, |t| e.entry_type == t)
                    && uri.map_or(true, |u| e.uri.trim_end_matches('/') == u.trim_end_matches('/'))
                    && dist.map_or(true, |d| e.dist == d)
            })
            .collect()
    }

    /// Check if a source entry already exists
    pub fn contains(&self, entry: &SourceEntry) -> bool {
        self.entries.iter().any(|e| e.matches(entry))
    }

    /// Enable or disable all matching entries
    pub fn set_enabled(
        &mut self,
        entry_type: SourceType,
        uri: &str,
        dist: &str,
        enabled: bool,
    ) -> usize {
        let mut count = 0;
        for entry in &mut self.entries {
            if entry.entry_type == entry_type
                && entry.uri.trim_end_matches('/') == uri.trim_end_matches('/')
                && entry.dist == dist
            {
                entry.set_enabled(enabled);
                entry.line = entry.to_line();
                // Mark file as modified
                self.modified_files.insert(entry.file.clone());
                count += 1;
            }
        }
        count
    }

    /// Backup all source files with .save extension (matches Python version behavior)
    pub fn backup(&mut self) -> Result<String> {
        let backup_ext = ".save".to_string();
        self.backup_with_ext(&backup_ext)?;
        self.backup_ext = Some(backup_ext.clone());
        Ok(backup_ext)
    }

    /// Backup all source files with a specific extension
    pub fn backup_with_ext(&self, ext: &str) -> Result<()> {
        let mut backed_up = std::collections::HashSet::new();

        for entry in &self.entries {
            let file_path = &entry.file;
            if file_path.exists() && !backed_up.contains(file_path) {
                let backup_path = file_path.with_extension(format!("{}{}", 
                    file_path.extension().and_then(|s| s.to_str()).unwrap_or(""),
                    ext
                ));
                fs::copy(file_path, backup_path)?;
                backed_up.insert(file_path.clone());
            }
        }

        Ok(())
    }

    /// Restore from backup with the given extension
    pub fn restore_backup(&self, backup_ext: &str) -> Result<()> {
        let main_file = Path::new(config::SOURCES_LIST_PATH);
        let backup_file = main_file.with_extension(format!("list{}", backup_ext));
        
        if backup_file.exists() && main_file.exists() {
            fs::copy(backup_file, main_file)?;
        }

        // Restore sources.list.d files
        let sources_d = Path::new(config::SOURCES_LIST_D_PATH);
        if sources_d.exists() {
            for entry in fs::read_dir(sources_d)? {
                let entry = entry?;
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.ends_with(backup_ext) {
                        // This is a backup file, find the original
                        let original = path.with_extension("");
                        if original.exists() {
                            fs::copy(&path, &original)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Save all sources back to their respective files
    pub fn save(&self) -> Result<()> {
        if self.entries.is_empty() {
            // Write empty sources.list with header
            let header = "## See sources.list(5) for more information\n\
                          # Remember that you can only use http, ftp or file URIs\n\
                          # CDROMs are managed through the apt-cdrom tool.\n";
            utils::write_string_to_file(Path::new(config::SOURCES_LIST_PATH), header, 0o644)?;
            return Ok(());
        }

        // Group entries by file
        let mut files_content: HashMap<PathBuf, Vec<&SourceEntry>> = HashMap::new();
        for entry in &self.entries {
            files_content
                .entry(entry.file.clone())
                .or_default()
                .push(entry);
        }

        // Write each file ONLY if it was modified
        for (file_path, entries) in files_content {
            // Skip files that haven't been modified
            if !self.modified_files.contains(&file_path) {
                continue;
            }

            // Backup existing file before modifying (only if it exists)
            if file_path.exists() {
                self.backup_file(&file_path)?;
            }

            // Detect file format by extension
            let is_deb822 = file_path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "sources")
                .unwrap_or(false);

            if is_deb822 {
                // Save as DEB822 format (.sources files)
                self.save_as_deb822(&file_path, &entries)?;
            } else {
                // Save as one-line format (.list files)
                let mut content = String::new();
                for entry in entries {
                    content.push_str(&entry.line);
                    content.push('\n');
                }
                utils::write_string_to_file(&file_path, &content, 0o644)?;
            }
        }

        Ok(())
    }

    /// Backup a single file with .save extension
    fn backup_file(&self, file_path: &Path) -> Result<()> {
        let backup_path = file_path.with_extension(format!("{}.save", 
            file_path.extension().and_then(|s| s.to_str()).unwrap_or("")
        ));
        fs::copy(file_path, backup_path)?;
        Ok(())
    }

    /// Save entries as DEB822 format (for .sources files)
    fn save_as_deb822(&self, file_path: &Path, entries: &[&SourceEntry]) -> Result<()> {
        use crate::deb822::{Deb822Stanza, write_deb822_file};

        // Group entries by (uri, dist, components) to create stanzas
        let mut stanza_map: HashMap<(String, String, Vec<String>), Vec<SourceType>> = HashMap::new();

        for entry in entries {
            let key = (
                entry.uri.clone(),
                entry.dist.clone(),
                entry.components.clone(),
            );
            stanza_map
                .entry(key)
                .or_default()
                .push(entry.entry_type);
        }

        // Create DEB822 stanzas from grouped entries
        let mut stanzas = Vec::new();
        for ((uri, dist, components), types) in stanza_map {
            let mut stanza = Deb822Stanza::new(file_path.to_path_buf());
            
            // Deduplicate types
            let mut unique_types = types.clone();
            unique_types.sort();
            unique_types.dedup();
            stanza.types = unique_types;
            
            stanza.uris = vec![uri];
            stanza.suites = vec![dist];
            stanza.components = components;
            stanza.enabled = true;

            // Try to preserve Signed-By if it was in the original file
            // Check if any entry has a keyring reference (for PPA/custom repos)
            // For system files, use the default Ubuntu keyring
            if file_path.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n == "ubuntu.sources")
                .unwrap_or(false)
            {
                stanza.signed_by = Some("/usr/share/keyrings/ubuntu-archive-keyring.gpg".to_string());
            }

            stanzas.push(stanza);
        }

        write_deb822_file(file_path, &stanzas)?;
        Ok(())
    }

    /// Get count of entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for SourcesList {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_sources_list(dir: &Path) -> Result<PathBuf> {
        let sources_file = dir.join("sources.list");
        let mut file = fs::File::create(&sources_file)?;
        writeln!(file, "deb http://archive.ubuntu.com/ubuntu noble main restricted")?;
        writeln!(file, "deb-src http://archive.ubuntu.com/ubuntu noble main")?;
        writeln!(file, "# deb http://example.com/ubuntu noble universe")?;
        Ok(sources_file)
    }

    #[test]
    fn test_load_file() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let sources_file = create_test_sources_list(temp_dir.path())?;

        let mut sl = SourcesList::empty();
        sl.load_file(&sources_file)?;

        assert_eq!(sl.len(), 3);
        assert_eq!(sl.entries[0].entry_type, SourceType::Binary);
        assert_eq!(sl.entries[0].uri, "http://archive.ubuntu.com/ubuntu");
        assert!(!sl.entries[0].disabled);
        assert!(sl.entries[2].disabled);

        Ok(())
    }

    #[test]
    fn test_add_new_entry() -> Result<()> {
        let mut sl = SourcesList::empty();
        
        sl.add(
            SourceType::Binary,
            "http://example.com/ubuntu".to_string(),
            "noble".to_string(),
            vec!["main".to_string()],
            false,
            None,
        )?;

        assert_eq!(sl.len(), 1);
        assert_eq!(sl.entries[0].uri, "http://example.com/ubuntu");

        Ok(())
    }

    #[test]
    fn test_add_duplicate_entry() -> Result<()> {
        let mut sl = SourcesList::empty();
        
        sl.add(
            SourceType::Binary,
            "http://example.com/ubuntu".to_string(),
            "noble".to_string(),
            vec!["main".to_string()],
            false,
            None,
        )?;

        // Add same entry again
        sl.add(
            SourceType::Binary,
            "http://example.com/ubuntu".to_string(),
            "noble".to_string(),
            vec!["main".to_string()],
            false,
            None,
        )?;

        // Should still only have 1 entry
        assert_eq!(sl.len(), 1);

        Ok(())
    }

    #[test]
    fn test_add_component_to_existing() -> Result<()> {
        let mut sl = SourcesList::empty();
        
        sl.add(
            SourceType::Binary,
            "http://example.com/ubuntu".to_string(),
            "noble".to_string(),
            vec!["main".to_string()],
            false,
            None,
        )?;

        sl.add(
            SourceType::Binary,
            "http://example.com/ubuntu".to_string(),
            "noble".to_string(),
            vec!["universe".to_string()],
            false,
            None,
        )?;

        assert_eq!(sl.len(), 1);
        assert_eq!(sl.entries[0].components.len(), 2);
        assert!(sl.entries[0].components.contains(&"main".to_string()));
        assert!(sl.entries[0].components.contains(&"universe".to_string()));

        Ok(())
    }

    #[test]
    fn test_find_entries() -> Result<()> {
        let mut sl = SourcesList::empty();
        
        sl.add(
            SourceType::Binary,
            "http://example.com/ubuntu".to_string(),
            "noble".to_string(),
            vec!["main".to_string()],
            false,
            None,
        )?;

        sl.add(
            SourceType::Source,
            "http://example.com/ubuntu".to_string(),
            "noble".to_string(),
            vec!["main".to_string()],
            false,
            None,
        )?;

        let found = sl.find(Some(SourceType::Binary), None, None);
        assert_eq!(found.len(), 1);

        let found = sl.find(None, Some("http://example.com/ubuntu"), None);
        assert_eq!(found.len(), 2);

        Ok(())
    }

    #[test]
    fn test_save_and_reload() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let sources_file = temp_dir.path().join("sources.list");

        let mut sl = SourcesList::empty();
        sl.add(
            SourceType::Binary,
            "http://example.com/ubuntu".to_string(),
            "noble".to_string(),
            vec!["main".to_string(), "restricted".to_string()],
            false,
            Some(sources_file.clone()),
        )?;

        sl.save()?;

        // Reload and verify
        let mut sl2 = SourcesList::empty();
        sl2.load_file(&sources_file)?;

        assert_eq!(sl2.len(), 1);
        assert_eq!(sl2.entries[0].components.len(), 2);

        Ok(())
    }

    #[test]
    fn test_remove_entry() -> Result<()> {
        let mut sl = SourcesList::empty();
        
        sl.add(
            SourceType::Binary,
            "http://example.com/ubuntu".to_string(),
            "noble".to_string(),
            vec!["main".to_string()],
            false,
            None,
        )?;

        let entry = sl.entries[0].clone();
        let removed = sl.remove(&entry);

        assert!(removed);
        assert_eq!(sl.len(), 0);

        Ok(())
    }

    #[test]
    fn test_set_enabled() -> Result<()> {
        let mut sl = SourcesList::empty();
        
        sl.add(
            SourceType::Binary,
            "http://example.com/ubuntu".to_string(),
            "noble".to_string(),
            vec!["main".to_string()],
            false,
            None,
        )?;

        let count = sl.set_enabled(
            SourceType::Binary,
            "http://example.com/ubuntu",
            "noble",
            false,
        );

        assert_eq!(count, 1);
        assert!(sl.entries[0].disabled);

        Ok(())
    }
}
