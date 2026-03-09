/// APT authentication handling for private repositories
///
/// This module manages authentication credentials for private PPAs and repositories.
/// Credentials are stored in /etc/apt/auth.conf.d/*.conf files in netrc format.
use crate::config::AUTH_CONF_D_PATH;
use crate::error::{AppError, Result};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

/// Authentication credentials for a repository
#[derive(Debug, Clone, PartialEq)]
pub struct AuthEntry {
    /// Machine (hostname/path) for authentication
    pub machine: String,
    /// Login username
    pub login: String,
    /// Password or token
    pub password: String,
}

impl AuthEntry {
    /// Create a new authentication entry
    pub fn new(machine: String, login: String, password: String) -> Self {
        Self {
            machine,
            login,
            password,
        }
    }

    /// Format as netrc-style line
    /// Format: machine hostname/path login username password password
    pub fn to_netrc_line(&self) -> String {
        format!(
            "machine {} login {} password {}",
            self.machine, self.login, self.password
        )
    }

    /// Parse from netrc-style line
    pub fn from_netrc_line(line: &str) -> Result<Self> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        // Expected format: machine <host> login <user> password <pass>
        if parts.len() != 6 {
            return Err(AppError::InvalidInput(format!(
                "Invalid auth.conf line format: {}",
                line
            )));
        }

        if parts[0] != "machine" || parts[2] != "login" || parts[4] != "password" {
            return Err(AppError::InvalidInput(format!(
                "Invalid auth.conf keywords in line: {}",
                line
            )));
        }

        Ok(Self {
            machine: parts[1].to_string(),
            login: parts[3].to_string(),
            password: parts[5].to_string(),
        })
    }
}

/// Add authentication credentials to auth.conf.d
pub fn add_auth(owner: &str, ppa_name: &str, login: &str, password: &str) -> Result<PathBuf> {
    // Create auth.conf.d directory if it doesn't exist
    let auth_dir = Path::new(AUTH_CONF_D_PATH);
    if !auth_dir.exists() {
        fs::create_dir_all(auth_dir)?;
    }

    // Generate filename based on PPA
    let filename = format!("{}-ubuntu-{}.conf", owner, ppa_name);
    let auth_file = auth_dir.join(filename);

    // Construct machine hostname
    // Format: ppa.launchpadcontent.net/owner/ppa-name
    let machine = format!("ppa.launchpadcontent.net/{}/{}", owner, ppa_name);

    // Create auth entry
    let auth_entry = AuthEntry::new(machine, login.to_string(), password.to_string());

    // Write to file
    fs::write(&auth_file, auth_entry.to_netrc_line() + "\n")?;

    // Set strict permissions (0600 - owner read/write only)
    let mut perms = fs::metadata(&auth_file)?.permissions();
    perms.set_mode(0o600);
    fs::set_permissions(&auth_file, perms)?;

    println!(
        "Saved authentication credentials to {}",
        auth_file.display()
    );

    Ok(auth_file)
}

/// Remove authentication credentials
pub fn remove_auth(owner: &str, ppa_name: &str) -> Result<()> {
    let auth_dir = Path::new(AUTH_CONF_D_PATH);
    let filename = format!("{}-ubuntu-{}.conf", owner, ppa_name);
    let auth_file = auth_dir.join(filename);

    if auth_file.exists() {
        fs::remove_file(&auth_file)?;
        println!("Removed authentication file: {}", auth_file.display());
    }

    Ok(())
}

/// Check if authentication exists for a PPA
pub fn has_auth(owner: &str, ppa_name: &str) -> bool {
    let auth_dir = Path::new(AUTH_CONF_D_PATH);
    let filename = format!("{}-ubuntu-{}.conf", owner, ppa_name);
    let auth_file = auth_dir.join(filename);
    auth_file.exists()
}

/// Read authentication credentials for a PPA
pub fn read_auth(owner: &str, ppa_name: &str) -> Result<Option<AuthEntry>> {
    let auth_dir = Path::new(AUTH_CONF_D_PATH);
    let filename = format!("{}-ubuntu-{}.conf", owner, ppa_name);
    let auth_file = auth_dir.join(filename);

    if !auth_file.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&auth_file)?;
    let line = content
        .lines()
        .next()
        .ok_or_else(|| AppError::InvalidInput("Empty auth.conf file".to_string()))?;

    Ok(Some(AuthEntry::from_netrc_line(line)?))
}

/// Parse Launchpad subscription URL to extract credentials
/// Format: https://username:password@private-ppa.launchpadcontent.net/...
pub fn parse_subscription_url(url: &str) -> Result<(String, String)> {
    // Parse URL to extract username and password
    let url_str = url.trim();

    // Check for https://
    if !url_str.starts_with("https://") {
        return Err(AppError::InvalidInput(format!(
            "Invalid subscription URL format: {}",
            url
        )));
    }

    // Extract credentials part
    // Format: https://user:pass@host/path
    let after_scheme = &url_str[8..]; // Skip "https://"

    if let Some(at_pos) = after_scheme.find('@') {
        let credentials = &after_scheme[..at_pos];

        if let Some(colon_pos) = credentials.find(':') {
            let username = credentials[..colon_pos].to_string();
            let password = credentials[colon_pos + 1..].to_string();
            return Ok((username, password));
        }
    }

    Err(AppError::InvalidInput(format!(
        "No credentials found in subscription URL: {}",
        url
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_entry_to_netrc() {
        let entry = AuthEntry::new(
            "ppa.launchpadcontent.net/user/ppa".to_string(),
            "username".to_string(),
            // lgtm[rust/hard-coded-cryptographic-value]
            "token123".to_string(),
        );

        let netrc = entry.to_netrc_line();
        assert_eq!(
            netrc,
            "machine ppa.launchpadcontent.net/user/ppa login username password token123"
        );
    }

    #[test]
    fn test_auth_entry_from_netrc() {
        let line = "machine ppa.launchpadcontent.net/user/ppa login username password token123";
        let entry = AuthEntry::from_netrc_line(line).unwrap();

        assert_eq!(entry.machine, "ppa.launchpadcontent.net/user/ppa");
        assert_eq!(entry.login, "username");
        assert_eq!(entry.password, "token123");
    }

    #[test]
    fn test_auth_entry_roundtrip() {
        let original = AuthEntry::new(
            "example.com/path".to_string(),
            "user".to_string(),
            // lgtm[rust/hard-coded-cryptographic-value]
            "pass".to_string(),
        );

        let netrc = original.to_netrc_line();
        let parsed = AuthEntry::from_netrc_line(&netrc).unwrap();

        assert_eq!(original, parsed);
    }

    #[test]
    fn test_parse_subscription_url() {
        // lgtm[rust/hard-coded-cryptographic-value]
        let url = "https://user123:tokenABC@private-ppa.launchpadcontent.net/team/ppa/ubuntu";
        let (username, password) = parse_subscription_url(url).unwrap();

        assert_eq!(username, "user123");
        assert_eq!(password, "tokenABC");
    }

    #[test]
    fn test_parse_subscription_url_invalid() {
        let url = "http://example.com/path";
        assert!(parse_subscription_url(url).is_err());

        let url2 = "https://example.com/path"; // No credentials
        assert!(parse_subscription_url(url2).is_err());
    }

    #[test]
    fn test_invalid_netrc_line() {
        let line = "invalid format";
        assert!(AuthEntry::from_netrc_line(line).is_err());

        let line2 = "machine host";
        assert!(AuthEntry::from_netrc_line(line2).is_err());
    }
}
