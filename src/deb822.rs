// DEB822 format parser and writer for APT sources

use crate::error::Result;
use crate::sources::SourceType;
use std::collections::HashMap;
use std::path::PathBuf;

/// Represents a DEB822 format source entry (stanza)
#[derive(Debug, Clone, PartialEq)]
pub struct Deb822Stanza {
    /// Types: deb, deb-src (can have multiple)
    pub types: Vec<SourceType>,
    /// URIs: Repository URLs (can have multiple)
    pub uris: Vec<String>,
    /// Suites: Distribution codenames (can have multiple)
    pub suites: Vec<String>,
    /// Components: Repository components (can have multiple)
    pub components: Vec<String>,
    /// Signed-By: Path to keyring file or inline key
    pub signed_by: Option<String>,
    /// Enabled: Whether this stanza is enabled (default: true)
    pub enabled: bool,
    /// Other fields that aren't parsed specifically
    pub other_fields: HashMap<String, String>,
    /// Source file where this stanza is located
    pub file: PathBuf,
}

impl Deb822Stanza {
    /// Create a new DEB822 stanza
    pub fn new(file: PathBuf) -> Self {
        Self {
            types: Vec::new(),
            uris: Vec::new(),
            suites: Vec::new(),
            components: Vec::new(),
            signed_by: None,
            enabled: true,
            other_fields: HashMap::new(),
            file,
        }
    }

    /// Check if this stanza represents a binary repository
    pub fn has_binary(&self) -> bool {
        self.types.contains(&SourceType::Binary)
    }

    /// Check if this stanza represents a source repository
    pub fn has_source(&self) -> bool {
        self.types.contains(&SourceType::Source)
    }

    /// Convert to string representation (DEB822 format)
    pub fn to_string(&self) -> String {
        let mut lines = Vec::new();

        // Enabled field (if disabled)
        if !self.enabled {
            lines.push("Enabled: no".to_string());
        }

        // Types field
        if !self.types.is_empty() {
            let types_str: Vec<String> = self.types.iter()
                .map(|t| t.as_str().to_string())
                .collect();
            lines.push(format!("Types: {}", types_str.join(" ")));
        }

        // URIs field
        if !self.uris.is_empty() {
            lines.push(format!("URIs: {}", self.uris.join(" ")));
        }

        // Suites field
        if !self.suites.is_empty() {
            lines.push(format!("Suites: {}", self.suites.join(" ")));
        }

        // Components field
        if !self.components.is_empty() {
            lines.push(format!("Components: {}", self.components.join(" ")));
        }

        // Signed-By field
        if let Some(signed_by) = &self.signed_by {
            if signed_by.contains("BEGIN PGP") {
                // Inline key - needs to be indented
                lines.push("Signed-By:".to_string());
                for line in signed_by.lines() {
                    lines.push(format!(" {}", line));
                }
            } else {
                // File path
                lines.push(format!("Signed-By: {}", signed_by));
            }
        }

        // Other fields
        for (key, value) in &self.other_fields {
            lines.push(format!("{}: {}", key, value));
        }

        lines.join("\n")
    }
}

/// Parse DEB822 format sources file
pub fn parse_deb822_file(content: &str, file_path: PathBuf) -> Result<Vec<Deb822Stanza>> {
    let mut stanzas = Vec::new();
    let mut current_stanza = Deb822Stanza::new(file_path.clone());
    let mut current_field = String::new();
    let mut current_value = String::new();
    let mut in_multiline = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip comments
        if trimmed.starts_with('#') {
            continue;
        }

        // Empty line separates stanzas
        if trimmed.is_empty() {
            if !current_field.is_empty() {
                // Save the last field
                process_field(&mut current_stanza, &current_field, &current_value)?;
                current_field.clear();
                current_value.clear();
            }

            if has_content(&current_stanza) {
                stanzas.push(current_stanza.clone());
                current_stanza = Deb822Stanza::new(file_path.clone());
            }
            in_multiline = false;
            continue;
        }

        // Continuation line (starts with space)
        if line.starts_with(' ') || line.starts_with('\t') {
            if in_multiline {
                if !current_value.is_empty() {
                    current_value.push('\n');
                }
                current_value.push_str(trimmed);
            }
            continue;
        }

        // New field line
        if let Some(colon_pos) = line.find(':') {
            // Save previous field if exists
            if !current_field.is_empty() {
                process_field(&mut current_stanza, &current_field, &current_value)?;
            }

            current_field = line[..colon_pos].trim().to_string();
            let value = line[colon_pos + 1..].trim();
            
            if value.is_empty() {
                // Multi-line value starts on next line
                current_value.clear();
                in_multiline = true;
            } else {
                current_value = value.to_string();
                in_multiline = false;
            }
        }
    }

    // Don't forget the last field and stanza
    if !current_field.is_empty() {
        process_field(&mut current_stanza, &current_field, &current_value)?;
    }
    if has_content(&current_stanza) {
        stanzas.push(current_stanza);
    }

    Ok(stanzas)
}

