use std::fs;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use crate::dictionary::{normalize_import_content, DictionaryError};

fn contains_word_case_insensitive(words: &[String], word: &str) -> bool {
    words.iter().any(|w| w.eq_ignore_ascii_case(word))
}

/// Words are the store keys. The value is a vestigial per-word language list:
/// ignored on read and written empty, so the on-disk format stays compatible
/// with older versions in both directions.
pub fn load(app: &AppHandle) -> Result<Vec<String>, String> {
    let store = app.store("dictionary.json").map_err(|e| e.to_string())?;
    Ok(store.entries().into_iter().map(|(word, _)| word).collect())
}

pub fn save(app: &AppHandle, words: &[String]) -> Result<(), String> {
    let store = app.store("dictionary.json").map_err(|e| e.to_string())?;
    store.reset();
    for word in words {
        store.set(word, serde_json::json!([]));
    }
    Ok(())
}

pub fn migrate_and_load(
    app: &AppHandle,
    dictionary_from_settings: Vec<String>,
) -> Result<Vec<String>, String> {
    let mut words = load(app)?;
    if !dictionary_from_settings.is_empty() {
        for word in dictionary_from_settings {
            if !contains_word_case_insensitive(&words, &word) {
                words.push(word);
            }
        }
        save(app, &words)?;
    }
    Ok(words)
}

pub fn export_dictionary(app: &AppHandle, file_path: String) -> Result<(), String> {
    log::debug!("Exporting dictionary to file: {}", file_path);
    let words = load(app)?;
    let content = match words.is_empty() {
        true => String::new(),
        false => format!("{}\n", words.join("\n")),
    };

    fs::write(&file_path, content).map_err(|e| e.to_string())?;
    Ok(())
}

fn validate_dictionary_format(words: &[String]) -> Result<(), DictionaryError> {
    for word in words {
        let has_digit = word.chars().any(|c| c.is_ascii_digit());
        let space_count = word.chars().filter(|c| *c == ' ').count();
        if has_digit || space_count > 1 {
            return Err(DictionaryError::InvalidWordFormat(word.to_string()));
        }
    }
    if words.is_empty() {
        return Err(DictionaryError::EmptyDictionary);
    }
    Ok(())
}

pub fn import_dictionary(app: &AppHandle, file_path: String) -> Result<(), String> {
    let raw = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    log::debug!("New dictionary: {} from file: {}", raw, &file_path);

    let is_legacy_csv = std::path::Path::new(&file_path)
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("csv"));

    let normalized = normalize_import_content(&raw, is_legacy_csv);
    validate_dictionary_format(&normalized).map_err(|e| e.to_string())?;
    let mut words = load(app)?;
    for word in normalized {
        if !contains_word_case_insensitive(&words, &word) {
            words.push(word);
        }
    }
    save(app, &words)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_dictionary_format_valid_multiple_words() {
        let result =
            validate_dictionary_format(&normalize_import_content("hello\nWORLD\ntest", false));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_dictionary_format_trims_whitespace() {
        let result = validate_dictionary_format(&normalize_import_content(
            "  hello  \n  world  \n  test  ",
            false,
        ));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_dictionary_format_skips_empty_entries() {
        let result =
            validate_dictionary_format(&normalize_import_content("hello\n\nworld\n\ntest", false));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_dictionary_format_invalid_with_numbers() {
        let result =
            validate_dictionary_format(&normalize_import_content("hello\nworld123\ntest", false));
        assert!(result.is_err());
        match result.unwrap_err() {
            DictionaryError::InvalidWordFormat(word) => {
                assert_eq!(word, "world123");
            }
            _ => panic!("Expected InvalidWordFormat error"),
        }
    }

    #[test]
    fn test_validate_dictionary_format_valid_with_hyphen() {
        let result =
            validate_dictionary_format(&normalize_import_content("hello\nworld-test\ntest", false));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_dictionary_format_valid_two_word_pair() {
        let result = validate_dictionary_format(&normalize_import_content(
            "hello \nworld test\ntest",
            false,
        ));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_dictionary_format_invalid_multiple_spaces() {
        let result =
            validate_dictionary_format(&normalize_import_content("hello\na b c\ntest", false));
        assert!(result.is_err());
        match result.unwrap_err() {
            DictionaryError::InvalidWordFormat(word) => {
                assert_eq!(word, "a b c");
            }
            _ => panic!("Expected InvalidWordFormat error"),
        }
    }

    #[test]
    fn test_validate_dictionary_format_valid_with_apostrophe() {
        let result = validate_dictionary_format(&normalize_import_content("aujourd'hui", false));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_dictionary_format_empty_string() {
        let result = validate_dictionary_format(&normalize_import_content("", false));
        assert!(result.is_err());
        match result.unwrap_err() {
            DictionaryError::EmptyDictionary => {}
            _ => panic!("Expected EmptyDictionary error"),
        }
    }

    #[test]
    fn test_validate_dictionary_format_only_newlines() {
        let result = validate_dictionary_format(&normalize_import_content("\n\n\n", false));
        assert!(result.is_err());
        match result.unwrap_err() {
            DictionaryError::EmptyDictionary => {}
            _ => panic!("Expected EmptyDictionary error"),
        }
    }
}
