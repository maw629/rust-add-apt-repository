/// Repository shortcut handlers for adding repositories
use crate::error::{AppError, Result};
use crate::sources::{SourceEntry, SourceType};
use std::path::PathBuf;

/// Represents a repository to be added
#[derive(Debug, Clone)]
pub struct Repository {
    /// Source entries (deb and optionally deb-src)
    pub entries: Vec<SourceEntry>,
    /// Description of the repository
    pub description: Option<String>,
    /// File where this should be saved
    pub file: PathBuf,
    /// Whether to enable source (deb-src) entries
    pub enable_source: bool,
    /// GPG key data (ASCII-armored)
    pub key_data: Option<String>,
    /// GPG key URL
    pub key_url: Option<String>,
}

impl Repository {
    /// Create a new repository
    pub fn new(file: PathBuf) -> Self {
        Self {
            entries: Vec::new(),
            description: None,
            file,
            enable_source: false,
            key_data: None,
            key_url: None,
        }
    }

    /// Add a binary (deb) entry
    pub fn add_binary_entry(
        &mut self,
        uri: String,
        dist: String,
        components: Vec<String>,
    ) {
        let mut entry = SourceEntry::new(SourceType::Binary, uri, dist, components);
        entry.file = self.file.clone();
        self.entries.push(entry);
    }

    /// Add a source (deb-src) entry
    pub fn add_source_entry(
        &mut self,
        uri: String,
        dist: String,
        components: Vec<String>,
    ) {
        let mut entry = SourceEntry::new(SourceType::Source, uri, dist, components);
        entry.file = self.file.clone();
        self.entries.push(entry);
    }

    /// Add both binary and source entries
    pub fn add_both_entries(
        &mut self,
        uri: String,
        dist: String,
        components: Vec<String>,
    ) {
        self.add_binary_entry(uri.clone(), dist.clone(), components.clone());
        if self.enable_source {
            self.add_source_entry(uri, dist, components);
        }
    }
}

/// Parse a sources.list line format
/// Format: "deb http://example.com/ubuntu jammy main universe"
pub fn parse_sourceslist_line(line: &str) -> Result<Repository> {
    let trimmed = line.trim();
    
    // Remove comment prefix if present
    let content = if let Some(stripped) = trimmed.strip_prefix('#') {
        stripped.trim()
    } else {
        trimmed
    };

    let parts: Vec<&str> = content.split_whitespace().collect();
    
    if parts.len() < 3 {
        return Err(AppError::InvalidInput(
            "Invalid sources.list line format. Expected: deb <uri> <dist> [component...]".to_string()
        ));
    }

    let entry_type_str = parts[0];
    let entry_type = SourceType::from_str(entry_type_str)
        .ok_or_else(|| AppError::InvalidInput(
            format!("Invalid entry type: {}. Must be 'deb' or 'deb-src'", entry_type_str)
        ))?;

    let uri = parts[1].to_string();
    let dist = parts[2].to_string();
    
    // Components: everything after dist, or default to "main" if empty
    let components: Vec<String> = if parts.len() > 3 {
        parts[3..].iter().map(|s| s.to_string()).collect()
    } else {
        vec!["main".to_string()]
    };

    // Validate URI
    if !uri.starts_with("http://") 
        && !uri.starts_with("https://") 
        && !uri.starts_with("ftp://")
        && !uri.starts_with("file://") 
    {
        return Err(AppError::InvalidInput(
            format!("Invalid URI scheme: {}. Must be http://, https://, ftp://, or file://", uri)
        ));
    }

    // Create repository
    let filename = generate_filename_from_uri(&uri);
    let file = PathBuf::from(format!("{}/{}.list", crate::config::SOURCES_LIST_D_PATH, filename));
    
    let mut repo = Repository::new(file);
    
    let mut entry = SourceEntry::new(entry_type, uri, dist, components);
    entry.file = repo.file.clone();
    repo.entries.push(entry);
    
    Ok(repo)
}

/// Parse a URI shortcut
/// Format: "http://example.com/ubuntu"
pub fn parse_uri_shortcut(
    uri: &str,
    dist: Option<&str>,
    components: &[String],
    enable_source: bool,
) -> Result<Repository> {
    // Validate URI
    if !uri.starts_with("http://") 
        && !uri.starts_with("https://") 
        && !uri.starts_with("ftp://")
        && !uri.starts_with("file://") 
    {
        return Err(AppError::InvalidInput(
            format!("Invalid URI: {}. Must start with http://, https://, ftp://, or file://", uri)
        ));
    }

    // Get distribution (use detected if not specified)
    let distribution = if let Some(d) = dist {
        d.to_string()
    } else {
        crate::utils::get_distro_codename()?
    };

    // Use provided components or default to "main"
    let comps = if components.is_empty() {
        vec!["main".to_string()]
    } else {
        components.to_vec()
    };

    // Create repository
    let filename = generate_filename_from_uri(uri);
    let file = PathBuf::from(format!("{}/{}.list", crate::config::SOURCES_LIST_D_PATH, filename));
    
    let mut repo = Repository::new(file);
    repo.enable_source = enable_source;
    repo.add_both_entries(uri.to_string(), distribution, comps);
    
    Ok(repo)
}

