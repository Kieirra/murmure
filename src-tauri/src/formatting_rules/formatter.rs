use super::types::FormattingSettings;
use regex::Regex;
use text2num::{replace_numbers_in_text, Language};

/// Apply all formatting rules to a transcription text
pub fn apply_formatting(text: String, settings: &FormattingSettings) -> String {
    let mut result = text;

    // 1. Apply custom rules first (find/replace with punctuation handling)
    // 1. Apply custom rules first (find/replace with punctuation handling)
    for rule in &settings.rules {
        if rule.enabled && !rule.trigger.is_empty() {
            result = apply_custom_rule(
                &result, 
                &rule.trigger, 
                &rule.replacement, 
                rule.exact_match, 
                rule.use_regex
            );
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

/// Apply a custom rule with optional punctuation handling
/// - exact_match=true:  Simple string replace (e.g., "*" -> "")
/// - exact_match=false: Smart replace with surrounding punctuation handling
/// - use_regex=true: Treat trigger as regex pattern
fn apply_custom_rule(text: &str, trigger: &str, replacement: &str, exact_match: bool, use_regex: bool) -> String {
    if use_regex {
        return match Regex::new(trigger) {
            Ok(re) => re.replace_all(text, replacement).to_string(),
            Err(_) => text.to_string(),
        };
    }
    
    if exact_match {
        // Exact match: simple string replacement
        return text.replace(trigger, replacement);
    }

    // Smart match: handle surrounding spaces and punctuation
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formatting_rules::types::{BuiltInOptions, FormattingRule};

    #[test]
    fn test_apply_regex_rule() {
        let settings = FormattingSettings {
            built_in: BuiltInOptions::default(),
            rules: vec![
                FormattingRule {
                    id: "1".to_string(),
                    trigger: r"\b(\d{4})\b".to_string(), // Matches 4 digits year
                    replacement: "Year $1".to_string(),
                    enabled: true,
                    exact_match: false,
                    use_regex: true,
                },
                FormattingRule {
                    id: "2".to_string(),
                    trigger: r"\s+".to_string(), // Matches multiple spaces
                    replacement: " ".to_string(),
                    enabled: true,
                    exact_match: false,
                    use_regex: true,
                }
            ],
        };

        let text = "In 2024   there was   peace.";
        let result = apply_formatting(text.to_string(), &settings);
        
        // Rule 1: "2024" -> "Year 2024"
        // Rule 2: "   " -> " "
        assert_eq!(result, "In Year 2024 there was peace.");
    }
    
    #[test]
    fn test_invalid_regex_fallback() {
         let settings = FormattingSettings {
            built_in: BuiltInOptions::default(),
            rules: vec![FormattingRule {
                id: "1".to_string(),
                trigger: r"[invalid".to_string(), 
                replacement: "replaced".to_string(),
                enabled: true,
                exact_match: false,
                use_regex: true,
            }],
        };
        
        let text = "some text";
        let result = apply_formatting(text.to_string(), &settings);
        assert_eq!(result, "some text"); // Should match input on error
    }
}
