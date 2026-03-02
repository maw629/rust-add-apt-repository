// Repository validation and template matching

use crate::error::{AppError, Result};
use std::collections::HashSet;

/// Known Ubuntu/Debian components
pub const VALID_COMPONENTS: &[&str] = &[
    "main",
    "restricted",
    "universe",
    "multiverse",
    "contrib",
    "non-free",
    "non-free-firmware",
];

/// Validate component names against known list
pub fn validate_components(components: &[String]) -> Result<()> {
    let valid_set: HashSet<&str> = VALID_COMPONENTS.iter().copied().collect();
    let mut invalid = Vec::new();

    for component in components {
        if !valid_set.contains(component.as_str()) {
            invalid.push(component.clone());
        }
    }

    if !invalid.is_empty() {
        crate::debug_log!("Warning: Unknown components: {}", invalid.join(", "));
        eprintln!("Warning: Unknown component(s): {}", invalid.join(", "));
        eprintln!("Known components: {}", VALID_COMPONENTS.join(", "));
    }

    Ok(())
}

/// Check if a URI matches known Ubuntu repository patterns
pub fn is_ubuntu_official(uri: &str) -> bool {
    let ubuntu_patterns = [
        "archive.ubuntu.com",
        "security.ubuntu.com",
        "old-releases.ubuntu.com",
        "ports.ubuntu.com",
    ];

    ubuntu_patterns.iter().any(|pattern| uri.contains(pattern))
}

/// Check if a URI matches known Debian repository patterns
pub fn is_debian_official(uri: &str) -> bool {
    let debian_patterns = [
        "deb.debian.org",
        "security.debian.org",
        "ftp.debian.org",
    ];

    debian_patterns.iter().any(|pattern| uri.contains(pattern))
}

/// Check if a URI is a PPA
pub fn is_ppa(uri: &str) -> bool {
    uri.contains("ppa.launchpad.net") || uri.contains("ppa.launchpadcontent.net")
}

/// Check if a URI is a Cloud Archive
pub fn is_cloud_archive(uri: &str) -> bool {
    uri.contains("ubuntu-cloud.archive.canonical.com")
}

/// Get repository type description
pub fn get_repo_type(uri: &str) -> &'static str {
    if is_ubuntu_official(uri) {
        "Official Ubuntu repository"
    } else if is_debian_official(uri) {
        "Official Debian repository"
    } else if is_ppa(uri) {
        "Ubuntu PPA"
    } else if is_cloud_archive(uri) {
        "Ubuntu Cloud Archive"
    } else {
        "Third-party repository"
    }
}

/// Validate that a suite name looks reasonable
pub fn validate_suite(suite: &str) -> Result<()> {
    if suite.is_empty() {
        return Err(AppError::InvalidInput("Suite cannot be empty".to_string()));
    }

    // Check for common mistakes
    if suite.contains(' ') {
        return Err(AppError::InvalidInput(
            format!("Suite contains spaces: '{}'. Did you mean to use components?", suite)
        ));
    }

    if suite.starts_with('/') || suite.contains("..") {
        return Err(AppError::InvalidInput(
            format!("Suite looks like a path: '{}'", suite)
        ));
    }

    crate::debug_log!("Suite validation passed: {}", suite);
    Ok(())
}

/// Check if two entries are equivalent (ignoring order of components)
pub fn entries_match(
    type1: &crate::sources::SourceType,
    uri1: &str,
    dist1: &str,
    components1: &[String],
    type2: &crate::sources::SourceType,
    uri2: &str,
    dist2: &str,
    components2: &[String],
) -> bool {
    if type1 != type2 || uri1 != uri2 || dist1 != dist2 {
        return false;
    }

    // Components are treated as sets (order doesn't matter)
    let set1: HashSet<&String> = components1.iter().collect();
    let set2: HashSet<&String> = components2.iter().collect();

    set1 == set2
}

/// Find matching binary entry for a source entry
pub fn find_binary_for_source<'a>(
    source_entry: &crate::sources::SourceEntry,
    all_entries: &'a [crate::sources::SourceEntry],
) -> Option<&'a crate::sources::SourceEntry> {
    all_entries.iter().find(|entry| {
        entry.entry_type == crate::sources::SourceType::Binary
            && entry.uri == source_entry.uri
            && entry.dist == source_entry.dist
            && entry.components == source_entry.components
    })
}

/// Find matching source entry for a binary entry
pub fn find_source_for_binary<'a>(
    binary_entry: &crate::sources::SourceEntry,
    all_entries: &'a [crate::sources::SourceEntry],
) -> Option<&'a crate::sources::SourceEntry> {
    all_entries.iter().find(|entry| {
        entry.entry_type == crate::sources::SourceType::Source
            && entry.uri == binary_entry.uri
            && entry.dist == binary_entry.dist
            && entry.components == binary_entry.components
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_components() {
        let valid = vec!["main".to_string(), "universe".to_string()];
        assert!(validate_components(&valid).is_ok());

        let invalid = vec!["invalid".to_string()];
        // Should warn but not error
        assert!(validate_components(&invalid).is_ok());
    }

    #[test]
    fn test_is_ubuntu_official() {
        assert!(is_ubuntu_official("http://archive.ubuntu.com/ubuntu"));
        assert!(is_ubuntu_official("http://security.ubuntu.com/ubuntu"));
        assert!(!is_ubuntu_official("http://example.com/repo"));
    }

    #[test]
    fn test_is_ppa() {
        assert!(is_ppa("http://ppa.launchpad.net/user/ppa/ubuntu"));
        assert!(is_ppa("http://ppa.launchpadcontent.net/user/ppa/ubuntu"));
        assert!(!is_ppa("http://archive.ubuntu.com/ubuntu"));
    }

    #[test]
    fn test_validate_suite() {
        assert!(validate_suite("noble").is_ok());
        assert!(validate_suite("noble-updates").is_ok());
        assert!(validate_suite("").is_err());
        assert!(validate_suite("noble main").is_err());
    }

    #[test]
    fn test_entries_match() {
        use crate::sources::SourceType;

        let comp1 = vec!["main".to_string(), "universe".to_string()];
        let comp2 = vec!["universe".to_string(), "main".to_string()];

        assert!(entries_match(
            &SourceType::Binary,
            "http://example.com/repo",
            "noble",
            &comp1,
            &SourceType::Binary,
            "http://example.com/repo",
            "noble",
            &comp2,
        ));

        assert!(!entries_match(
            &SourceType::Binary,
            "http://example.com/repo",
            "noble",
            &comp1,
            &SourceType::Source,
            "http://example.com/repo",
            "noble",
            &comp2,
        ));
    }

    #[test]
    fn test_get_repo_type() {
        assert_eq!(get_repo_type("http://archive.ubuntu.com/ubuntu"), "Official Ubuntu repository");
        assert_eq!(get_repo_type("http://ppa.launchpad.net/user/ppa"), "Ubuntu PPA");
        assert_eq!(get_repo_type("http://example.com/repo"), "Third-party repository");
    }
}
