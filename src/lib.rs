pub mod auth;
pub mod cli;
pub mod cloudarchive;
pub mod config;
pub mod deb822;
pub mod debug;
pub mod error;
pub mod global;
pub mod gpg;
pub mod ppa;
pub mod repository;
pub mod sources;
pub mod sourceslist;
pub mod utils;
pub mod validation;

pub use error::{AppError, Result};

use crate::cli::Cli;
use crate::repository::{parse_sourceslist_line, parse_uri_shortcut};
use crate::sourceslist::SourcesList;
use std::fs;
use std::path::PathBuf;

/// Main entry point for the library
pub fn run() -> Result<()> {
    let args = Cli::parse_args();

    // Enable debug mode if requested
    if args.debug {
        debug::enable_debug();
        debug_log!("Debug mode enabled");
        debug::log_args(&args);
    }

    // Handle list mode (doesn't require root)
    if args.repo_spec.list {
        debug_log!("Listing repositories");
        return global::list_repositories();
    }

    // For all other operations, we need root (already checked in main.rs)

    // Handle dry-run mode
    if args.dry_run {
        println!("Running in dry-run mode. No changes will be made.");
        debug_log!("Dry-run mode active");
    }

    // Check for global operations (no repository specified)
    let has_repo_spec = args.repo_spec.ppa.is_some()
        || args.repo_spec.cloud.is_some()
        || args.repo_spec.uri.is_some()
        || args.repo_spec.sourceslist.is_some()
        || !args.repo_spec.line.is_empty();

    if !has_repo_spec {
        // Global operations (no repository specified)
        debug_log!("Processing global operations");
        return handle_global_operations(&args);
    }

    // Determine what repository to add/remove
    let repo_spec = args.get_repo_spec()?;
    debug_log!("Repository specification: {}", repo_spec);

    if args.remove {
        debug_log!("Removing repository");
        remove_repository(&repo_spec, args.dry_run)
    } else {
        debug_log!("Adding repository");
        add_repository(&repo_spec, &args)
    }
}

/// Handle global operations (component, pocket, source management)
fn handle_global_operations(args: &Cli) -> Result<()> {
    let has_operation =
        !args.component.is_empty() || args.pocket.is_some() || args.enable_source > 0;

    if !has_operation {
        return Err(AppError::InvalidInput(
            "No repository or global operation specified. Use -c, -p, -s, or provide a repository."
                .to_string(),
        ));
    }

    // Handle component operations
    for component in &args.component {
        if args.remove {
            global::disable_component(component, args.dry_run)?;
        } else {
            global::enable_component(component, args.dry_run)?;
        }
    }

    // Handle pocket operations
    if let Some(pocket) = &args.pocket {
        if args.remove {
            global::disable_pocket(pocket, args.dry_run)?;
        } else {
            global::enable_pocket(pocket, args.dry_run)?;
        }
    }

    // Handle source code enable/disable
    if args.enable_source > 0 {
        if args.remove {
            global::disable_source_code(args.dry_run)?;
        } else {
            // -s once: enable existing, -s twice: also add missing
            let add_missing = args.enable_source >= 2;
            global::enable_source_code(add_missing, args.dry_run)?;
        }
    }

    // Update package cache unless --no-update
    if !args.no_update && !args.dry_run {
        println!("Updating package cache...");
        let status = std::process::Command::new("apt-get").arg("update").status();

        match status {
            Ok(s) if s.success() => println!("Package lists updated."),
            Ok(s) => println!("Warning: apt-get update exited with status {}", s),
            Err(e) => println!("Warning: Failed to run apt-get update: {}", e),
        }
    }

    Ok(())
}

