/// Configuration constants for APT repository paths

pub const SOURCES_LIST_PATH: &str = "/etc/apt/sources.list";
pub const SOURCES_LIST_D_PATH: &str = "/etc/apt/sources.list.d";
pub const TRUSTED_GPG_D_PATH: &str = "/etc/apt/trusted.gpg.d";
pub const AUTH_CONF_D_PATH: &str = "/etc/apt/auth.conf.d";

/// Application version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Application name
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
