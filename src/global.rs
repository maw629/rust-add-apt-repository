// Global operations (component, pocket, source management) across all repositories

use crate::sourceslist::SourcesList;
use crate::sources::{SourceEntry, SourceType};
use crate::config;
use crate::utils;
use crate::error::Result;
use std::collections::HashSet;
use std::path::PathBuf;

/// Enable a component in all repositories in sources.list
pub fn enable_component(component: &str, dry_run: bool) -> Result<()> {
    let main_file = std::path::Path::new(config::SOURCES_LIST_PATH);
    if !main_file.exists() {
        println!("Main sources.list not found");
        return Ok(());
    }
    
    let content = utils::read_file_to_string(main_file)?;
    let mut modified_lines = Vec::new();
    let mut modified = false;
    let mut affected_repos = Vec::new();
    
    for line in content.lines() {
        if let Some(entry) = SourceEntry::from_line(line, main_file.to_path_buf()) {
            if entry.disabled || entry.entry_type == SourceType::Source {
                modified_lines.push(line.to_string());
                continue;
            }
            
            if entry.components.contains(&component.to_string()) {
                modified_lines.push(line.to_string());
                continue;
            }
            
            let new_components = format!("{} {}", entry.components.join(" "), component);
            let new_line = format!("{} {} {} {}", 
                entry.entry_type.as_str(), entry.uri, entry.dist, new_components
            );
            modified_lines.push(new_line.clone());
            modified = true;
            affected_repos.push(format!("{} {}", entry.uri, entry.dist));
        } else {
            modified_lines.push(line.to_string());
        }
    }
    
    if !modified {
        println!("Component '{}' already present in all repositories or no applicable repositories found", component);
        return Ok(());
    }
    
    println!("Adding component '{}' to {} repositories:", component, affected_repos.len());
    for repo in &affected_repos {
        println!("  {}", repo);
    }
    
    if !dry_run {
        utils::write_string_to_file(main_file, &modified_lines.join("\n"), 0o644)?;
        println!("Component added successfully");
    } else {
        println!("(dry-run mode, no changes made)");
    }
    
    Ok(())
}

/// Disable a component from all repositories
pub fn disable_component(component: &str, dry_run: bool) -> Result<()> {
    let main_file = std::path::Path::new(config::SOURCES_LIST_PATH);
    if !main_file.exists() {
        println!("Main sources.list not found");
        return Ok(());
    }
    
    let content = utils::read_file_to_string(main_file)?;
    let mut modified_lines = Vec::new();
    let mut modified = false;
    let mut affected_repos = Vec::new();
    
    for line in content.lines() {
        if let Some(entry) = SourceEntry::from_line(line, main_file.to_path_buf()) {
            if entry.disabled {
                modified_lines.push(line.to_string());
                continue;
            }
            
            let new_components: Vec<String> = entry.components.iter()
                .filter(|c| *c != component)
                .cloned()
                .collect();
            
            if new_components.len() < entry.components.len() {
                if new_components.is_empty() {
                    modified_lines.push(line.to_string());
                } else {
                    let new_line = format!("{} {} {} {}", 
                        entry.entry_type.as_str(), entry.uri, entry.dist, new_components.join(" ")
                    );
                    modified_lines.push(new_line);
                    modified = true;
                    affected_repos.push(format!("{} {}", entry.uri, entry.dist));
                }
            } else {
                modified_lines.push(line.to_string());
            }
        } else {
            modified_lines.push(line.to_string());
        }
    }
    
    if !modified {
        println!("Component '{}' not found in any repositories", component);
        return Ok(());
    }
    
    println!("Removing component '{}' from {} repositories:", component, affected_repos.len());
    for repo in &affected_repos {
        println!("  {}", repo);
    }
    
    if !dry_run {
        utils::write_string_to_file(main_file, &modified_lines.join("\n"), 0o644)?;
        println!("Component removed successfully");
    } else {
        println!("(dry-run mode, no changes made)");
    }
    
    Ok(())
}

