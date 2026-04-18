use super::formatter::{apply_custom_rule, apply_formatting};
use super::types::FormattingSettings;
use serde::Serialize;

pub struct FormattedWithHighlights {
    pub text: String,
    pub highlights: Vec<HighlightRange>,
}

#[derive(Serialize, Clone)]
pub struct HighlightRange {
    pub start: usize,
    pub end: usize,
}

pub fn apply_formatting_with_highlights(
    raw_text: String,
    settings: &FormattingSettings,
) -> FormattedWithHighlights {
    let original = raw_text.clone();
    apply_formatting_with_highlights_and_original(raw_text, original, settings)
}

pub fn apply_formatting_with_highlights_and_original(
    raw_text: String,
    original_text: String,
    settings: &FormattingSettings,
) -> FormattedWithHighlights {
    let mut custom_applied = raw_text.clone();
    let formatted = apply_formatting(raw_text, settings);

    for rule in &settings.rules {
        if rule.enabled && !rule.trigger.is_empty() {
            custom_applied = apply_custom_rule(
                &custom_applied,
                &rule.trigger,
                &rule.replacement,
                &rule.match_mode,
            );
        }
    }

    // Compare original (pre-dictionary) words with post-dictionary+formatting words
    // This detects both dictionary corrections and formatting rule changes
    let original_words: Vec<&str> = original_text.split_whitespace().collect();
    let custom_words: Vec<&str> = custom_applied.split_whitespace().collect();

    if custom_words.len() != original_words.len() {
        let changed_via_lcs = lcs_changed_words(&original_words, &custom_words);
        return build_highlights_from_changed(&formatted, &changed_via_lcs);
    }

    let changed_words: std::collections::HashSet<String> = custom_words
        .iter()
        .enumerate()
        .filter_map(|(i, cw)| {
            let orig_word = original_words.get(i).copied().unwrap_or("");
            if *cw != orig_word {
                Some(cw.to_string())
            } else {
                None
            }
        })
        .collect();

    build_highlights_from_changed(&formatted, &changed_words)
}

fn lcs_changed_words(raw: &[&str], custom: &[&str]) -> std::collections::HashSet<String> {
    let m = raw.len();
    let n = custom.len();
    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if raw[i - 1] == custom[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    let mut changed = std::collections::HashSet::new();
    let mut i = m;
    let mut j = n;
    let mut matched_custom = vec![false; n];

    while i > 0 && j > 0 {
        if raw[i - 1] == custom[j - 1] {
            matched_custom[j - 1] = true;
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] >= dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    for (idx, matched) in matched_custom.iter().enumerate() {
        if !matched {
            changed.insert(custom[idx].to_string());
        }
    }

    changed
}

fn build_highlights_from_changed(
    formatted: &str,
    changed_words: &std::collections::HashSet<String>,
) -> FormattedWithHighlights {
    let formatted_words: Vec<&str> = formatted.split_whitespace().collect();
    let mut highlights = Vec::new();
    let mut byte_offset: usize = 0;

    for fw in &formatted_words {
        let word_start = match formatted[byte_offset..].find(fw) {
            Some(pos) => byte_offset + pos,
            None => byte_offset,
        };
        let word_end = word_start + fw.len();

        if changed_words.contains(*fw) {
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
                },
                FormattingRule {
                    id: "2".to_string(),
                    trigger: "world".to_string(),
                    replacement: "monde".to_string(),
                    enabled: true,
                    match_mode: MatchMode::Exact,
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
            }],
        };
        let result = apply_formatting_with_highlights("hello world? yes".to_string(), &settings);
        assert_eq!(result.text, "bonjour world ? yes");
        assert_eq!(result.highlights.len(), 1);
        assert_eq!(result.highlights[0].start, 0);
        assert_eq!(result.highlights[0].end, 7);
    }
}
