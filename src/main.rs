use rust_add_apt_repository::{cli::Cli, run};
use std::process;

fn main() {
    let args = Cli::parse_args();

    // Check if running as root (except for --list or --dry-run)
    if !args.dry_run && !args.repo_spec.list && !is_root() {
        eprintln!("Error: must run as root");
        process::exit(1);
    }

    // Run the application
    if let Err(e) = run() {
        eprintln!("{}", e);
        let exit_code = e.exit_code();
        process::exit(exit_code);
    }
}

fn is_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}
