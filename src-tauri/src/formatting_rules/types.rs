use serde::{Deserialize, Serialize};

/// A single formatting rule that defines a find/replace operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattingRule {
    /// Unique identifier for the rule
    pub id: String,
    /// The text to search for (trigger text)
    pub trigger: String,
    /// The text to replace with (can be multi-line)
    pub replacement: String,
    /// Whether the rule is currently active
    pub enabled: bool,
}

/// Built-in formatting options (toggles)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltInOptions {
    /// Add a space before ? and !
    pub space_before_punctuation: bool,
    /// Add a trailing space at the end of each transcription
    pub trailing_space: bool,
}

impl Default for BuiltInOptions {
    fn default() -> Self {
        Self {
            space_before_punctuation: false,
            trailing_space: false,
        }
    }
}

/// Complete formatting settings including built-in options and custom rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattingSettings {
    pub built_in: BuiltInOptions,
    pub rules: Vec<FormattingRule>,
}

impl Default for FormattingSettings {
    fn default() -> Self {
        Self {
            built_in: BuiltInOptions::default(),
            rules: Vec::new(),
        }
    }
}
