use serde::{Deserialize, Deserializer, Serialize};

/// The matching strategy for a formatting rule
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MatchMode {
    /// Case-insensitive matching with surrounding punctuation handling
    #[default]
    Smart,
    /// Literal string replacement, case-sensitive
    Exact,
    /// User-provided regex pattern with capture group support
    Regex,
}

/// A single formatting rule that defines a find/replace operation
#[derive(Debug, Clone, Serialize)]
pub struct FormattingRule {
    /// Unique identifier for the rule
    pub id: String,
    /// The text to search for (trigger text or regex pattern)
    pub trigger: String,
    /// The text to replace with (can be multi-line, supports $1/$2 in regex mode)
    pub replacement: String,
    /// Whether the rule is currently active
    pub enabled: bool,
    /// The matching strategy (smart, exact, or regex)
    pub match_mode: MatchMode,
}

/// Intermediate struct for backward-compatible deserialization
/// Handles both old format (exact_match: bool) and new format (match_mode: MatchMode)
#[derive(Deserialize)]
struct FormattingRuleRaw {
    id: String,
    trigger: String,
    replacement: String,
    enabled: bool,
    #[serde(default)]
    match_mode: Option<MatchMode>,
    #[serde(default)]
    exact_match: Option<bool>,
}

impl From<FormattingRuleRaw> for FormattingRule {
    fn from(raw: FormattingRuleRaw) -> Self {
        let match_mode = raw.match_mode.unwrap_or(match raw.exact_match {
            Some(true) => MatchMode::Exact,
            Some(false) | None => MatchMode::Smart,
        });
        FormattingRule {
            id: raw.id,
            trigger: raw.trigger,
            replacement: raw.replacement,
            enabled: raw.enabled,
            match_mode,
        }
    }
}

impl<'de> Deserialize<'de> for FormattingRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = FormattingRuleRaw::deserialize(deserializer)?;
        Ok(FormattingRule::from(raw))
    }
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
