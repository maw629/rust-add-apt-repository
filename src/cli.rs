use clap::{Args, Parser};

/// Rust implementation of add-apt-repository
#[derive(Parser, Debug)]
#[command(name = "rust-add-apt-repository")]
#[command(version)]
#[command(about = "Add or remove apt repositories")]
#[command(
    long_about = "Adds or removes APT repositories from /etc/apt/sources.list or /etc/apt/sources.list.d\n\n\
    Supports PPAs (ppa:user/ppa-name), Cloud Archives (cloud-archive:release), \n\
    URIs, and sources.list format lines. Can also perform global operations on \n\
    existing repositories (components, pockets, source code)."
)]
#[command(after_help = "EXAMPLES:\n  \
    Add a PPA:\n    \
    sudo rust-add-apt-repository ppa:graphics-drivers/ppa\n\n  \
    Add Cloud Archive:\n    \
    sudo rust-add-apt-repository cloud-archive:bobcat\n\n  \
    Add repository by URI:\n    \
    sudo rust-add-apt-repository --uri http://example.com/repo --dist noble --component main\n\n  \
    Enable universe component globally:\n    \
    sudo rust-add-apt-repository --component universe\n\n  \
    Enable source repositories:\n    \
    sudo rust-add-apt-repository -s\n\n  \
    List repositories:\n    \
    rust-add-apt-repository --list\n\n\
    See 'man rust-add-apt-repository' or EXAMPLES.md for more information.")]
pub struct Cli {
    /// Print debug information to stderr
    #[arg(short, long, help = "Enable debug mode with verbose output")]
    pub debug: bool,

    /// Remove the specified repository instead of adding it
    #[arg(short, long, help = "Disable/remove repository")]
    pub remove: bool,

    /// Enable source code (deb-src) repositories
    ///
    /// Use once (-s) to enable existing commented deb-src lines.
    /// Use twice (-ss) to also add missing deb-src entries.
    /// Without a repository: operates globally on all repositories.
    #[arg(short = 's', long = "enable-source", action = clap::ArgAction::Count)]
    pub enable_source: u8,

    /// Repository component (e.g., main, universe, restricted, multiverse)
    ///
    /// Can be specified multiple times: --component main --component universe
    /// Without a repository: adds/removes component globally in /etc/apt/sources.list
    #[arg(short, long, value_name = "COMPONENT")]
    pub component: Vec<String>,

    /// Distribution codename (e.g., noble, jammy, bookworm)
    ///
    /// Defaults to current system distribution if not specified
    #[arg(long, value_name = "DIST")]
    pub dist: Option<String>,

    /// Add/remove pocket globally (e.g., updates, security, backports, proposed)
    ///
    /// Without a repository: operates globally on all existing repositories
    #[arg(short, long, value_name = "POCKET")]
    pub pocket: Option<String>,

    /// Assume yes to all queries (non-interactive mode)
    #[arg(short, long)]
    pub yes: bool,

    /// Do not run apt-get update after making changes
    #[arg(short = 'n', long = "no-update")]
    pub no_update: bool,

    /// Login to Launchpad for accessing private PPAs
    #[arg(short, long)]
    pub login: bool,

    /// Show what would be done without making any changes
    #[arg(long)]
    pub dry_run: bool,

    /// GPG key URL to download and import
    ///
    /// Download a GPG key from the specified URL and import it to the system keyring.
    /// Only works with --uri (not with PPA or Cloud Archive).
    /// Example: --key https://cli.github.com/packages/githubcli-archive-keyring.gpg
    #[arg(long, value_name = "URL", conflicts_with_all = ["keyring", "key_id"])]
    pub key: Option<String>,

    /// GPG keyring file to import
    ///
    /// Import a GPG key from a local file to the system keyring.
    /// Only works with --uri (not with PPA or Cloud Archive).
    /// Example: --keyring /usr/share/keyrings/example.gpg
    #[arg(long, value_name = "FILE", conflicts_with_all = ["key", "key_id"])]
    pub keyring: Option<String>,

    /// GPG key ID to fetch from keyserver
    ///
    /// Fetch a GPG key from Ubuntu keyserver by its key ID (short, long, or fingerprint).
    /// Only works with --uri (not with PPA or Cloud Archive).
    /// Example: --key-id 23F3D4EA75716059
    #[arg(long, value_name = "KEY_ID", conflicts_with_all = ["key", "keyring"])]
    pub key_id: Option<String>,

    /// Repository specification (mutually exclusive group)
    #[command(flatten)]
    pub repo_spec: RepoSpec,
}

