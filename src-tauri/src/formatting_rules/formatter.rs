use super::types::{FormattingSettings, MatchMode};
use regex::Regex;
use text2num::{replace_numbers_in_text, Language};

/// Apply all formatting rules to a transcription text
pub fn apply_formatting(text: String, settings: &FormattingSettings) -> String {
    let mut result = text;

    // 1. Apply custom rules first (find/replace with punctuation handling)
    for rule in &settings.rules {
        if rule.enabled && !rule.trigger.is_empty() {
            result = apply_custom_rule(&result, &rule.trigger, &rule.replacement, &rule.match_mode);
        }
    }

    // 2. Apply built-in option: space before ? and !
    if settings.built_in.space_before_punctuation {
        result = add_space_before_punctuation(&result);
    }

    // 3. Apply built-in option: convert text numbers to digits
    if settings.built_in.convert_text_numbers {
        result = convert_text_numbers(
            &result,
            &settings.built_in.text_numbers_language,
            settings.built_in.text_numbers_threshold,
        );
    }

    // 4. Apply built-in option: trailing space
    if settings.built_in.trailing_space && !result.ends_with(' ') && !result.ends_with('\n') {
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
                r"(?i)(?:[,\.]\s|\s)?{escaped}[,\.]?",
                escaped = escaped_trigger
            );
            match Regex::new(&pattern) {
                Ok(re) => re.replace_all(text, replacement).to_string(),
                Err(_) => text.to_string(),
            }
        }
        MatchMode::Regex => match Regex::new(trigger) {
            Ok(re) => re.replace_all(text, replacement).to_string(),
            Err(_) => text.to_string(),
        },
    }
}
