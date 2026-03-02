pub mod cli;
pub mod config;
pub mod error;
pub mod sources;
pub mod sourceslist;
pub mod utils;

pub use error::{AppError, Result};

/// Main entry point for the library
pub fn run() -> Result<()> {
    // Placeholder for main application logic
    // Will be implemented in later phases
    Ok(())
}
