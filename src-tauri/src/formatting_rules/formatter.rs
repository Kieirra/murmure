use super::types::{FormattingSettings, MatchMode};
use regex::Regex;
use text2num::{replace_numbers_in_text, Language};

/// Apply short text correction: for transcriptions with word count <= max_words,
/// remove trailing punctuation and lowercase first letter of Capitalized words.
/// Acronyms (ALL CAPS) and mixed-case words (iPhone) are preserved.
fn apply_short_text_correction(text: String) -> String {
    let mut result = text;

    // 1. Remove trailing punctuation
    if let Some(last_char) = result.chars().last() {
        if matches!(last_char, '.' | '!' | '?' | ',') {
            result.pop();
        }
    }

    // 2. Lowercase first letter of Capitalized words (not ALL CAPS, not mixedCase)
    result = result
        .split_whitespace()
        .map(|word| {
            let chars: Vec<char> = word.chars().collect();
            if chars.len() >= 2 && chars[0].is_uppercase() && chars[1].is_lowercase() {
                // Pattern: "Hello" → "hello" (first uppercase + second lowercase)
                let mut lowered = String::new();
                for c in chars[0].to_lowercase() {
                    lowered.push(c);
                }
                lowered.extend(&chars[1..]);
                lowered
            } else {
                // "API", "iPhone", "ok", "I" → unchanged
                word.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    result
}

/// Apply all formatting rules to a transcription text
pub fn apply_formatting(text: String, settings: &FormattingSettings) -> String {
    let mut result = text;

    // 1. Short text correction (configurable threshold, 0 = disabled)
    let threshold = settings.built_in.short_text_correction;
    let word_count = result.split_whitespace().count();
    let is_short_text = if threshold > 0 && word_count > 0 && word_count <= threshold {
        result = apply_short_text_correction(result);
        true
    } else {
        false
    };

    // 2. Apply custom rules (find/replace with punctuation handling)
    for rule in &settings.rules {
        if rule.enabled && !rule.trigger.is_empty() {
            result = apply_custom_rule(&result, &rule.trigger, &rule.replacement, &rule.match_mode);
        }
    }

    // 3. Apply built-in option: space before ? and !
    if settings.built_in.space_before_punctuation {
        result = add_space_before_punctuation(&result);
    }

    // 4. Apply built-in option: convert text numbers to digits
    if settings.built_in.convert_text_numbers {
        result = convert_text_numbers(
            &result,
            &settings.built_in.text_numbers_language,
            settings.built_in.text_numbers_threshold,
        );
    }

    // 5. Apply built-in option: trailing space (skip for short texts inserted mid-sentence)
    if !is_short_text
        && settings.built_in.trailing_space
        && !result.ends_with(' ')
        && !result.ends_with('\n')
    {
        result.push(' ');
    }

    result
}

/// Convert text numbers to digits (e.g., "one" -> "1")
fn convert_text_numbers(text: &str, language: &str, threshold: f64) -> String {
    let lang = match language {
        "fr" => Language::french(),
        "en" => Language::english(),
        "de" => Language::german(),
        "it" => Language::italian(),
        "es" => Language::spanish(),
        "nl" => Language::dutch(),
        "pt" => Language::portuguese(),
        _ => Language::english(),
    };
    replace_numbers_in_text(text, &lang, threshold)
}

/// Add a space before ? and ! if they are preceded by a non-space character
fn add_space_before_punctuation(text: &str) -> String {
    let mut result = String::with_capacity(text.len() + 10);
    let chars: Vec<char> = text.chars().collect();

    for (i, c) in chars.iter().enumerate() {
        if (*c == '?' || *c == '!') && i > 0 {
            let prev = chars[i - 1];
            // Only add space if previous character is not already a space or newline
            if prev != ' ' && prev != '\n' && prev != '\t' {
                result.push(' ');
            }
        }
        result.push(*c);
    }

    result
}

/// Apply a custom rule based on the match mode
/// - Exact:  Simple string replace (e.g., "*" -> "")
/// - Smart:  Replace with surrounding punctuation handling (case-insensitive)
/// - Regex:  User-provided regex pattern with capture group support ($1, $2...)
fn apply_custom_rule(
    text: &str,
    trigger: &str,
    replacement: &str,
    match_mode: &MatchMode,
) -> String {
    match match_mode {
        MatchMode::Exact => text.replace(trigger, replacement),
        MatchMode::Smart => {
            let escaped_trigger = regex::escape(trigger);
            let pattern = format!(
                r"(?i)(?P<pre>(?:[,\.]\s|\s)?){escaped}[,\.]?",
                escaped = escaped_trigger
            );
            match Regex::new(&pattern) {
                Ok(re) if replacement.is_empty() => re.replace_all(text, "").to_string(),
                Ok(re) => {
                    let escaped = replacement.replace("$", "$$");
                    let replacement = format!("${{pre}}{}", escaped);
                    re.replace_all(text, replacement.as_str()).to_string()
                }
                Err(_) => text.to_string(),
            }
        }
        MatchMode::Regex => match Regex::new(trigger) {
            Ok(re) => re.replace_all(text, replacement).to_string(),
            Err(_) => text.to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::super::types::BuiltInOptions;
    use super::*;

    // Tests for apply_short_text_correction (pure transformation, no threshold guard)
    #[test]
    fn short_text_single_word_with_capital_and_period() {
        assert_eq!(apply_short_text_correction("Bonjour.".into()), "bonjour");
    }

    #[test]
    fn short_text_two_words() {
        assert_eq!(
            apply_short_text_correction("Très bien.".into()),
            "très bien"
        );
    }

    #[test]
    fn short_text_three_words() {
        assert_eq!(
            apply_short_text_correction("Un deux trois.".into()),
            "un deux trois"
        );
    }

    #[test]
    fn short_text_acronym_preserved() {
        assert_eq!(apply_short_text_correction("API.".into()), "API");
        assert_eq!(apply_short_text_correction("OK.".into()), "OK");
    }

    #[test]
    fn short_text_mixed_word_and_acronym() {
        assert_eq!(apply_short_text_correction("Mon API.".into()), "mon API");
    }

    #[test]
    fn short_text_already_lowercase() {
        assert_eq!(apply_short_text_correction("test.".into()), "test");
    }

    #[test]
    fn short_text_no_punctuation() {
        assert_eq!(apply_short_text_correction("Bonjour".into()), "bonjour");
    }

    #[test]
    fn short_text_word_with_apostrophe() {
        assert_eq!(
            apply_short_text_correction("Aujourd'hui.".into()),
            "aujourd'hui"
        );
    }

    #[test]
    fn short_text_mixed_case_preserved() {
        assert_eq!(apply_short_text_correction("iPhone.".into()), "iPhone");
    }

    #[test]
    fn short_text_exclamation_mark() {
        assert_eq!(apply_short_text_correction("Non!".into()), "non");
    }

    #[test]
    fn short_text_question_mark() {
        assert_eq!(apply_short_text_correction("Quoi?".into()), "quoi");
    }

    #[test]
    fn short_text_trailing_comma() {
        assert_eq!(apply_short_text_correction("Donc,".into()), "donc");
    }

    #[test]
    fn short_text_single_uppercase_letter() {
        assert_eq!(apply_short_text_correction("I.".into()), "I");
    }

    // Threshold tests via apply_formatting
    fn make_settings(threshold: usize) -> FormattingSettings {
        FormattingSettings {
            built_in: BuiltInOptions {
                short_text_correction: threshold,
                ..Default::default()
            },
            rules: vec![],
        }
    }

    #[test]
    fn threshold_0_disables_correction() {
        let result = apply_formatting("Bonjour.".into(), &make_settings(0));
        assert!(result.contains("Bonjour."));
    }

    #[test]
    fn threshold_1_only_single_word() {
        assert_eq!(
            apply_formatting("Bonjour.".into(), &make_settings(1)).trim(),
            "bonjour"
        );
        assert!(apply_formatting("Très bien.".into(), &make_settings(1)).contains("Très bien."));
    }

    #[test]
    fn threshold_3_corrects_up_to_3_words() {
        assert_eq!(
            apply_formatting("Bonjour.".into(), &make_settings(3)).trim(),
            "bonjour"
        );
        assert_eq!(
            apply_formatting("Un deux trois.".into(), &make_settings(3)).trim(),
            "un deux trois"
        );
        assert!(
            apply_formatting("Un deux trois quatre.".into(), &make_settings(3))
                .contains("Un deux trois quatre.")
        );
    }

    #[test]
    fn threshold_5_corrects_up_to_5_words() {
        assert_eq!(
            apply_formatting("Un deux trois quatre cinq.".into(), &make_settings(5)).trim(),
            "un deux trois quatre cinq"
        );
        assert!(
            apply_formatting("Un deux trois quatre cinq six.".into(), &make_settings(5))
                .contains("Un deux trois quatre cinq six.")
        );
    }

    // Tests for Smart mode auto-spacing
    #[test]
    fn smart_mode_preserves_space_mid_sentence() {
        let result = apply_custom_rule("I'm gonna go", "gonna", "going to", &MatchMode::Smart);
        assert_eq!(result, "I'm going to go");
    }

    #[test]
    fn smart_mode_no_leading_space_at_start() {
        let result = apply_custom_rule("Gonna go now", "gonna", "going to", &MatchMode::Smart);
        assert_eq!(result, "going to go now");
    }

    #[test]
    fn smart_mode_preserves_punctuation_prefix() {
        let result = apply_custom_rule("hello, gonna go", "gonna", "going to", &MatchMode::Smart);
        assert_eq!(result, "hello, going to go");
    }

    #[test]
    fn smart_mode_leading_space_in_replacement_is_preserved() {
        let result = apply_custom_rule("I'm gonna go", "gonna", " going to", &MatchMode::Smart);
        assert_eq!(result, "I'm  going to go");
    }

    #[test]
    fn smart_mode_empty_replacement_deletes_with_space() {
        let result = apply_custom_rule("hello world foo", "world", "", &MatchMode::Smart);
        assert_eq!(result, "hello foo");
    }

    #[test]
    fn smart_mode_empty_replacement_at_start() {
        let result = apply_custom_rule("world foo", "world", "", &MatchMode::Smart);
        assert_eq!(result, " foo");
    }

    #[test]
    fn smart_mode_case_insensitive() {
        let result = apply_custom_rule("Hello WORLD test", "world", "earth", &MatchMode::Smart);
        assert_eq!(result, "Hello earth test");
    }

    #[test]
    fn smart_mode_strips_trailing_punctuation() {
        let result = apply_custom_rule("hello world.", "world", "earth", &MatchMode::Smart);
        assert_eq!(result, "hello earth");
    }
}
