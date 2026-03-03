use crate::error::{AppError, Result};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

/// Check if the current process is running as root
pub fn is_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}

/// Check if a file exists and is readable
pub fn file_exists_and_readable(path: &Path) -> bool {
    path.exists() && path.is_file()
}

/// Check if a directory exists and is readable
pub fn dir_exists_and_readable(path: &Path) -> bool {
    path.exists() && path.is_dir()
}

/// Create a directory with proper permissions (0o755)
pub fn create_dir_if_not_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
        let permissions = fs::Permissions::from_mode(0o755);
        fs::set_permissions(path, permissions)?;
    }
    Ok(())
}

/// Read file contents as string
pub fn read_file_to_string(path: &Path) -> Result<String> {
    fs::read_to_string(path).map_err(|e| AppError::Io(e))
}

/// Write string to file with specific permissions
pub fn write_string_to_file(path: &Path, content: &str, mode: u32) -> Result<()> {
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        create_dir_if_not_exists(parent)?;
    }

    fs::write(path, content)?;

    let permissions = fs::Permissions::from_mode(mode);
    fs::set_permissions(path, permissions)?;

    Ok(())
}

/// Safely backup a file before modification
pub fn backup_file(path: &Path) -> Result<PathBuf> {
    let backup_path = path.with_extension("backup");
    fs::copy(path, &backup_path)?;
    Ok(backup_path)
}

/// List all .list files in sources.list.d directory
pub fn list_sources_list_d_files() -> Result<Vec<PathBuf>> {
    let sources_d = Path::new(crate::config::SOURCES_LIST_D_PATH);

    if !sources_d.exists() {
        return Ok(Vec::new());
    }

    let mut files = Vec::new();
    for entry in fs::read_dir(sources_d)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "list" {
                    files.push(path);
                }
            }
        }
    }

    files.sort();
    Ok(files)
}

/// List all .sources files in sources.list.d directory (DEB822 format)
pub fn list_sources_list_d_deb822_files() -> Result<Vec<PathBuf>> {
    let sources_d = Path::new(crate::config::SOURCES_LIST_D_PATH);

    if !sources_d.exists() {
        return Ok(Vec::new());
    }

    let mut files = Vec::new();
    for entry in fs::read_dir(sources_d)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "sources" {
                    files.push(path);
                }
            }
        }
    }

    files.sort();
    Ok(files)
}

/// Get distribution information from lsb_release or os-release
pub fn get_distro_codename() -> Result<String> {
    // Try lsb_release first
    if let Ok(output) = std::process::Command::new("lsb_release")
        .args(["-cs"])
        .output()
    {
        if output.status.success() {
            let codename = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !codename.is_empty() {
                return Ok(codename);
            }
        }
    }

    // Fallback to os-release file
    let os_release = read_file_to_string(Path::new("/etc/os-release"))?;
    for line in os_release.lines() {
        if let Some(value) = line.strip_prefix("VERSION_CODENAME=") {
            return Ok(value.trim_matches('"').to_string());
        }
    }

    Err(AppError::General(
        "Could not determine distribution codename".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_is_root() {
        // This will vary depending on test environment
        let _ = is_root();
    }

    #[test]
    fn test_file_exists() {
        let path = PathBuf::from("/etc/apt/sources.list");
        // May not exist in all test environments
        let _ = file_exists_and_readable(&path);
    }

    #[test]
    fn test_get_distro_codename() {
        // Should work on Ubuntu/Debian systems
        if let Ok(codename) = get_distro_codename() {
            assert!(!codename.is_empty());
        }
    }
}
