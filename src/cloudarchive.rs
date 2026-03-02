/// Ubuntu Cloud Archive (UCA) support
/// 
/// This module handles Cloud Archive shortcut parsing and repository creation.
/// Cloud Archive provides newer OpenStack packages for Ubuntu LTS releases.

use crate::error::{AppError, Result};
use crate::repository::Repository;
use crate::sources::SourceType;
use crate::utils;
use std::collections::HashMap;
use std::path::PathBuf;

/// Ubuntu Cloud Archive base URI
pub const CLOUD_ARCHIVE_URI: &str = "http://ubuntu-cloud.archive.canonical.com/ubuntu";

/// Cloud Archive keyring package name
pub const CLOUD_KEYRING_PACKAGE: &str = "ubuntu-cloud-keyring";

/// Get OpenStack release to Ubuntu codename mappings
/// Maps OpenStack release names to their base Ubuntu versions
fn get_openstack_releases() -> HashMap<&'static str, &'static str> {
    let mut releases = HashMap::new();
    
    // Historical releases (LTS mapping)
    releases.insert("icehouse", "trusty");  // 14.04
    releases.insert("juno", "trusty");
    releases.insert("kilo", "trusty");
    releases.insert("liberty", "trusty");
    releases.insert("mitaka", "xenial");    // 16.04
    releases.insert("newton", "xenial");
    releases.insert("ocata", "xenial");
    releases.insert("pike", "xenial");
    releases.insert("queens", "bionic");    // 18.04
    releases.insert("rocky", "bionic");
    releases.insert("stein", "bionic");
    releases.insert("train", "bionic");
    releases.insert("ussuri", "focal");     // 20.04
    releases.insert("victoria", "focal");
    releases.insert("wallaby", "focal");
    releases.insert("xena", "focal");
    releases.insert("yoga", "jammy");       // 22.04
    releases.insert("zed", "jammy");
    releases.insert("antelope", "jammy");
    releases.insert("bobcat", "jammy");
    releases.insert("caracal", "noble");    // 24.04
    releases.insert("dalmatian", "noble");
    releases.insert("epoxy", "noble");
    
    releases
}

/// Parse cloud archive shortcut
/// Format: cloud-archive:release[-pocket] or uca:release[-pocket]
/// Where pocket is "updates" or "proposed"
pub fn parse_cloud_archive_shortcut(shortcut: &str) -> Result<(String, Option<String>)> {
    let shortcut = shortcut.trim();
    
    // Remove "cloud-archive:" or "uca:" prefix if present
    let spec = if let Some(stripped) = shortcut.strip_prefix("cloud-archive:") {
        stripped
    } else if let Some(stripped) = shortcut.strip_prefix("uca:") {
        stripped
    } else {
        // No prefix - assume it's just the release name (from -C flag)
        shortcut
    };
    
    // Split by hyphen to separate release and pocket
    let parts: Vec<&str> = spec.split('-').collect();
    
    let (release, pocket) = match parts.len() {
        1 => {
            // Just release name (defaults to updates)
            (parts[0].to_string(), Some("updates".to_string()))
        }
        2 => {
            // Release and pocket
            let release = parts[0].to_string();
            let pocket = parts[1].to_string();
            
            // Validate pocket
            if pocket != "updates" && pocket != "proposed" {
                return Err(AppError::InvalidInput(
                    format!("Invalid pocket: {}. Must be 'updates' or 'proposed'", pocket)
                ));
            }
            
            (release, Some(pocket))
        }
        _ => {
            return Err(AppError::InvalidInput(
                format!("Invalid cloud archive format: {}. Too many parts", spec)
            ));
        }
    };
    
    // Validate release name
    let releases = get_openstack_releases();
    if !releases.contains_key(release.as_str()) {
        return Err(AppError::InvalidInput(
            format!("Unknown OpenStack release: {}. Valid releases include: yoga, zed, antelope, bobcat, caracal", release)
        ));
    }
    
    Ok((release, pocket))
}

/// Construct Cloud Archive suite name
/// Format: {ubuntu-codename}-{release}[-pocket]
fn construct_suite(release: &str, pocket: Option<&str>) -> Result<String> {
    // Get current Ubuntu codename
    let codename = utils::get_distro_codename()?;
    
    let suite = if let Some(p) = pocket {
        if p == "updates" {
            // For updates pocket, suite is just codename-release
            format!("{}-{}", codename, release)
        } else {
            // For proposed pocket, include it
            format!("{}-{}-{}", codename, release, p)
        }
    } else {
        format!("{}-{}", codename, release)
    };
    
    Ok(suite)
}

/// Check if ubuntu-cloud-keyring package is installed
pub fn check_cloud_keyring_installed() -> bool {
    // Check using dpkg-query
    let output = std::process::Command::new("dpkg-query")
        .args(&["-W", "-f=${Status}", CLOUD_KEYRING_PACKAGE])
        .output();
    
    if let Ok(output) = output {
        let status = String::from_utf8_lossy(&output.stdout);
        status.contains("install ok installed")
    } else {
        false
    }
}

