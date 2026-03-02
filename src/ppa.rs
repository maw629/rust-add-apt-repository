/// PPA (Personal Package Archive) support for Launchpad
use crate::error::{AppError, Result};
use crate::repository::Repository;
use crate::sources::SourceType;
use crate::utils;
use serde::Deserialize;
use std::path::PathBuf;
use std::process::Command;

/// Launchpad PPA information
#[derive(Debug, Clone, Deserialize)]
pub struct PPAInfo {
    /// Display name of the PPA
    #[serde(rename = "displayname")]
    pub display_name: String,
    /// Description of the PPA
    pub description: Option<String>,
    /// Web link to the PPA on Launchpad
    pub web_link: String,
    /// Signing key fingerprint
    pub signing_key_fingerprint: Option<String>,
    /// Whether the PPA is private
    pub private: bool,
}

/// Parse a PPA shortcut
/// Format: ppa:user/ppa-name or ppa:user/distribution/ppa-name
pub fn parse_ppa_shortcut(ppa_spec: &str) -> Result<(String, String)> {
    let ppa_spec = ppa_spec.trim();
    
    // Remove "ppa:" prefix if present
    let spec = if let Some(stripped) = ppa_spec.strip_prefix("ppa:") {
        stripped
    } else {
        ppa_spec
    };

    // Split by /
    let parts: Vec<&str> = spec.split('/').collect();
    
    match parts.len() {
        2 => {
            // Format: user/ppa-name (most common)
            Ok((parts[0].to_string(), parts[1].to_string()))
        }
        3 => {
            // Format: user/distribution/ppa-name (explicit distribution)
            // We ignore the distribution part for now and use the last as ppa name
            Ok((parts[0].to_string(), parts[2].to_string()))
        }
        _ => Err(AppError::InvalidInput(
            format!("Invalid PPA format: {}. Expected ppa:user/ppa-name", ppa_spec)
        )),
    }
}

/// Fetch PPA information from Launchpad API
pub fn fetch_ppa_info(owner: &str, ppa_name: &str) -> Result<PPAInfo> {
    let api_url = format!(
        "https://api.launchpad.net/1.0/~{}/+archive/ubuntu/{}",
        owner, ppa_name
    );

    let output = Command::new("curl")
        .arg("-fsSL")
        .arg(&api_url)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::General(format!(
            "Failed to fetch PPA information from Launchpad: {}. PPA may not exist.",
            stderr
        )));
    }

    let json_data = String::from_utf8_lossy(&output.stdout);
    
    serde_json::from_str(&json_data).map_err(|e| {
        AppError::General(format!("Failed to parse Launchpad API response: {}", e))
    })
}

