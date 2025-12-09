use super::types::FormattingSettings;

/// Apply all formatting rules to a transcription text
pub fn apply_formatting(text: String, settings: &FormattingSettings) -> String {
    let mut result = text;

    // 1. Apply custom rules first (find/replace)
    for rule in &settings.rules {
        if rule.enabled && !rule.trigger.is_empty() {
            result = result.replace(&rule.trigger, &rule.replacement);
        }
    }

    // 2. Apply built-in option: space before ? and !
    if settings.built_in.space_before_punctuation {
        result = add_space_before_punctuation(&result);
    }

    // 3. Apply built-in option: trailing space
    if settings.built_in.trailing_space {
        if !result.ends_with(' ') && !result.ends_with('\n') {
            result.push(' ');
        }
    }

    result
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