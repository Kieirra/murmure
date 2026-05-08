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
#[derive(Debug, Clone, Default, Serialize)]
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
    /// Optional display name shown in the collapsed view in place of the trigger pattern.
    /// `None` means "fallback on trigger". Empty strings are normalized to `None` by the frontend.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
    #[serde(default)]
    name: Option<String>,
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
            name: raw.name,
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
#[serde(default)]
pub struct BuiltInOptions {
    /// Maximum word count for short text correction (0 = disabled, 1-5 = threshold)
    pub short_text_correction: usize,
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
            short_text_correction: 3,
            space_before_punctuation: false,
            trailing_space: true,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_legacy_rule_without_name() {
        let json = r#"{
            "id": "abc",
            "trigger": "foo",
            "replacement": "bar",
            "enabled": true,
            "match_mode": "smart"
        }"#;
        let rule: FormattingRule = serde_json::from_str(json).unwrap();
        assert_eq!(rule.name, None);
    }

    #[test]
    fn deserialize_rule_with_name() {
        let json = r#"{
            "id": "abc",
            "trigger": "foo",
            "replacement": "bar",
            "enabled": true,
            "match_mode": "smart",
            "name": "Mon raccourci"
        }"#;
        let rule: FormattingRule = serde_json::from_str(json).unwrap();
        assert_eq!(rule.name, Some("Mon raccourci".to_string()));
    }

    #[test]
    fn serialize_rule_without_name_omits_field() {
        let rule = FormattingRule {
            id: "abc".to_string(),
            trigger: "foo".to_string(),
            replacement: "bar".to_string(),
            enabled: true,
            match_mode: MatchMode::Smart,
            name: None,
        };
        let json = serde_json::to_string(&rule).unwrap();
        assert!(!json.contains("\"name\""));
    }
}