/// Get GPG key from Ubuntu keyserver
pub fn fetch_ppa_key(fingerprint: &str) -> Result<String> {
    let keyserver_url = format!(
        "https://keyserver.ubuntu.com/pks/lookup?op=get&search=0x{}",
        fingerprint
    );

    let output = Command::new("curl")
        .arg("-fsSL")
        .arg(&keyserver_url)
        .output()?;

    if !output.status.success() {
        return Err(AppError::General(format!(
            "Failed to fetch GPG key from keyserver: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Construct PPA repository URI
pub fn construct_ppa_uri(owner: &str, ppa_name: &str) -> String {
    format!("https://ppa.launchpadcontent.net/{}/{}/ubuntu", owner, ppa_name)
}

/// Create a Repository from PPA specification
pub fn create_ppa_repository(
    ppa_spec: &str,
    enable_source: bool,
    components: &[String],
) -> Result<Repository> {
    // Parse PPA shortcut
    let (owner, ppa_name) = parse_ppa_shortcut(ppa_spec)?;
    
    println!("Fetching PPA information from Launchpad...");
    
    // Fetch PPA info from Launchpad
    let ppa_info = fetch_ppa_info(&owner, &ppa_name)?;
    
    // Check if private
    if ppa_info.private {
        return Err(AppError::General(
            "This PPA is private. Use --login flag for authentication (not yet implemented).".to_string()
        ));
    }
    
    // Construct URI
    let uri = construct_ppa_uri(&owner, &ppa_name);
    
    // Get distribution
    let dist = utils::get_distro_codename()?;
    
    // Use provided components or default to "main"
    let comps = if components.is_empty() {
        vec!["main".to_string()]
    } else {
        components.to_vec()
    };
    
    // Create repository file
    let filename = format!("{}-ubuntu-{}-{}", owner, ppa_name, dist);
    let file = PathBuf::from(format!(
        "{}/{}.list",
        crate::config::SOURCES_LIST_D_PATH,
        filename
    ));
    
    let mut repo = Repository::new(file);
    repo.description = Some(ppa_info.display_name.clone());
    repo.enable_source = enable_source;
    
    // Add binary entry
    let mut binary_entry = crate::sources::SourceEntry::new(
        SourceType::Binary,
        uri.clone(),
        dist.clone(),
        comps.clone(),
    );
    binary_entry.file = repo.file.clone();
    repo.entries.push(binary_entry);
    
    // Add source entry if requested
    if enable_source {
        let mut source_entry = crate::sources::SourceEntry::new(
            SourceType::Source,
            uri,
            dist,
            comps,
        );
        source_entry.file = repo.file.clone();
        repo.entries.push(source_entry);
    }
    
    // Fetch and set GPG key
    if let Some(fingerprint) = ppa_info.signing_key_fingerprint {
        println!("Fetching GPG key {} from keyserver...", fingerprint);
        match fetch_ppa_key(&fingerprint) {
            Ok(key_data) => {
                repo.key_data = Some(key_data);
            }
            Err(e) => {
                eprintln!("Warning: Failed to fetch GPG key: {}", e);
                eprintln!("You may need to manually import the key.");
            }
        }
    }
    
    Ok(repo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ppa_shortcut_basic() {
        let (owner, ppa) = parse_ppa_shortcut("ppa:deadsnakes/ppa").unwrap();
        assert_eq!(owner, "deadsnakes");
        assert_eq!(ppa, "ppa");
    }

    #[test]
    fn test_parse_ppa_shortcut_without_prefix() {
        let (owner, ppa) = parse_ppa_shortcut("deadsnakes/ppa").unwrap();
        assert_eq!(owner, "deadsnakes");
        assert_eq!(ppa, "ppa");
    }

    #[test]
    fn test_parse_ppa_shortcut_with_distribution() {
        let (owner, ppa) = parse_ppa_shortcut("ppa:user/ubuntu/custom-ppa").unwrap();
        assert_eq!(owner, "user");
        assert_eq!(ppa, "custom-ppa");
    }

    #[test]
    fn test_parse_ppa_shortcut_invalid() {
        assert!(parse_ppa_shortcut("ppa:invalid").is_err());
        assert!(parse_ppa_shortcut("invalid/format/with/too/many/parts").is_err());
    }

    #[test]
    fn test_construct_ppa_uri() {
        let uri = construct_ppa_uri("deadsnakes", "ppa");
        assert_eq!(uri, "https://ppa.launchpadcontent.net/deadsnakes/ppa/ubuntu");
    }

    #[test]
    #[ignore] // Requires network access
    fn test_fetch_ppa_info_real() {
        let info = fetch_ppa_info("deadsnakes", "ppa").unwrap();
        assert_eq!(info.display_name, "New Python Versions");
        assert!(!info.private);
        assert!(info.signing_key_fingerprint.is_some());
    }

    #[test]
    #[ignore] // Requires network access
    fn test_fetch_ppa_key_real() {
        // deadsnakes PPA key
        let key = fetch_ppa_key("F23C5A6CF475977595C89F51BA6932366A755776").unwrap();
        assert!(key.contains("BEGIN PGP PUBLIC KEY BLOCK"));
        assert!(key.contains("END PGP PUBLIC KEY BLOCK"));
    }
}
