use serde::{Deserialize, Deserializer};

pub(super) fn deserialize_short_text_correction<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum LegacyOrNew {
        Num(usize),
        Bool(bool),
    }

    match LegacyOrNew::deserialize(deserializer)? {
        LegacyOrNew::Num(n) => Ok(n),
        LegacyOrNew::Bool(true) => Ok(3),
        LegacyOrNew::Bool(false) => Ok(0),
    }
}

#[cfg(test)]
mod tests {
    use super::super::types::{BuiltInOptions, FormattingSettings};

    #[test]
    fn deserialize_built_in_with_usize_value() {
        let json = r#"{"short_text_correction": 3}"#;
        let opts: BuiltInOptions = serde_json::from_str(json).unwrap();
        assert_eq!(opts.short_text_correction, 3);
    }

    #[test]
    fn deserialize_built_in_with_usize_zero() {
        let json = r#"{"short_text_correction": 0}"#;
        let opts: BuiltInOptions = serde_json::from_str(json).unwrap();
        assert_eq!(opts.short_text_correction, 0);
    }

    #[test]
    fn deserialize_built_in_with_usize_five() {
        let json = r#"{"short_text_correction": 5}"#;
        let opts: BuiltInOptions = serde_json::from_str(json).unwrap();
        assert_eq!(opts.short_text_correction, 5);
    }

    #[test]
    fn deserialize_built_in_with_legacy_true() {
        let json = r#"{"short_text_correction": true}"#;
        let opts: BuiltInOptions = serde_json::from_str(json).unwrap();
        assert_eq!(opts.short_text_correction, 3);
    }

    #[test]
    fn deserialize_built_in_with_legacy_false() {
        let json = r#"{"short_text_correction": false}"#;
        let opts: BuiltInOptions = serde_json::from_str(json).unwrap();
        assert_eq!(opts.short_text_correction, 0);
    }

    #[test]
    fn deserialize_built_in_missing_field_uses_default() {
        let json = r#"{}"#;
        let opts: BuiltInOptions = serde_json::from_str(json).unwrap();
        assert_eq!(opts.short_text_correction, 3);
    }

    #[test]
    fn deserialize_full_settings_with_legacy_bool_keeps_rules() {
        let json = r#"{
            "built_in": {
                "short_text_correction": true,
                "space_before_punctuation": false,
                "trailing_space": true,
                "convert_text_numbers": false,
                "text_numbers_language": "fr",
                "text_numbers_threshold": 0.5
            },
            "rules": [
                {
                    "id": "r1",
                    "trigger": "foo",
                    "replacement": "bar",
                    "enabled": true,
                    "match_mode": "smart"
                },
                {
                    "id": "r2",
                    "trigger": "baz",
                    "replacement": "qux",
                    "enabled": false,
                    "match_mode": "exact"
                }
            ]
        }"#;
        let settings: FormattingSettings = serde_json::from_str(json).unwrap();
        assert_eq!(settings.built_in.short_text_correction, 3);
        assert_eq!(settings.rules.len(), 2);
        assert_eq!(settings.rules[0].id, "r1");
        assert_eq!(settings.rules[1].id, "r2");
    }
}