#[derive(Args, Debug)]
#[group(multiple = false)]
pub struct RepoSpec {
    /// List currently configured repositories
    #[arg(
        short = 'L',
        long,
        help = "Show all repositories from sources.list and sources.list.d"
    )]
    pub list: bool,

    /// PPA to add (Personal Package Archive)
    ///
    /// Format: ppa:user/ppa-name or ppa:user/ppa-name/release
    /// Example: ppa:graphics-drivers/ppa
    #[arg(short = 'P', long, value_name = "PPA")]
    pub ppa: Option<String>,

    /// Cloud Archive to add (Ubuntu OpenStack releases)
    ///
    /// Format: cloud-archive:release or uca:release
    /// Example: cloud-archive:bobcat, uca:caracal
    #[arg(short = 'C', long = "cloud", value_name = "RELEASE")]
    pub cloud: Option<String>,

    /// Repository URI (web address)
    ///
    /// Must be used with --dist and --component
    /// Example: http://archive.ubuntu.com/ubuntu
    #[arg(short = 'U', long, value_name = "URI")]
    pub uri: Option<String>,

    /// Full sources.list entry line
    ///
    /// Format: deb [options] uri distribution [components...]
    /// Example: "deb http://archive.ubuntu.com/ubuntu noble main universe"
    #[arg(short = 'S', long, value_name = "LINE")]
    pub sourceslist: Option<Vec<String>>,

    /// sources.list line (positional, deprecated - use --sourceslist instead)
    #[arg(hide = true)]
    pub line: Vec<String>,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Get the repository specification from arguments
    pub fn get_repo_spec(&self) -> crate::error::Result<String> {
        if let Some(ppa) = &self.repo_spec.ppa {
            Ok(format!("ppa:{}", ppa))
        } else if let Some(cloud) = &self.repo_spec.cloud {
            Ok(format!("cloud-archive:{}", cloud))
        } else if let Some(uri) = &self.repo_spec.uri {
            Ok(uri.clone())
        } else if let Some(lines) = &self.repo_spec.sourceslist {
            Ok(lines.join(" "))
        } else if !self.repo_spec.line.is_empty() {
            Ok(self.repo_spec.line.join(" "))
        } else {
            Err(crate::error::AppError::InvalidInput(
                "No repository specified. Use --uri, --sourceslist, --ppa, --cloud, or provide a line.".to_string()
            ))
        }
    }

    /// Validate key-related arguments
    pub fn validate_key_flags(&self) -> crate::error::Result<()> {
        // Check if any key flag is specified
        let has_key_flag = self.key.is_some() || self.keyring.is_some() || self.key_id.is_some();

        if !has_key_flag {
            return Ok(()); // No key flags, nothing to validate
        }

        // Key flags should only be used with --uri
        if self.repo_spec.ppa.is_some() {
            return Err(crate::error::AppError::InvalidInput(
                "Key flags (--key, --keyring, --key-id) cannot be used with PPA. PPAs automatically fetch keys from Launchpad.".to_string()
            ));
        }

        if self.repo_spec.cloud.is_some() {
            return Err(crate::error::AppError::InvalidInput(
                "Key flags (--key, --keyring, --key-id) cannot be used with Cloud Archive. Cloud Archive uses ubuntu-cloud-keyring package.".to_string()
            ));
        }

        if self.repo_spec.uri.is_none() {
            return Err(crate::error::AppError::InvalidInput(
                "Key flags (--key, --keyring, --key-id) require --uri to specify the repository."
                    .to_string(),
            ));
        }

        // Validate keyring file exists if --keyring is used
        if let Some(keyring_path) = &self.keyring {
            let path = std::path::Path::new(keyring_path);
            if !path.exists() {
                return Err(crate::error::AppError::InvalidInput(format!(
                    "Keyring file not found: {}",
                    keyring_path
                )));
            }
            if !path.is_file() {
                return Err(crate::error::AppError::InvalidInput(format!(
                    "Keyring path is not a file: {}",
                    keyring_path
                )));
            }
        }

        // Validate key ID format if --key-id is used
        if let Some(key_id) = &self.key_id {
            let cleaned = key_id.trim().replace(' ', "").to_uppercase();

            // Key ID should be 8, 16, or 40 hex characters (short, long, or fingerprint)
            let valid_lengths = [8, 16, 40];
            if !valid_lengths.contains(&cleaned.len()) {
                return Err(crate::error::AppError::InvalidInput(
                    format!(
                        "Invalid key ID format: '{}'. Key ID should be 8, 16, or 40 hexadecimal characters.",
                        key_id
                    )
                ));
            }

            // Check if all characters are hex
            if !cleaned.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(crate::error::AppError::InvalidInput(
                    format!(
                        "Invalid key ID format: '{}'. Key ID must contain only hexadecimal characters (0-9, A-F).",
                        key_id
                    )
                ));
            }
        }

        Ok(())
    }
}