/// Process a field and add it to the stanza
fn process_field(stanza: &mut Deb822Stanza, field: &str, value: &str) -> Result<()> {
    match field.to_lowercase().as_str() {
        "types" => {
            for type_str in value.split_whitespace() {
                if let Some(source_type) = SourceType::from_str(type_str) {
                    stanza.types.push(source_type);
                }
            }
        }
        "uris" => {
            stanza.uris = value.split_whitespace()
                .map(|s| s.to_string())
                .collect();
        }
        "suites" => {
            stanza.suites = value.split_whitespace()
                .map(|s| s.to_string())
                .collect();
        }
        "components" => {
            stanza.components = value.split_whitespace()
                .map(|s| s.to_string())
                .collect();
        }
        "signed-by" => {
            stanza.signed_by = Some(value.to_string());
        }
        "enabled" => {
            stanza.enabled = !matches!(value.to_lowercase().as_str(), "no" | "false" | "0");
        }
        _ => {
            // Store other fields
            stanza.other_fields.insert(field.to_string(), value.to_string());
        }
    }
    Ok(())
}

/// Check if stanza has any content
fn has_content(stanza: &Deb822Stanza) -> bool {
    !stanza.types.is_empty() || !stanza.uris.is_empty()
}

/// Write DEB822 stanzas to a file
pub fn write_deb822_file(file_path: &std::path::Path, stanzas: &[Deb822Stanza]) -> Result<()> {
    let mut content = String::new();

    for (i, stanza) in stanzas.iter().enumerate() {
        if i > 0 {
            content.push('\n'); // Blank line between stanzas
        }
        content.push_str(&stanza.to_string());
        content.push('\n');
    }

    crate::utils::write_string_to_file(file_path, &content, 0o644)?;
    Ok(())
}

/// Convert a one-line format entry to DEB822 stanza
pub fn from_oneline(
    source_type: SourceType,
    uri: String,
    suite: String,
    components: Vec<String>,
    signed_by: Option<String>,
    file: PathBuf,
) -> Deb822Stanza {
    let mut stanza = Deb822Stanza::new(file);
    stanza.types.push(source_type);
    stanza.uris.push(uri);
    stanza.suites.push(suite);
    stanza.components = components;
    stanza.signed_by = signed_by;
    stanza
}