fn add_repository(repo_spec: &str, args: &Cli) -> Result<()> {
    // Parse the repository specification
    let mut repo = if let Some(ppa_spec) = &args.repo_spec.ppa {
        // PPA shortcut
        ppa::create_ppa_repository(
            ppa_spec,
            args.enable_source > 0,
            &args.component,
            args.login,
        )?
    } else if let Some(cloud_spec) = &args.repo_spec.cloud {
        // Cloud Archive shortcut
        cloudarchive::create_cloud_archive_repository(
            cloud_spec,
            args.enable_source > 0,
            &args.component,
        )?
    } else if let Some(uri) = &args.repo_spec.uri {
        // URI shortcut
        let dist = args.dist.as_deref();
        parse_uri_shortcut(uri, dist, &args.component, args.enable_source > 0)?
    } else if let Some(lines) = &args.repo_spec.sourceslist {
        // Sources.list line format
        parse_sourceslist_line(&lines.join(" "))?
    } else if repo_spec.starts_with("ppa:") {
        // Positional PPA argument
        ppa::create_ppa_repository(
            repo_spec,
            args.enable_source > 0,
            &args.component,
            args.login,
        )?
    } else if repo_spec.starts_with("cloud-archive:") || repo_spec.starts_with("uca:") {
        // Positional Cloud Archive argument
        cloudarchive::create_cloud_archive_repository(
            repo_spec,
            args.enable_source > 0,
            &args.component,
        )?
    } else if repo_spec.starts_with("deb ") || repo_spec.starts_with("deb-src ") {
        // Positional argument with sources.list line
        parse_sourceslist_line(repo_spec)?
    } else if repo_spec.starts_with("http://")
        || repo_spec.starts_with("https://")
        || repo_spec.starts_with("ftp://")
        || repo_spec.starts_with("file://")
    {
        // Positional argument with URI
        let dist = args.dist.as_deref();
        parse_uri_shortcut(repo_spec, dist, &args.component, args.enable_source > 0)?
    } else {
        return Err(AppError::InvalidInput(format!(
            "Unrecognized repository format: {}",
            repo_spec
        )));
    };

    // Apply enable-source flag
    if args.enable_source > 0 && !repo.enable_source {
        repo.enable_source = true;
        // Add source entries if not already present
        let binary_entries: Vec<_> = repo
            .entries
            .iter()
            .filter(|e| e.entry_type == crate::sources::SourceType::Binary)
            .cloned()
            .collect();

        for entry in binary_entries {
            repo.add_source_entry(
                entry.uri.clone(),
                entry.dist.clone(),
                entry.components.clone(),
            );
        }
    };

    // Log repository information in debug mode
    debug::log_repository_info(&repo);

    // Validate components if any
    if !repo.entries.is_empty() {
        let components = &repo.entries[0].components;
        validation::validate_components(components)?;

        // Validate suite
        let suite = &repo.entries[0].dist;
        validation::validate_suite(suite)?;

        // Identify repository type
        let uri = &repo.entries[0].uri;
        let repo_type = validation::get_repo_type(uri);
        debug_log!("Repository type: {}", repo_type);
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
        // Save changes (backup is done automatically for existing files)
        sources_list.save()?;
        println!("\nRepository added successfully.");

        // Handle GPG key if provided
        if repo.key_data.is_some() || repo.key_url.is_some() {
            println!("\nImporting GPG key...");

            let keyring_filename = gpg::generate_keyring_filename(
                repo.file
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("repository"),
            );
            let keyring_path = gpg::get_keyring_path(&keyring_filename);

            let fingerprints = if let Some(key_data) = &repo.key_data {
                gpg::import_key(key_data, &keyring_path)?
            } else if let Some(key_url) = &repo.key_url {
                gpg::import_key_from_url(key_url, &keyring_path)?
            } else {
                Vec::new()
            };

            if !fingerprints.is_empty() {
                println!("Imported key fingerprints:");
                for fp in &fingerprints {
                    println!("  {}", fp.fingerprint);
                }
                println!("Keyring saved to: {}", keyring_path.display());
            }
        }

        // Run apt-get update unless --no-update specified
        if !args.no_update {
            println!("\nUpdating package lists...");
            let status = std::process::Command::new("apt-get").arg("update").status();

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

fn remove_repository(repo_spec: &str, dry_run: bool) -> Result<()> {
    // Parse the repository specification
    let repo = if repo_spec.starts_with("ppa:") {
        // PPA removal
        ppa::create_ppa_repository(repo_spec, false, &[], false)?
    } else if repo_spec.starts_with("cloud-archive:") || repo_spec.starts_with("uca:") {
        // Cloud Archive removal
        cloudarchive::create_cloud_archive_repository(repo_spec, false, &[])?
    } else if repo_spec.starts_with("deb ") || repo_spec.starts_with("deb-src ") {
        parse_sourceslist_line(repo_spec)?
    } else if repo_spec.starts_with("http://")
        || repo_spec.starts_with("https://")
        || repo_spec.starts_with("ftp://")
        || repo_spec.starts_with("file://")
    {
        return Err(AppError::General(
            "For URI removal, please provide the full sources.list line (deb ...)".to_string(),
        ));
    } else {
        return Err(AppError::General(
            "Repository removal for Cloud Archive not yet implemented".to_string(),
        ));
    };

    println!("Repository to remove:");
    for entry in &repo.entries {
        println!("  {}", entry.to_line());
    }

    if dry_run {
        println!("\n[DRY RUN] Would remove/disable repository (no changes made)");
        return Ok(());
    }

    // Load existing sources
    let mut sources_list = SourcesList::new()?;

    // Find matching entries
    let mut found_entries = Vec::new();
    for (idx, entry) in sources_list.entries.iter().enumerate() {
        for repo_entry in &repo.entries {
            if entry.entry_type == repo_entry.entry_type
                && entry.uri.trim_end_matches('/') == repo_entry.uri.trim_end_matches('/')
                && entry.dist == repo_entry.dist
            {
                found_entries.push((idx, entry.file.clone()));
            }
        }
    }

    if found_entries.is_empty() {
        println!("\nRepository not found in sources.");
        return Ok(());
    }

    // Disable (comment out) the entries
    let mut disabled_count = 0;
    for (idx, file) in &found_entries {
        sources_list.entries[*idx].set_enabled(false);
        sources_list.entries[*idx].line = sources_list.entries[*idx].to_line();
        // Mark file as modified
        sources_list.modified_files.insert(file.clone());
        disabled_count += 1;
    }

    // Check if any files now contain only disabled/invalid entries
    let files_to_check: std::collections::HashSet<PathBuf> =
        found_entries.iter().map(|(_, f)| f.clone()).collect();

    for file in files_to_check {
        let active_entries = sources_list
            .entries
            .iter()
            .filter(|e| e.file == file && !e.disabled)
            .count();

        if active_entries == 0 {
            // Backup before removing file entirely
            if file.exists() {
                let backup_path = file.with_extension(format!(
                    "{}.save",
                    file.extension().and_then(|s| s.to_str()).unwrap_or("")
                ));
                fs::copy(&file, backup_path)?;
                fs::remove_file(&file)?;
                println!("Removed empty sources file: {}", file.display());
            }
            // Remove entries from sources_list
            sources_list.entries.retain(|e| e.file != file);
        }
    }

    // Save changes
    sources_list.save()?;

    println!(
        "\nDisabled {} repository entr{}",
        disabled_count,
        if disabled_count == 1 { "y" } else { "ies" }
    );

    // Handle PPA-specific removal (auth and key files)
    if repo_spec.starts_with("ppa:") {
        // Extract owner and ppa_name from repo_spec
        if let Ok((owner, ppa_name)) = ppa::parse_ppa_shortcut(repo_spec) {
            // Remove authentication if exists
            if auth::has_auth(&owner, &ppa_name) {
                println!("\nRemoving authentication credentials...");
                ppa::remove_ppa_auth(&owner, &ppa_name)?;
            }
        }
    }

    // Check for associated keyring and prompt for removal
    let keyring_file = repo.file.with_extension("gpg");
    let keyring_path = PathBuf::from(crate::config::TRUSTED_GPG_D_PATH)
        .join(keyring_file.file_name().unwrap_or_default());

    if keyring_path.exists() {
        println!("\nFound associated keyring: {}", keyring_path.display());
        print!("Remove this keyring file? [y/N] ");
        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut response = String::new();
        io::stdin().read_line(&mut response)?;

        if response.trim().to_lowercase() == "y" {
            gpg::remove_keyring(&keyring_path)?;
            println!("Keyring removed.");
        }
    }

    Ok(())
}
