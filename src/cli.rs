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
    #[arg(trailing_var_arg = true, hide = true)]
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
}