/// Expand a DEB822 stanza into multiple one-line entries
/// Each combination of (type, uri, suite) becomes a separate entry
pub fn expand_to_oneline(stanza: &Deb822Stanza) -> Vec<(SourceType, String, String, Vec<String>)> {
    let mut entries = Vec::new();

    for source_type in &stanza.types {
        for uri in &stanza.uris {
            for suite in &stanza.suites {
                entries.push((
                    *source_type,
                    uri.clone(),
                    suite.clone(),
                    stanza.components.clone(),
                ));
            }
        }
    }

    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_stanza() {
        let content = "\
Types: deb
URIs: http://archive.ubuntu.com/ubuntu/
Suites: noble
Components: main restricted
";
        let stanzas = parse_deb822_file(content, PathBuf::from("/test")).unwrap();
        assert_eq!(stanzas.len(), 1);
        
        let stanza = &stanzas[0];
        assert_eq!(stanza.types, vec![SourceType::Binary]);
        assert_eq!(stanza.uris, vec!["http://archive.ubuntu.com/ubuntu/"]);
        assert_eq!(stanza.suites, vec!["noble"]);
        assert_eq!(stanza.components, vec!["main", "restricted"]);
    }

    #[test]
    fn test_parse_multiple_types() {
        let content = "\
Types: deb deb-src
URIs: http://archive.ubuntu.com/ubuntu/
Suites: noble
Components: main
";
        let stanzas = parse_deb822_file(content, PathBuf::from("/test")).unwrap();
        assert_eq!(stanzas[0].types.len(), 2);
        assert!(stanzas[0].has_binary());
        assert!(stanzas[0].has_source());
    }

    #[test]
    fn test_parse_multiple_suites() {
        let content = "\
Types: deb
URIs: http://archive.ubuntu.com/ubuntu/
Suites: noble noble-updates noble-backports
Components: main universe
";
        let stanzas = parse_deb822_file(content, PathBuf::from("/test")).unwrap();
        assert_eq!(stanzas[0].suites.len(), 3);
        assert_eq!(stanzas[0].suites, vec!["noble", "noble-updates", "noble-backports"]);
    }

    #[test]
    fn test_parse_signed_by_file() {
        let content = "\
Types: deb
URIs: http://example.com/repo
Suites: stable
Components: main
Signed-By: /usr/share/keyrings/example.gpg
";
        let stanzas = parse_deb822_file(content, PathBuf::from("/test")).unwrap();
        assert_eq!(stanzas[0].signed_by, Some("/usr/share/keyrings/example.gpg".to_string()));
    }

    #[test]
    fn test_parse_disabled_stanza() {
        let content = "\
Enabled: no
Types: deb
URIs: http://example.com/repo
Suites: stable
Components: main
";
        let stanzas = parse_deb822_file(content, PathBuf::from("/test")).unwrap();
        assert!(!stanzas[0].enabled);
    }

    #[test]
    fn test_parse_multiple_stanzas() {
        let content = "\
Types: deb
URIs: http://archive.ubuntu.com/ubuntu/
Suites: noble
Components: main

Types: deb
URIs: http://security.ubuntu.com/ubuntu/
Suites: noble-security
Components: main
";
        let stanzas = parse_deb822_file(content, PathBuf::from("/test")).unwrap();
        assert_eq!(stanzas.len(), 2);
        assert_eq!(stanzas[0].uris[0], "http://archive.ubuntu.com/ubuntu/");
        assert_eq!(stanzas[1].uris[0], "http://security.ubuntu.com/ubuntu/");
    }

    #[test]
    fn test_stanza_to_string() {
        let mut stanza = Deb822Stanza::new(PathBuf::from("/test"));
        stanza.types.push(SourceType::Binary);
        stanza.uris.push("http://example.com/repo".to_string());
        stanza.suites.push("stable".to_string());
        stanza.components.push("main".to_string());

        let output = stanza.to_string();
        assert!(output.contains("Types: deb"));
        assert!(output.contains("URIs: http://example.com/repo"));
        assert!(output.contains("Suites: stable"));
        assert!(output.contains("Components: main"));
    }

    #[test]
    fn test_expand_to_oneline() {
        let mut stanza = Deb822Stanza::new(PathBuf::from("/test"));
        stanza.types.push(SourceType::Binary);
        stanza.types.push(SourceType::Source);
        stanza.uris.push("http://example.com/repo".to_string());
        stanza.suites.push("stable".to_string());
        stanza.suites.push("stable-updates".to_string());
        stanza.components.push("main".to_string());

        let entries = expand_to_oneline(&stanza);
        // 2 types × 1 uri × 2 suites = 4 entries
        assert_eq!(entries.len(), 4);
    }

    #[test]
    fn test_from_oneline() {
        let stanza = from_oneline(
            SourceType::Binary,
            "http://example.com/repo".to_string(),
            "stable".to_string(),
            vec!["main".to_string()],
            Some("/path/to/key.gpg".to_string()),
            PathBuf::from("/test"),
        );

        assert_eq!(stanza.types, vec![SourceType::Binary]);
        assert_eq!(stanza.uris, vec!["http://example.com/repo"]);
        assert_eq!(stanza.suites, vec!["stable"]);
        assert_eq!(stanza.signed_by, Some("/path/to/key.gpg".to_string()));
    }

    #[test]
    fn test_parse_with_comments() {
        let content = "\
# This is a comment
Types: deb
URIs: http://example.com/repo
# Another comment
Suites: stable
Components: main
";
        let stanzas = parse_deb822_file(content, PathBuf::from("/test")).unwrap();
        assert_eq!(stanzas.len(), 1);
        assert_eq!(stanzas[0].suites, vec!["stable"]);
    }
}
