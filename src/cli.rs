use clap::{Parser, Args};

/// Rust implementation of add-apt-repository
#[derive(Parser, Debug)]
#[command(name = "rust-add-apt-repository")]
#[command(about = "Adds a repository into /etc/apt/sources.list or /etc/apt/sources.list.d or removes an existing one", long_about = None)]
#[command(version)]
pub struct Cli {
    /// Print debug information
    #[arg(short, long)]
    pub debug: bool,

    /// Disable repository
    #[arg(short, long)]
    pub remove: bool,

    /// Allow downloading of the source packages from the repository
    #[arg(short, long = "enable-source", action = clap::ArgAction::Count)]
    pub enable_source: u8,

    /// Components to use with the repository (can be used multiple times)
    #[arg(short, long)]
    pub component: Vec<String>,

    /// Add entry for this pocket
    #[arg(short, long)]
    pub pocket: Option<String>,

    /// Assume yes to all queries
    #[arg(short, long)]
    pub yes: bool,

    /// Do not update package cache after adding
    #[arg(short = 'n', long = "no-update")]
    pub no_update: bool,

    /// Login to Launchpad
    #[arg(short, long)]
    pub login: bool,

    /// Don't actually make any changes
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
    #[arg(short = 'L', long)]
    pub list: bool,

    /// PPA to add (format: ppa:user/ppa-name)
    #[arg(short = 'P', long)]
    pub ppa: Option<String>,

    /// Cloud Archive to add (format: cloud-archive:release)
    #[arg(short = 'C', long = "cloud")]
    pub cloud: Option<String>,

    /// Archive URI to add
    #[arg(short = 'U', long)]
    pub uri: Option<String>,

    /// Full sources.list entry line to add
    #[arg(short = 'S', long)]
    pub sourceslist: Option<Vec<String>>,

    /// sources.list line to add (deprecated, positional argument)
    #[arg(trailing_var_arg = true)]
    pub line: Vec<String>,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
