use std::fmt;
use std::io;

/// Main error type for the application
#[derive(Debug)]
pub enum AppError {
    /// IO error
    Io(io::Error),
    /// Permission error (not running as root)
    Permission(String),
    /// Invalid user input
    InvalidInput(String),
    /// General error with message
    General(String),
}

impl AppError {
    /// Get the exit code for this error
    pub fn exit_code(&self) -> i32 {
        match self {
            AppError::Io(_) => 1,
            AppError::Permission(_) => 1,
            AppError::InvalidInput(_) => 2,
            AppError::General(_) => 1,
        }
    }
}

pub type Result<T> = std::result::Result<T, AppError>;

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "IO error: {}", err),
            AppError::Permission(msg) => write!(f, "Permission error: {}", msg),
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AppError::General(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}