/// Generate a filename from a URI
/// Examples:
/// - http://archive.ubuntu.com/ubuntu -> archive_ubuntu_com_ubuntu
/// - https://ppa.launchpadcontent.net/user/ppa/ubuntu -> ppa_launchpadcontent_net_user_ppa_ubuntu
fn generate_filename_from_uri(uri: &str) -> String {
    let mut filename = uri.to_string();
    
    // Remove scheme
    for scheme in &["https://", "http://", "ftp://", "file://"] {
        if let Some(stripped) = filename.strip_prefix(scheme) {
            filename = stripped.to_string();
            break;
        }
    }
    
    // Remove trailing slashes
    filename = filename.trim_end_matches('/').to_string();
    
    // Replace special characters with underscores
    filename = filename
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect();
    
    // Limit length to avoid filesystem issues
    if filename.len() > 100 {
        filename.truncate(100);
    }
    
    filename
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sourceslist_line_basic() {
        let line = "deb http://archive.ubuntu.com/ubuntu noble main";
        let repo = parse_sourceslist_line(line).unwrap();
        
        assert_eq!(repo.entries.len(), 1);
        assert_eq!(repo.entries[0].entry_type, SourceType::Binary);
        assert_eq!(repo.entries[0].uri, "http://archive.ubuntu.com/ubuntu");
        assert_eq!(repo.entries[0].dist, "noble");
        assert_eq!(repo.entries[0].components, vec!["main"]);
    }

    #[test]
    fn test_parse_sourceslist_line_multiple_components() {
        let line = "deb http://archive.ubuntu.com/ubuntu noble main restricted universe";
        let repo = parse_sourceslist_line(line).unwrap();
        
        assert_eq!(repo.entries[0].components, vec!["main", "restricted", "universe"]);
    }

    #[test]
    fn test_parse_sourceslist_line_no_components() {
        let line = "deb http://archive.ubuntu.com/ubuntu noble";
        let repo = parse_sourceslist_line(line).unwrap();
        
        // Should default to "main"
        assert_eq!(repo.entries[0].components, vec!["main"]);
    }

    #[test]
    fn test_parse_sourceslist_line_deb_src() {
        let line = "deb-src http://archive.ubuntu.com/ubuntu noble main";
        let repo = parse_sourceslist_line(line).unwrap();
        
        assert_eq!(repo.entries[0].entry_type, SourceType::Source);
    }

    #[test]
    fn test_parse_sourceslist_line_with_comment() {
        let line = "# deb http://archive.ubuntu.com/ubuntu noble main";
        let repo = parse_sourceslist_line(line).unwrap();
        
        assert_eq!(repo.entries[0].uri, "http://archive.ubuntu.com/ubuntu");
    }

    #[test]
    fn test_parse_sourceslist_line_invalid() {
        let line = "deb";
        assert!(parse_sourceslist_line(line).is_err());
    }

    #[test]
    fn test_parse_sourceslist_line_invalid_scheme() {
        let line = "deb gopher://example.com/ubuntu noble main";
        assert!(parse_sourceslist_line(line).is_err());
    }

    #[test]
    fn test_parse_uri_shortcut_basic() {
        let repo = parse_uri_shortcut(
            "http://example.com/ubuntu",
            Some("noble"),
            &[],
            false,
        ).unwrap();
        
        assert_eq!(repo.entries.len(), 1); // Only deb, not deb-src
        assert_eq!(repo.entries[0].uri, "http://example.com/ubuntu");
        assert_eq!(repo.entries[0].dist, "noble");
        assert_eq!(repo.entries[0].components, vec!["main"]);
    }

    #[test]
    fn test_parse_uri_shortcut_with_source() {
        let repo = parse_uri_shortcut(
            "http://example.com/ubuntu",
            Some("noble"),
            &[],
            true,
        ).unwrap();
        
        assert_eq!(repo.entries.len(), 2); // deb and deb-src
        assert_eq!(repo.entries[0].entry_type, SourceType::Binary);
        assert_eq!(repo.entries[1].entry_type, SourceType::Source);
    }

    #[test]
    fn test_parse_uri_shortcut_with_components() {
        let comps = vec!["main".to_string(), "universe".to_string()];
        let repo = parse_uri_shortcut(
            "http://example.com/ubuntu",
            Some("noble"),
            &comps,
            false,
        ).unwrap();
        
        assert_eq!(repo.entries[0].components, vec!["main", "universe"]);
    }

    #[test]
    fn test_generate_filename_from_uri() {
        assert_eq!(
            generate_filename_from_uri("http://archive.ubuntu.com/ubuntu"),
            "archive_ubuntu_com_ubuntu"
        );
        
        assert_eq!(
            generate_filename_from_uri("https://ppa.launchpadcontent.net/user/ppa/ubuntu/"),
            "ppa_launchpadcontent_net_user_ppa_ubuntu"
        );
    }

    #[test]
    fn test_repository_add_both_entries() {
        let file = PathBuf::from("/tmp/test.list");
        let mut repo = Repository::new(file);
        repo.enable_source = true;
        
        repo.add_both_entries(
            "http://example.com".to_string(),
            "noble".to_string(),
            vec!["main".to_string()],
        );
        
        assert_eq!(repo.entries.len(), 2);
        assert_eq!(repo.entries[0].entry_type, SourceType::Binary);
        assert_eq!(repo.entries[1].entry_type, SourceType::Source);
    }
}
