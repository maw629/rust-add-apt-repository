// Debug logging utilities

use std::sync::atomic::{AtomicBool, Ordering};

static DEBUG_ENABLED: AtomicBool = AtomicBool::new(false);

/// Enable debug mode
pub fn enable_debug() {
    DEBUG_ENABLED.store(true, Ordering::SeqCst);
}

/// Check if debug mode is enabled
pub fn is_debug() -> bool {
    DEBUG_ENABLED.load(Ordering::SeqCst)
}

/// Print debug message if debug mode is enabled
#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        if $crate::debug::is_debug() {
            eprintln!("[DEBUG] {}", format!($($arg)*));
        }
    };
}

/// Print verbose information about a repository
pub fn log_repository_info(repo: &crate::repository::Repository) {
    if !is_debug() {
        return;
    }

    eprintln!("[DEBUG] Repository information:");
    eprintln!("[DEBUG]   File: {}", repo.file.display());
    eprintln!("[DEBUG]   Description: {:?}", repo.description);
    eprintln!("[DEBUG]   Enable source: {}", repo.enable_source);
    eprintln!("[DEBUG]   Use DEB822: {}", repo.use_deb822);
    eprintln!("[DEBUG]   Entries: {}", repo.entries.len());

    for (i, entry) in repo.entries.iter().enumerate() {
        eprintln!(
            "[DEBUG]     Entry {}: {} {} {} {}",
            i + 1,
            entry.entry_type.as_str(),
            entry.uri,
            entry.dist,
            entry.components.join(" ")
        );
    }

    if repo.key_data.is_some() {
        eprintln!("[DEBUG]   Has GPG key data: yes");
    }
    if let Some(key_url) = &repo.key_url {
        eprintln!("[DEBUG]   GPG key URL: {}", key_url);
    }
    if let Some(keyring_path) = &repo.keyring_path {
        eprintln!("[DEBUG]   Keyring path: {}", keyring_path.display());
    }
}

/// Print verbose information about parsed arguments
pub fn log_args(args: &crate::cli::Cli) {
    if !is_debug() {
        return;
    }

    eprintln!("[DEBUG] Command-line arguments:");
    eprintln!("[DEBUG]   Debug: {}", args.debug);
    eprintln!("[DEBUG]   Remove: {}", args.remove);
    eprintln!("[DEBUG]   Enable source: {}", args.enable_source);
    eprintln!("[DEBUG]   Components: {:?}", args.component);
    eprintln!("[DEBUG]   Dist: {:?}", args.dist);
    eprintln!("[DEBUG]   Pocket: {:?}", args.pocket);
    eprintln!("[DEBUG]   Yes: {}", args.yes);
    eprintln!("[DEBUG]   No update: {}", args.no_update);
    eprintln!("[DEBUG]   Login: {}", args.login);
    eprintln!("[DEBUG]   Dry run: {}", args.dry_run);
    eprintln!("[DEBUG]   List: {}", args.repo_spec.list);
}
