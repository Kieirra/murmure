use std::collections::HashMap;
use std::fs;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use crate::dictionary::DictionaryError;

pub fn load(app: &AppHandle) -> Result<HashMap<String, Vec<String>>, String> {
    let store = app.store("dictionary.json").map_err(|e| e.to_string())?;
    let mut words = HashMap::new();
    for (key, value) in store.entries() {
        let languages = serde_json::from_value::<Vec<String>>(value).map_err(|e| e.to_string())?;
        words.insert(key, languages);
    }
    Ok(words)
}

pub fn save(app: &AppHandle, dictionary: &HashMap<String, Vec<String>>) -> Result<(), String> {
    let store = app.store("dictionary.json").map_err(|e| e.to_string())?;
    store.reset();
    for (word, languages) in dictionary {
        store.set(
            word,
            serde_json::to_value(languages).map_err(|e| e.to_string())?,
        );
    }
    Ok(())
}

pub fn migrate_and_load(
    app: &AppHandle,
    dictionary_from_settings: Vec<String>,
) -> Result<HashMap<String, Vec<String>>, String> {
    let mut dictionary = load(app)?;
    if !dictionary_from_settings.is_empty() {
        for word in dictionary_from_settings {
            dictionary
                .entry(word)
                .or_insert(vec!["english".to_string(), "french".to_string()]);
        }
        save(app, &dictionary)?;
    }
    Ok(dictionary)
}

pub fn export_dictionary(app: &AppHandle, file_path: String) -> Result<(), String> {
    log::debug!("Exporting dictionary to file: {}", file_path);
    let dictionary = load(app)?;
    let words: Vec<String> = dictionary.into_keys().collect();
    let csv_content = words.join("\n");

    fs::write(&file_path, csv_content).map_err(|e| e.to_string())?;
    Ok(())
}

fn validate_dictionary_format(new_dictionary: String) -> Result<Vec<String>, DictionaryError> {
    let words: Vec<&str> = new_dictionary.split('\n').collect();
    let mut valid_words: Vec<String> = Vec::new();

    for word in words {
        let trimmed = word.trim();
        if trimmed.is_empty() {
            continue;
        }

        if !trimmed.chars().all(|c| c.is_alphabetic()) {
            return Err(DictionaryError::InvalidWordFormat(trimmed.to_string()));
        }

        valid_words.push(trimmed.to_lowercase());
    }
    if valid_words.is_empty() {
        return Err(DictionaryError::EmptyDictionary);
    }
    Ok(valid_words)
}

pub fn import_dictionary(app: &AppHandle, file_path: String) -> Result<(), String> {
    let new_dictionary = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    log::debug!(
        "New dictionary: {} from file: {}",
        new_dictionary,
        &file_path
    );

    let valid_words = validate_dictionary_format(new_dictionary).map_err(|e| e.to_string())?;
    let mut dictionary = load(app)?;
    for word in valid_words {
        dictionary
            .entry(word)
            .or_insert(vec!["english".to_string(), "french".to_string()]);
    }
    save(app, &dictionary)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_dictionary_format_valid_multiple_words() {
        let result = validate_dictionary_format("hello\nWORLD\ntest".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["hello", "world", "test"]);
    }

    #[test]
    fn test_validate_dictionary_format_trims_whitespace() {
        let result = validate_dictionary_format("  hello  \n  world  \n  test  ".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["hello", "world", "test"]);
    }

    #[test]
    fn test_validate_dictionary_format_skips_empty_entries() {
        let result = validate_dictionary_format("hello\n\nworld\n\ntest".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["hello", "world", "test"]);
    }

    #[test]
    fn test_validate_dictionary_format_invalid_with_numbers() {
        let result = validate_dictionary_format("hello\nworld123\ntest".to_string());
        assert!(result.is_err());
        match result.unwrap_err() {
            DictionaryError::InvalidWordFormat(word) => {
                assert_eq!(word, "world123");
            }
            _ => panic!("Expected InvalidWordFormat error"),
        }
    }

    #[test]
    fn test_validate_dictionary_format_invalid_with_special_characters() {
        let result = validate_dictionary_format("hello\nworld-test\ntest".to_string());
        assert!(result.is_err());
        match result.unwrap_err() {
            DictionaryError::InvalidWordFormat(word) => {
                assert_eq!(word, "world-test");
            }
            _ => panic!("Expected InvalidWordFormat error"),
        }
    }

    #[test]
    fn test_validate_dictionary_format_invalid_with_spaces_in_word() {
        let result = validate_dictionary_format("hello \nworld test\ntest".to_string());
        assert!(result.is_err());
        match result.unwrap_err() {
            DictionaryError::InvalidWordFormat(word) => {
                assert_eq!(word, "world test");
            }
            _ => panic!("Expected InvalidWordFormat error"),
        }
    }

    #[test]
    fn test_validate_dictionary_format_empty_string() {
        let result = validate_dictionary_format("".to_string());
        assert!(result.is_err());
        match result.unwrap_err() {
            DictionaryError::EmptyDictionary => {}
            _ => panic!("Expected EmptyDictionary error"),
        }
    }

    #[test]
    fn test_validate_dictionary_format_only_newlines() {
        let result = validate_dictionary_format("\n\n\n".to_string());
        assert!(result.is_err());
        match result.unwrap_err() {
            DictionaryError::EmptyDictionary => {}
            _ => panic!("Expected EmptyDictionary error"),
        }
    }
}
