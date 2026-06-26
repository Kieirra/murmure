use super::formatter::{apply_custom_rule, apply_formatting};
use super::types::FormattingSettings;
use serde::Serialize;
use std::collections::HashSet;

pub struct FormattedWithHighlights {
    pub text: String,
    pub highlights: Vec<HighlightRange>,
}

#[derive(Serialize, Clone)]
pub struct HighlightRange {
    pub start: usize,
    pub end: usize,
}

pub fn apply_formatting_with_highlights_and_original(
    raw_text: String,
    _original_text: String,
    settings: &FormattingSettings,
    dictionary: &[String],
) -> FormattedWithHighlights {
    let dict_set: HashSet<String> = dictionary.iter().map(|w| w.to_lowercase()).collect();
    let rule_changed = rule_changed_words(&raw_text, settings);
    let formatted = apply_formatting(raw_text, settings);
    build_highlights(&formatted, &dict_set, &rule_changed)
}

// A word is highlighted if it belongs to the dictionary OR it is part of the
// replacement of a formatting rule that fired. Dictionary covers boosting +
// post-correction; rule replacements cover the full multi-word output.
fn rule_changed_words(raw_text: &str, settings: &FormattingSettings) -> HashSet<String> {
    let mut current = raw_text.to_string();
    let mut changed = HashSet::new();
    for rule in &settings.rules {
        if rule.enabled && !rule.trigger.is_empty() {
            let before = current.clone();
            current =
                apply_custom_rule(&current, &rule.trigger, &rule.replacement, &rule.match_mode);
            if current != before {
                for w in rule.replacement.split_whitespace() {
                    changed.insert(w.to_lowercase());
                }
            }
        }
    }
    changed
}

fn normalize_for_dict(word: &str) -> String {
    word.trim_end_matches(|c: char| {
        matches!(
            c,
            '.' | ',' | '!' | '?' | ';' | ':' | '\'' | '"' | '\u{2019}' | '\u{201D}'
        )
    })
    .to_lowercase()
}

