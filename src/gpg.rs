/// GPG key management for APT repositories
use crate::error::{AppError, Result};
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Represents a GPG key fingerprint
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyFingerprint {
    pub fingerprint: String,
}

impl KeyFingerprint {
    /// Create a new key fingerprint
    pub fn new(fingerprint: String) -> Self {
        Self {
            fingerprint: fingerprint.to_uppercase().replace(' ', ""),
        }
    }

    /// Get the last 8 characters (short key ID)
    pub fn short_id(&self) -> String {
        let len = self.fingerprint.len();
        if len >= 8 {
            self.fingerprint[len - 8..].to_string()
        } else {
            self.fingerprint.clone()
        }
    }

    /// Get the last 16 characters (long key ID)
    pub fn long_id(&self) -> String {
        let len = self.fingerprint.len();
        if len >= 16 {
            self.fingerprint[len - 16..].to_string()
        } else {
            self.fingerprint.clone()
        }
    }
}

/// Import a GPG key from ASCII-armored text
pub fn import_key(key_data: &str, keyring_path: &Path) -> Result<Vec<KeyFingerprint>> {
    // Create temporary file for the key
    let temp_file = std::env::temp_dir().join(format!("apt-key-{}.asc", std::process::id()));

    let mut file = fs::File::create(&temp_file)?;
    file.write_all(key_data.as_bytes())?;
    drop(file);

    // Import the key using gpg --dearmor or gpg --import
    // First, let's dearmor it to binary format
    let output = Command::new("gpg")
        .arg("--dearmor")
        .arg("--yes")
        .arg("--output")
        .arg(keyring_path)
        .arg(&temp_file)
        .output()?;

    // Clean up temp file
    let _ = fs::remove_file(&temp_file);

    if !output.status.success() {
        return Err(AppError::General(format!(
            "Failed to import GPG key: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    // Set proper permissions (0644)
    let permissions = fs::Permissions::from_mode(0o644);
    fs::set_permissions(keyring_path, permissions)?;

    // Extract fingerprints from the imported key
    let fingerprints = extract_fingerprints_from_keyring(keyring_path)?;

    Ok(fingerprints)
}

/// Import a GPG key from a URL (handles both ASCII-armored and binary keys)
pub fn import_key_from_url(url: &str, keyring_path: &Path) -> Result<Vec<KeyFingerprint>> {
    // Download the key as raw bytes
    let key_bytes = download_key(url)?;

    // Try to detect if it's ASCII-armored or binary
    // ASCII-armored keys start with "-----BEGIN PGP PUBLIC KEY BLOCK-----"
    let is_ascii = key_bytes
        .get(0..5)
        .map(|prefix| prefix == b"-----")
        .unwrap_or(false);

    if is_ascii {
        // ASCII-armored key - use existing import_key() which expects string
        let key_data = String::from_utf8_lossy(&key_bytes).to_string();
        import_key(&key_data, keyring_path)
    } else {
        // Binary key - write directly to keyring without dearmoring
        fs::write(keyring_path, &key_bytes)?;

        // Set proper permissions (0644)
        let permissions = fs::Permissions::from_mode(0o644);
        fs::set_permissions(keyring_path, permissions)?;

        // Extract fingerprints from the imported key
        let fingerprints = extract_fingerprints_from_keyring(keyring_path)?;

        Ok(fingerprints)
    }
}

/// Download a GPG key from a URL (handles both ASCII and binary keys)
fn download_key(url: &str) -> Result<Vec<u8>> {
    let output = Command::new("curl").arg("-fsSL").arg(url).output()?;

    if !output.status.success() {
        return Err(AppError::General(format!(
            "Failed to download key from {}: {}",
            url,
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    Ok(output.stdout)
}

/// Extract fingerprints from a keyring file
pub fn extract_fingerprints_from_keyring(keyring_path: &Path) -> Result<Vec<KeyFingerprint>> {
    if !keyring_path.exists() {
        return Ok(Vec::new());
    }

    let output = Command::new("gpg")
        .arg("--no-default-keyring")
        .arg("--keyring")
        .arg(keyring_path)
        .arg("--list-keys")
        .arg("--with-colons")
        .arg("--with-fingerprint")
        .output()?;

    if !output.status.success() {
        return Err(AppError::General(format!(
            "Failed to list keys: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut fingerprints = Vec::new();

    // Parse gpg --with-colons output
    // Format: fpr:::::::::FINGERPRINT:
    for line in stdout.lines() {
        if line.starts_with("fpr:") {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 10 {
                let fingerprint = parts[9].to_string();
                if !fingerprint.is_empty() {
                    fingerprints.push(KeyFingerprint::new(fingerprint));
                }
            }
        }
    }

    Ok(fingerprints)
}

/// Extract fingerprints from ASCII-armored key data
pub fn extract_fingerprints_from_key_data(key_data: &str) -> Result<Vec<KeyFingerprint>> {
    // Create temporary file
    let temp_file = std::env::temp_dir().join(format!("apt-key-{}.asc", std::process::id()));

    let mut file = fs::File::create(&temp_file)?;
    file.write_all(key_data.as_bytes())?;
    drop(file);

    // Show key info without importing
    let output = Command::new("gpg")
        .arg("--with-colons")
        .arg("--with-fingerprint")
        .arg("--import-options")
        .arg("show-only")
        .arg("--import")
        .arg(&temp_file)
        .output()?;

    // Clean up temp file
    let _ = fs::remove_file(&temp_file);

    if !output.status.success() {
        return Err(AppError::General(format!(
            "Failed to parse key data: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut fingerprints = Vec::new();

    for line in stdout.lines() {
        if line.starts_with("fpr:") {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 10 {
                let fingerprint = parts[9].to_string();
                if !fingerprint.is_empty() {
                    fingerprints.push(KeyFingerprint::new(fingerprint));
                }
            }
        }
    }

    Ok(fingerprints)
}

/// Remove a keyring file
pub fn remove_keyring(keyring_path: &Path) -> Result<()> {
    if keyring_path.exists() {
        fs::remove_file(keyring_path)?;
    }
    Ok(())
}

/// Generate a keyring filename from a repository name or identifier
pub fn generate_keyring_filename(identifier: &str) -> String {
    let mut filename = identifier.to_string();

    // Replace special characters with dashes
    filename = filename
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect();

    // Limit length
    if filename.len() > 80 {
        filename.truncate(80);
    }

    format!("{}.gpg", filename)
}

/// Get the full path for a keyring file in trusted.gpg.d
pub fn get_keyring_path(filename: &str) -> PathBuf {
    PathBuf::from(crate::config::TRUSTED_GPG_D_PATH).join(filename)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_fingerprint_short_id() {
        let fp = KeyFingerprint::new("F6ECB3762474EDA9D21B7022871920D1991BC93C".to_string());
        assert_eq!(fp.short_id(), "991BC93C");
    }

    #[test]
    fn test_key_fingerprint_long_id() {
        let fp = KeyFingerprint::new("F6ECB3762474EDA9D21B7022871920D1991BC93C".to_string());
        assert_eq!(fp.long_id(), "871920D1991BC93C");
    }

    #[test]
    fn test_key_fingerprint_normalize() {
        let fp =
            KeyFingerprint::new("f6ec b376 2474 eda9 d21b 7022 8719 20d1 991b c93c".to_string());
        assert_eq!(fp.fingerprint, "F6ECB3762474EDA9D21B7022871920D1991BC93C");
    }

    #[test]
    fn test_generate_keyring_filename() {
        assert_eq!(
            generate_keyring_filename("example.com/ubuntu"),
            "example-com-ubuntu.gpg"
        );

        assert_eq!(
            generate_keyring_filename("ppa:user/ppa-name"),
            "ppa-user-ppa-name.gpg"
        );
    }

    #[test]
    fn test_get_keyring_path() {
        let path = get_keyring_path("test.gpg");
        assert!(path.to_string_lossy().contains("trusted.gpg.d"));
        assert!(path.to_string_lossy().ends_with("test.gpg"));
    }

    #[test]
    fn test_extract_fingerprints_from_keyring() {
        // Test with actual Ubuntu keyring if it exists
        let keyring = Path::new("/etc/apt/trusted.gpg.d/ubuntu-keyring-2018-archive.gpg");
        if keyring.exists() {
            let result = extract_fingerprints_from_keyring(keyring);
            assert!(result.is_ok());
            let fingerprints = result.unwrap();
            assert!(!fingerprints.is_empty());
            // Ubuntu 2018 key
            assert!(fingerprints
                .iter()
                .any(|fp| fp.fingerprint.ends_with("991BC93C")));
        }
    }
}
