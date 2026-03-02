use std::path::PathBuf;

/// Represents a single APT source entry (one line in sources.list)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceEntry {
    /// Type: "deb" or "deb-src"
    pub entry_type: SourceType,
    /// Repository URI
    pub uri: String,
    /// Distribution (e.g., "noble", "noble-updates")
    pub dist: String,
    /// Components (e.g., ["main", "restricted", "universe"])
    pub components: Vec<String>,
    /// Whether this entry is disabled (commented out)
    pub disabled: bool,
    /// Source file where this entry is located
    pub file: PathBuf,
    /// Optional architectures filter
    pub architectures: Vec<String>,
    /// Original line text (for preservation)
    pub line: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SourceType {
    Binary,  // deb
    Source,  // deb-src
}

impl SourceType {
    pub fn as_str(&self) -> &str {
        match self {
            SourceType::Binary => "deb",
            SourceType::Source => "deb-src",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "deb" => Some(SourceType::Binary),
            "deb-src" => Some(SourceType::Source),
            _ => None,
        }
    }
}

impl SourceEntry {
    /// Create a new SourceEntry
    pub fn new(
        entry_type: SourceType,
        uri: String,
        dist: String,
        components: Vec<String>,
    ) -> Self {
        Self {
            entry_type,
            uri,
            dist,
            components,
            disabled: false,
            file: PathBuf::from(crate::config::SOURCES_LIST_PATH),
            architectures: Vec::new(),
            line: String::new(),
        }
    }

    /// Parse a sources.list line into a SourceEntry
    pub fn from_line(line: &str, file: PathBuf) -> Option<Self> {
        let trimmed = line.trim();
        
        // Check if disabled (commented)
        let (disabled, content) = if let Some(stripped) = trimmed.strip_prefix('#') {
            (true, stripped.trim())
        } else {
            (false, trimmed)
        };

        // Skip empty lines
        if content.is_empty() {
            return None;
        }

        let parts: Vec<&str> = content.split_whitespace().collect();
        
        // Need at least: type uri dist
        if parts.len() < 3 {
            return None;
        }

        let entry_type = SourceType::from_str(parts[0])?;
        let uri = parts[1].to_string();
        let dist = parts[2].to_string();
        let components: Vec<String> = parts[3..].iter().map(|s| s.to_string()).collect();

        Some(Self {
            entry_type,
            uri,
            dist,
            components,
            disabled,
            file,
            architectures: Vec::new(),
            line: line.to_string(),
        })
    }

    /// Convert SourceEntry to sources.list line format
    pub fn to_line(&self) -> String {
        let prefix = if self.disabled { "# " } else { "" };
        let comps = self.components.join(" ");
        format!(
            "{}{} {} {} {}",
            prefix,
            self.entry_type.as_str(),
            self.uri,
            self.dist,
            comps
        )
    }

    /// Set enabled/disabled state
    pub fn set_enabled(&mut self, enabled: bool) {
        self.disabled = !enabled;
    }

    /// Check if this entry matches another (ignoring disabled state and file)
    pub fn matches(&self, other: &Self) -> bool {
        self.entry_type == other.entry_type
            && self.uri == other.uri
            && self.dist == other.dist
            && self.components == other.components
    }
}

/// Represents DEB822 format source entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deb822SourceEntry {
    /// Types: vec of "deb" and/or "deb-src"
    pub types: Vec<SourceType>,
    /// Repository URIs (can be multiple)
    pub uris: Vec<String>,
    /// Suites/distributions
    pub suites: Vec<String>,
    /// Components
    pub components: Vec<String>,
    /// Whether disabled
    pub disabled: bool,
    /// Source file
    pub file: PathBuf,
    /// Optional signed-by key
    pub signed_by: Option<String>,
    /// Original content
    pub content: String,
}

impl Deb822SourceEntry {
    pub fn new(
        types: Vec<SourceType>,
        uris: Vec<String>,
        suites: Vec<String>,
        components: Vec<String>,
    ) -> Self {
        Self {
            types,
            uris,
            suites,
            components,
            disabled: false,
            file: PathBuf::new(),
            signed_by: None,
            content: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_source_entry_from_line() {
        let line = "deb http://archive.ubuntu.com/ubuntu noble main restricted";
        let entry = SourceEntry::from_line(line, PathBuf::from("/etc/apt/sources.list")).unwrap();
        
        assert_eq!(entry.entry_type, SourceType::Binary);
        assert_eq!(entry.uri, "http://archive.ubuntu.com/ubuntu");
        assert_eq!(entry.dist, "noble");
        assert_eq!(entry.components, vec!["main", "restricted"]);
        assert!(!entry.disabled);
    }

    #[test]
    fn test_source_entry_disabled() {
        let line = "# deb http://archive.ubuntu.com/ubuntu noble main";
        let entry = SourceEntry::from_line(line, PathBuf::from("/etc/apt/sources.list")).unwrap();
        
        assert!(entry.disabled);
        assert_eq!(entry.entry_type, SourceType::Binary);
    }

    #[test]
    fn test_source_entry_deb_src() {
        let line = "deb-src http://archive.ubuntu.com/ubuntu noble main universe";
        let entry = SourceEntry::from_line(line, PathBuf::from("/etc/apt/sources.list")).unwrap();
        
        assert_eq!(entry.entry_type, SourceType::Source);
        assert_eq!(entry.components, vec!["main", "universe"]);
    }

    #[test]
    fn test_source_entry_to_line() {
        let entry = SourceEntry::new(
            SourceType::Binary,
            "http://archive.ubuntu.com/ubuntu".to_string(),
            "noble".to_string(),
            vec!["main".to_string(), "restricted".to_string()],
        );
        
        let line = entry.to_line();
        assert_eq!(line, "deb http://archive.ubuntu.com/ubuntu noble main restricted");
    }

    #[test]
    fn test_source_entry_matches() {
        let entry1 = SourceEntry::new(
            SourceType::Binary,
            "http://example.com".to_string(),
            "noble".to_string(),
            vec!["main".to_string()],
        );
        
        let entry2 = SourceEntry::new(
            SourceType::Binary,
            "http://example.com".to_string(),
            "noble".to_string(),
            vec!["main".to_string()],
        );
        
        assert!(entry1.matches(&entry2));
    }
}