/// Enable a pocket (e.g., updates, security) for all repositories
pub fn enable_pocket(pocket: &str, dry_run: bool) -> Result<()> {
    let main_file = std::path::Path::new(config::SOURCES_LIST_PATH);
    if !main_file.exists() {
        println!("Main sources.list not found");
        return Ok(());
    }
    
    let content = utils::read_file_to_string(main_file)?;
    let mut base_entries = Vec::new();
    let mut existing_pockets = HashSet::new();
    
    for line in content.lines() {
        if let Some(entry) = SourceEntry::from_line(line, main_file.to_path_buf()) {
            if entry.disabled {
                continue;
            }
            
            if entry.dist.contains('-') {
                existing_pockets.insert((entry.uri.clone(), entry.dist.clone(), entry.entry_type));
            } else {
                base_entries.push(entry);
            }
        }
    }
    
    let mut new_lines = Vec::new();
    let mut added_count = 0;
    
    for entry in base_entries {
        let pocket_dist = format!("{}-{}", entry.dist, pocket);
        
        if existing_pockets.contains(&(entry.uri.clone(), pocket_dist.clone(), entry.entry_type)) {
            continue;
        }
        
        let new_line = format!("{} {} {} {}", 
            entry.entry_type.as_str(), entry.uri, pocket_dist, entry.components.join(" ")
        );
        new_lines.push(new_line);
        added_count += 1;
    }
    
    if added_count == 0 {
        println!("Pocket '{}' entries already exist for all applicable repositories", pocket);
        return Ok(());
    }
    
    println!("Adding '{}' pocket for {} repositories:", pocket, added_count);
    for line in &new_lines {
        println!("  {}", line);
    }
    
    if !dry_run {
        let mut all_content = content;
        if !all_content.ends_with('\n') {
            all_content.push('\n');
        }
        all_content.push_str(&new_lines.join("\n"));
        all_content.push('\n');
        
        utils::write_string_to_file(main_file, &all_content, 0o644)?;
        println!("Pocket entries added successfully");
    } else {
        println!("(dry-run mode, no changes made)");
    }
    
    Ok(())
}

/// Disable a pocket from all repositories
pub fn disable_pocket(pocket: &str, dry_run: bool) -> Result<()> {
    let main_file = std::path::Path::new(config::SOURCES_LIST_PATH);
    if !main_file.exists() {
        println!("Main sources.list not found");
        return Ok(());
    }
    
    let content = utils::read_file_to_string(main_file)?;
    let pocket_suffix = format!("-{}", pocket);
    let mut modified_lines = Vec::new();
    let mut removed_count = 0;
    let mut removed_repos = Vec::new();
    
    for line in content.lines() {
        if let Some(entry) = SourceEntry::from_line(line, main_file.to_path_buf()) {
            if entry.dist.ends_with(&pocket_suffix) {
                modified_lines.push(format!("# {}", line));
                removed_count += 1;
                removed_repos.push(format!("{} {}", entry.uri, entry.dist));
            } else {
                modified_lines.push(line.to_string());
            }
        } else {
            modified_lines.push(line.to_string());
        }
    }
    
    if removed_count == 0 {
        println!("No '{}' pocket entries found", pocket);
        return Ok(());
    }
    
    println!("Disabling '{}' pocket entries ({} repositories):", pocket, removed_count);
    for repo in &removed_repos {
        println!("  {}", repo);
    }
    
    if !dry_run {
        utils::write_string_to_file(main_file, &modified_lines.join("\n"), 0o644)?;
        println!("Pocket entries disabled successfully");
    } else {
        println!("(dry-run mode, no changes made)");
    }
    
    Ok(())
}