/// Install ubuntu-cloud-keyring package
pub fn install_cloud_keyring() -> Result<()> {
    println!("Installing {} package for GPG keys...", CLOUD_KEYRING_PACKAGE);
    
    let status = std::process::Command::new("apt-get")
        .args(&["install", "-y", CLOUD_KEYRING_PACKAGE])
        .status()?;
    
    if !status.success() {
        return Err(AppError::General(
            format!("Failed to install {}", CLOUD_KEYRING_PACKAGE)
        ));
    }
    
    println!("Successfully installed {}", CLOUD_KEYRING_PACKAGE);
    Ok(())
}

/// Create a Repository from Cloud Archive specification
pub fn create_cloud_archive_repository(
    shortcut: &str,
    enable_source: bool,
    components: &[String],
) -> Result<Repository> {
    // Parse cloud archive shortcut
    let (release, pocket) = parse_cloud_archive_shortcut(shortcut)?;
    
    println!("Adding Ubuntu Cloud Archive repository...");
    println!("  OpenStack Release: {}", release);
    if let Some(ref p) = pocket {
        println!("  Pocket: {}", p);
    }
    
    // Check if keyring is installed, prompt to install if not
    if !check_cloud_keyring_installed() {
        println!("\nThe {} package is required for Cloud Archive.", CLOUD_KEYRING_PACKAGE);
        print!("Install it now? [Y/n] ");
        
        use std::io::{self, Write};
        io::stdout().flush()?;
        
        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        
        let response = response.trim().to_lowercase();
        if response.is_empty() || response == "y" || response == "yes" {
            install_cloud_keyring()?;
        } else {
            return Err(AppError::General(
                format!("{} package is required. Aborting.", CLOUD_KEYRING_PACKAGE)
            ));
        }
    }
    
    // Construct suite name
    let suite = construct_suite(&release, pocket.as_deref())?;
    
    // Use provided components or default to "main"
    let comps = if components.is_empty() {
        vec!["main".to_string()]
    } else {
        components.to_vec()
    };
    
    // Create repository file
    let filename = format!("cloudarchive-{}", release);
    let file = PathBuf::from(format!(
        "{}/{}.list",
        crate::config::SOURCES_LIST_D_PATH,
        filename
    ));
    
    let mut repo = Repository::new(file);
    repo.description = Some(format!("Ubuntu Cloud Archive - OpenStack {}", release.to_uppercase()));
    repo.enable_source = enable_source;
    
    // Add binary entry
    let mut binary_entry = crate::sources::SourceEntry::new(
        SourceType::Binary,
        CLOUD_ARCHIVE_URI.to_string(),
        suite.clone(),
        comps.clone(),
    );
    binary_entry.file = repo.file.clone();
    repo.entries.push(binary_entry);
    
    // Add source entry if requested
    if enable_source {
        let mut source_entry = crate::sources::SourceEntry::new(
            SourceType::Source,
            CLOUD_ARCHIVE_URI.to_string(),
            suite,
            comps,
        );
        source_entry.file = repo.file.clone();
        repo.entries.push(source_entry);
    }
    
    // Note: GPG keys come from ubuntu-cloud-keyring package, not individual files
    // So we don't set repo.key_data
    
    Ok(repo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cloud_archive_basic() {
        let (release, pocket) = parse_cloud_archive_shortcut("cloud-archive:bobcat").unwrap();
        assert_eq!(release, "bobcat");
        assert_eq!(pocket, Some("updates".to_string()));
    }

    #[test]
    fn test_parse_cloud_archive_with_pocket() {
        let (release, pocket) = parse_cloud_archive_shortcut("cloud-archive:caracal-proposed").unwrap();
        assert_eq!(release, "caracal");
        assert_eq!(pocket, Some("proposed".to_string()));
    }

    #[test]
    fn test_parse_cloud_archive_uca_prefix() {
        let (release, pocket) = parse_cloud_archive_shortcut("uca:antelope").unwrap();
        assert_eq!(release, "antelope");
        assert_eq!(pocket, Some("updates".to_string()));
    }

    #[test]
    fn test_parse_cloud_archive_uca_with_pocket() {
        let (release, pocket) = parse_cloud_archive_shortcut("uca:yoga-updates").unwrap();
        assert_eq!(release, "yoga");
        assert_eq!(pocket, Some("updates".to_string()));
    }

    #[test]
    fn test_parse_cloud_archive_invalid_prefix() {
        let result = parse_cloud_archive_shortcut("invalid:bobcat");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_cloud_archive_invalid_pocket() {
        let result = parse_cloud_archive_shortcut("cloud-archive:bobcat-testing");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_cloud_archive_unknown_release() {
        let result = parse_cloud_archive_shortcut("cloud-archive:unknown-release");
        assert!(result.is_err());
    }

    #[test]
    fn test_construct_suite_basic() {
        // This test depends on the system's Ubuntu version
        let suite = construct_suite("bobcat", Some("updates"));
        assert!(suite.is_ok());
        // Suite should be like "noble-bobcat" for Ubuntu 24.04
    }

    #[test]
    fn test_construct_suite_proposed() {
        let suite = construct_suite("caracal", Some("proposed")).unwrap();
        // Should contain the release and pocket
        assert!(suite.contains("caracal"));
        assert!(suite.contains("proposed"));
    }

    #[test]
    fn test_openstack_releases_populated() {
        let releases = get_openstack_releases();
        assert!(releases.contains_key("yoga"));
        assert!(releases.contains_key("zed"));
        assert!(releases.contains_key("antelope"));
        assert!(releases.contains_key("bobcat"));
        assert!(releases.contains_key("caracal"));
    }
}
