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
    /// If true, matches exact text only. If false, also handles surrounding punctuation/spaces.
    pub exact_match: bool,
}

/// Built-in formatting options (toggles)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltInOptions {
    /// Add a space before ? and !
    pub space_before_punctuation: bool,
    /// Add a trailing space at the end of each transcription
    pub trailing_space: bool,
    /// Convert numbers written in letters to digits (e.g., "one" -> "1")
    pub convert_text_numbers: bool,
    /// Language for text-to-number conversion (e.g., "fr", "en")
    pub text_numbers_language: String,
    /// Threshold for text-to-number conversion (0.0 to 1.0)
    pub text_numbers_threshold: f64,
}

impl Default for BuiltInOptions {
    fn default() -> Self {
        Self {
            space_before_punctuation: false,
            trailing_space: false,
            convert_text_numbers: false,
            text_numbers_language: "en".to_string(),
            text_numbers_threshold: 0.0,
        }
    }
}

/// Complete formatting settings including built-in options and custom rules
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FormattingSettings {
    pub built_in: BuiltInOptions,
    pub rules: Vec<FormattingRule>,
}
