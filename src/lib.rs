pub mod cli;
pub mod config;
pub mod error;
pub mod repository;
pub mod sources;
pub mod sourceslist;
pub mod utils;

pub use error::{AppError, Result};

use crate::cli::Cli;
use crate::repository::{parse_sourceslist_line, parse_uri_shortcut};
use crate::sourceslist::SourcesList;

/// Main entry point for the library
pub fn run() -> Result<()> {
    let args = Cli::parse_args();
    
    // Handle list mode (doesn't require root)
    if args.repo_spec.list {
        return list_repositories();
    }
    
    // For all other operations, we need root (already checked in main.rs)
    
    // Handle dry-run mode
    if args.dry_run {
        println!("Running in dry-run mode. No changes will be made.");
    }
    
    // Determine what repository to add/remove
    let repo_spec = args.get_repo_spec()?;
    
    if args.remove {
        remove_repository(&repo_spec, args.dry_run)
    } else {
        add_repository(&repo_spec, &args)
    }
}

fn add_repository(repo_spec: &str, args: &Cli) -> Result<()> {
    // Parse the repository specification
    let mut repo = if let Some(uri) = &args.repo_spec.uri {
        // URI shortcut
        let dist = args.dist.as_deref();
        parse_uri_shortcut(uri, dist, &args.component, args.enable_source > 0)?
    } else if let Some(lines) = &args.repo_spec.sourceslist {
        // Sources.list line format
        parse_sourceslist_line(&lines.join(" "))?
    } else if repo_spec.starts_with("deb ") || repo_spec.starts_with("deb-src ") {
        // Positional argument with sources.list line
        parse_sourceslist_line(repo_spec)?
    } else if repo_spec.starts_with("http://") 
           || repo_spec.starts_with("https://") 
           || repo_spec.starts_with("ftp://") 
           || repo_spec.starts_with("file://") {
        // Positional argument with URI
        let dist = args.dist.as_deref();
        parse_uri_shortcut(repo_spec, dist, &args.component, args.enable_source > 0)?
    } else {
        return Err(AppError::InvalidInput(
            format!("Unrecognized repository format: {}", repo_spec)
        ));
    };

    // Apply enable-source flag
    if args.enable_source > 0 && !repo.enable_source {
        repo.enable_source = true;
        // Add source entries if not already present
        let binary_entries: Vec<_> = repo.entries
            .iter()
            .filter(|e| e.entry_type == crate::sources::SourceType::Binary)
            .cloned()
            .collect();
        
        for entry in binary_entries {
            repo.add_source_entry(entry.uri.clone(), entry.dist.clone(), entry.components.clone());
        }
    }

    // Display repository information
    println!("Repository to add:");
    for entry in &repo.entries {
        println!("  {}", entry.to_line());
    }
    if let Some(desc) = &repo.description {
        println!("  Description: {}", desc);
    }
    println!("  File: {}", repo.file.display());
    
    // Confirm with user unless -y is specified
    if !args.yes && !args.dry_run {
        print!("\nDo you want to add this repository? [y/N] ");
        use std::io::{self, Write};
        io::stdout().flush()?;
        
        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        
        let response = response.trim().to_lowercase();
        if response != "y" && response != "yes" {
            println!("Operation cancelled.");
            return Ok(());
        }
    }
    
    if args.dry_run {
        println!("\n[DRY RUN] Would add repository (no changes made)");
        return Ok(());
    }
    
    // Load existing sources
    let mut sources_list = SourcesList::new()?;
    
    // Backup before making changes
    let backup_ext = sources_list.backup()?;
    println!("\nBacked up sources (extension: {})", backup_ext);
    
    // Add each entry
    let mut added = false;
    for entry in &repo.entries {
        let idx = sources_list.add(
            entry.entry_type,
            entry.uri.clone(),
            entry.dist.clone(),
            entry.components.clone(),
            false,
            Some(repo.file.clone()),
        )?;
        
        let added_entry = &sources_list.entries[idx];
        println!("Added: {}", added_entry.to_line());
        added = true;
    }
    
    if added {
        // Save changes
        sources_list.save()?;
        println!("\nRepository added successfully.");
        
        // Run apt-get update unless --no-update specified
        if !args.no_update {
            println!("\nUpdating package lists...");
            let status = std::process::Command::new("apt-get")
                .arg("update")
                .status();
            
            match status {
                Ok(s) if s.success() => println!("Package lists updated."),
                Ok(s) => println!("Warning: apt-get update exited with status {}", s),
                Err(e) => println!("Warning: Failed to run apt-get update: {}", e),
            }
        }
    } else {
        println!("\nRepository already exists (no changes made).");
    }
    
    Ok(())
}

fn remove_repository(_repo_spec: &str, _dry_run: bool) -> Result<()> {
    // Repository removal will be implemented in later phase
    Err(AppError::General("Repository removal not yet implemented".to_string()))
}

fn list_repositories() -> Result<()> {
    let sources_list = SourcesList::new()?;
    
    if sources_list.is_empty() {
        println!("No repositories configured.");
        return Ok(());
    }
    
    println!("Configured repositories:\n");
    
    let mut current_file = None;
    for entry in &sources_list.entries {
        // Print file header when it changes
        if current_file.as_ref() != Some(&entry.file) {
            if current_file.is_some() {
                println!(); // Blank line between files
            }
            println!("{}:", entry.file.display());
            current_file = Some(entry.file.clone());
        }
        
        println!("  {}", entry.line);
    }
    
    Ok(())
}