/// Enable source code repositories
pub fn enable_source_code(add_missing: bool, dry_run: bool) -> Result<()> {
    let mut sources = SourcesList::new()?;
    let mut enabled_count = 0;
    let mut added_count = 0;
    let mut files_to_update: HashSet<PathBuf> = HashSet::new();
    
    for entry in &mut sources.entries {
        if entry.entry_type == SourceType::Source && entry.disabled {
            entry.disabled = false;
            enabled_count += 1;
            files_to_update.insert(entry.file.clone());
        }
    }
    
    if enabled_count > 0 {
        println!("Enabled {} existing deb-src lines", enabled_count);
    }
    
    if add_missing {
        let mut deb_entries: Vec<(PathBuf, SourceEntry)> = Vec::new();
        let mut existing_src = HashSet::new();
        
        for entry in &sources.entries {
            if entry.entry_type == SourceType::Binary && !entry.disabled {
                deb_entries.push((entry.file.clone(), entry.clone()));
            } else if entry.entry_type == SourceType::Source {
                existing_src.insert((entry.uri.clone(), entry.dist.clone(), entry.components.clone()));
            }
        }
        
        let mut entries_to_add: Vec<(PathBuf, SourceEntry)> = Vec::new();
        for (file_path, deb_entry) in deb_entries {
            let key = (deb_entry.uri.clone(), deb_entry.dist.clone(), deb_entry.components.clone());
            
            if existing_src.contains(&key) {
                continue;
            }
            
            let mut src_entry = deb_entry.clone();
            src_entry.entry_type = SourceType::Source;
            src_entry.line = format!("{} {} {} {}",
                SourceType::Source.as_str(), src_entry.uri, src_entry.dist, src_entry.components.join(" ")
            );
            
            entries_to_add.push((file_path.clone(), src_entry));
            files_to_update.insert(file_path);
            added_count += 1;
        }
        
        for (_, entry) in entries_to_add {
            sources.entries.push(entry);
        }
        
        if added_count > 0 {
            println!("Added {} new deb-src lines", added_count);
        }
    }
    
    if enabled_count == 0 && added_count == 0 {
        println!("No changes needed - source code repositories already enabled");
        return Ok(());
    }
    
    if !dry_run {
        for file_path in files_to_update {
            let file_entries: Vec<_> = sources.entries.iter()
                .filter(|e| e.file == file_path)
                .collect();
            
            let mut lines = Vec::new();
            for entry in file_entries {
                let prefix = if entry.disabled { "# " } else { "" };
                lines.push(format!("{}{}", prefix, entry.line));
            }
            
            utils::write_string_to_file(&file_path, &lines.join("\n"), 0o644)?;
        }
        
        println!("Source code repositories enabled successfully");
    } else {
        println!("(dry-run mode, no changes made)");
    }
    
    Ok(())
}

/// Disable all source code repositories
pub fn disable_source_code(dry_run: bool) -> Result<()> {
    let mut sources = SourcesList::new()?;
    let mut disabled_count = 0;
    let mut files_to_update: HashSet<PathBuf> = HashSet::new();
    
    for entry in &mut sources.entries {
        if entry.entry_type == SourceType::Source && !entry.disabled {
            entry.disabled = true;
            disabled_count += 1;
            files_to_update.insert(entry.file.clone());
        }
    }
    
    if disabled_count == 0 {
        println!("No enabled deb-src lines found");
        return Ok(());
    }
    
    println!("Disabling {} deb-src lines", disabled_count);
    
    if !dry_run {
        for file_path in files_to_update {
            let file_entries: Vec<_> = sources.entries.iter()
                .filter(|e| e.file == file_path)
                .collect();
            
            let mut lines = Vec::new();
            for entry in file_entries {
                let prefix = if entry.disabled { "# " } else { "" };
                lines.push(format!("{}{}", prefix, entry.line));
            }
            
            utils::write_string_to_file(&file_path, &lines.join("\n"), 0o644)?;
        }
        
        println!("Source code repositories disabled successfully");
    } else {
        println!("(dry-run mode, no changes made)");
    }
    
    Ok(())
}

/// List all configured repositories
pub fn list_repositories() -> Result<()> {
    let sources = SourcesList::new()?;
    
    if sources.is_empty() {
        println!("No repositories configured.");
        return Ok(());
    }
    
    println!("Configured APT repositories:");
    println!();
    
    let mut current_file: Option<PathBuf> = None;
    
    for entry in &sources.entries {
        if entry.disabled {
            continue;
        }
        
        if current_file.as_ref() != Some(&entry.file) {
            if current_file.is_some() {
                println!();
            }
            println!("# {}", entry.file.display());
            current_file = Some(entry.file.clone());
        }
        
        println!("{} {} {} {}", 
            entry.entry_type.as_str(), entry.uri, entry.dist, entry.components.join(" ")
        );
    }
    println!();
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pocket_suffix_parsing() {
        let dist = "noble-updates";
        assert!(dist.ends_with("-updates"));
        assert!(dist.contains('-'));
        
        let pocket = "security";
        let new_dist = format!("noble-{}", pocket);
        assert_eq!(new_dist, "noble-security");
    }

    #[test]
    fn test_source_type_comparison() {
        assert_eq!(SourceType::Binary, SourceType::Binary);
        assert_eq!(SourceType::Source, SourceType::Source);
        assert_ne!(SourceType::Binary, SourceType::Source);
    }

    #[test]
    fn test_component_filtering() {
        let components = ["main".to_string(), "universe".to_string(), "restricted".to_string()];
        let filtered: Vec<String> = components.iter()
            .filter(|c| *c != "universe")
            .cloned()
            .collect();
        
        assert_eq!(filtered.len(), 2);
        assert!(filtered.contains(&"main".to_string()));
        assert!(!filtered.contains(&"universe".to_string()));
    }
}