fn build_highlights(
    formatted: &str,
    dict_set: &HashSet<String>,
    rule_changed: &HashSet<String>,
) -> FormattedWithHighlights {
    let mut highlights = Vec::new();
    let mut byte_offset: usize = 0;

    for fw in formatted.split_whitespace() {
        let word_start = match formatted[byte_offset..].find(fw) {
            Some(pos) => byte_offset + pos,
            None => byte_offset,
        };
        let word_end = word_start + fw.len();

        let is_dict_member = dict_set.contains(&normalize_for_dict(fw));
        let is_rule_changed = rule_changed.contains(&fw.to_lowercase());

        if is_dict_member || is_rule_changed {
            let char_start = formatted[..word_start].chars().count();
            let char_end = char_start + fw.chars().count();
            highlights.push(HighlightRange {
                start: char_start,
                end: char_end,
            });
        }

        byte_offset = word_end;
    }

    FormattedWithHighlights {
        text: formatted.to_string(),
        highlights,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formatting_rules::types::{BuiltInOptions, FormattingRule, MatchMode};

    fn apply_formatting_with_highlights(
        raw_text: String,
        settings: &FormattingSettings,
    ) -> FormattedWithHighlights {
        let original = raw_text.clone();
        apply_formatting_with_highlights_and_original(raw_text, original, settings, &[])
    }

    fn settings_with_rule(trigger: &str, replacement: &str) -> FormattingSettings {
        FormattingSettings {
            built_in: BuiltInOptions {
                short_text_correction: 0,
                trailing_space: false,
                ..Default::default()
            },
            rules: vec![FormattingRule {
                id: "test".to_string(),
                trigger: trigger.to_string(),
                replacement: replacement.to_string(),
                enabled: true,
                match_mode: MatchMode::Exact,
                ..Default::default()
            }],
        }
    }

    #[test]
    fn no_rules_no_highlights() {
        let settings = FormattingSettings {
            built_in: BuiltInOptions {
                short_text_correction: 0,
                trailing_space: false,
                ..Default::default()
            },
            rules: vec![],
        };
        let result = apply_formatting_with_highlights("hello world".to_string(), &settings);
        assert_eq!(result.text, "hello world");
        assert!(result.highlights.is_empty());
    }

    #[test]
    fn single_word_replacement() {
        let settings = settings_with_rule("hello", "bonjour");
        let result = apply_formatting_with_highlights("hello world".to_string(), &settings);
        assert_eq!(result.text, "bonjour world");
        assert_eq!(result.highlights.len(), 1);
        assert_eq!(result.highlights[0].start, 0);
        assert_eq!(result.highlights[0].end, 7);
    }

    #[test]
    fn no_change_no_highlight() {
        let settings = settings_with_rule("missing", "replaced");
        let result = apply_formatting_with_highlights("hello world".to_string(), &settings);
        assert_eq!(result.text, "hello world");
        assert!(result.highlights.is_empty());
    }

    #[test]
    fn multiple_replacements() {
        let settings = FormattingSettings {
            built_in: BuiltInOptions {
                short_text_correction: 0,
                trailing_space: false,
                ..Default::default()
            },
            rules: vec![
                FormattingRule {
                    id: "1".to_string(),
                    trigger: "hello".to_string(),
                    replacement: "bonjour".to_string(),
                    enabled: true,
                    match_mode: MatchMode::Exact,
                    ..Default::default()
                },
                FormattingRule {
                    id: "2".to_string(),
                    trigger: "world".to_string(),
                    replacement: "monde".to_string(),
                    enabled: true,
                    match_mode: MatchMode::Exact,
                    ..Default::default()
                },
            ],
        };
        let result = apply_formatting_with_highlights("hello world".to_string(), &settings);
        assert_eq!(result.text, "bonjour monde");
        assert_eq!(result.highlights.len(), 2);
    }

    #[test]
    fn space_before_punctuation_does_not_cause_false_highlights() {
        let settings = FormattingSettings {
            built_in: BuiltInOptions {
                short_text_correction: 0,
                trailing_space: false,
                space_before_punctuation: true,
                ..Default::default()
            },
            rules: vec![],
        };
        let result =
            apply_formatting_with_highlights("tu viens? oui je viens".to_string(), &settings);
        assert_eq!(result.text, "tu viens ? oui je viens");
        assert!(
            result.highlights.is_empty(),
            "built-in rules should not generate highlights"
        );
    }

    #[test]
    fn custom_rule_with_space_before_punctuation() {
        let settings = FormattingSettings {
            built_in: BuiltInOptions {
                short_text_correction: 0,
                trailing_space: false,
                space_before_punctuation: true,
                ..Default::default()
            },
            rules: vec![FormattingRule {
                id: "test".to_string(),
                trigger: "hello".to_string(),
                replacement: "bonjour".to_string(),
                enabled: true,
                match_mode: MatchMode::Exact,
                ..Default::default()
            }],
        };
        let result = apply_formatting_with_highlights("hello world? yes".to_string(), &settings);
        assert_eq!(result.text, "bonjour world ? yes");
        assert_eq!(result.highlights.len(), 1);
        assert_eq!(result.highlights[0].start, 0);
        assert_eq!(result.highlights[0].end, 7);
    }

    #[test]
    fn dictionary_member_highlighted_without_any_change() {
        let settings = FormattingSettings {
            built_in: BuiltInOptions {
                short_text_correction: 0,
                trailing_space: false,
                ..Default::default()
            },
            rules: vec![],
        };
        let result = apply_formatting_with_highlights_and_original(
            "parakeet rocks".to_string(),
            "parakeet rocks".to_string(),
            &settings,
            &["parakeet".to_string()],
        );
        assert_eq!(result.highlights.len(), 1);
        assert_eq!(result.highlights[0].start, 0);
        assert_eq!(result.highlights[0].end, 8);
    }

    #[test]
    fn real_claude_code_regex_rule_repro() {
        let settings = FormattingSettings {
            built_in: BuiltInOptions {
                short_text_correction: 3,
                trailing_space: true,
                space_before_punctuation: true,
                ..Default::default()
            },
            rules: vec![FormattingRule {
                id: "cc".to_string(),
                trigger: "(?i)cloud( )?code".to_string(),
                replacement: "Claude code".to_string(),
                enabled: true,
                match_mode: MatchMode::Regex,
                ..Default::default()
            }],
        };
        let result = apply_formatting_with_highlights_and_original(
            "cloud code".to_string(),
            "cloud code".to_string(),
            &settings,
            &[],
        );
        assert_eq!(result.text, "Claude code");
        assert_eq!(result.highlights.len(), 2);
    }

    #[test]
    fn multi_word_rule_replacement_highlights_all_words() {
        let settings = settings_with_rule("cloudcode", "Claude code");
        let result = apply_formatting_with_highlights_and_original(
            "cloudcode here".to_string(),
            "cloudcode here".to_string(),
            &settings,
            &[],
        );
        assert_eq!(result.text, "Claude code here");
        assert_eq!(result.highlights.len(), 2);
    }
}
