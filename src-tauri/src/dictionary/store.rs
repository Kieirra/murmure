use std::collections::HashMap;
use std::fs;
use std::path::Path;
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

fn get_filename(download_dir: &Path) -> String {
    let filename = "murmure-dictionary.csv";
    if download_dir.join(filename).exists() {
        let mut i = 1;
        while download_dir
            .join(format!("murmure-dictionary-{}.csv", i))
            .exists()
        {
            i += 1;
        }
        return format!("murmure-dictionary-{}.csv", i);
    }
    filename.to_string()
}

pub fn export_dictionary(app: &AppHandle, directory: String) -> Result<(), String> {
    log::debug!("Exporting dictionary to directory: {}", directory);
    let download_dir = Path::new(&directory);
    let filename = get_filename(download_dir);
    let file_path = download_dir.join(filename);

    let dictionary = load(app)?;
    let words: Vec<String> = dictionary.into_keys().collect();
    let csv_content = words.join(",");

    fs::write(&file_path, csv_content).map_err(|e| e.to_string())?;
    Ok(())
}

fn validate_dictionary_format(new_dictionary: String) -> Result<Vec<String>, DictionaryError> {
    let words: Vec<&str> = new_dictionary.split(',').collect();
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
    use std::fs;

    #[test]
    fn test_get_filename() {
        let temp_dir = std::env::temp_dir().join("murmure_test_1");
        fs::create_dir_all(&temp_dir).unwrap();
        let result = get_filename(&temp_dir);
        assert_eq!(result, "murmure-dictionary.csv");
        let result = get_filename(&temp_dir);
        assert_eq!(result, "murmure-dictionary.csv");
        let base_file = temp_dir.join(result);
        fs::write(&base_file, "test").unwrap();
        let result = get_filename(&temp_dir);
        assert_eq!(result, "murmure-dictionary-1.csv");
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_validate_dictionary_format_valid_multiple_words() {
        let result = validate_dictionary_format("hello,WORLD,test".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["hello", "world", "test"]);
    }

    #[test]
    fn test_validate_dictionary_format_trims_whitespace() {
        let result = validate_dictionary_format("  hello  ,  world  ,  test  ".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["hello", "world", "test"]);
    }

    #[test]
    fn test_validate_dictionary_format_skips_empty_entries() {
        let result = validate_dictionary_format("hello,,world,,test".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["hello", "world", "test"]);
    }

    #[test]
    fn test_validate_dictionary_format_invalid_with_numbers() {
        let result = validate_dictionary_format("hello,world123,test".to_string());
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
        let result = validate_dictionary_format("hello,world-test,test".to_string());
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
        let result = validate_dictionary_format("hello,world test,test".to_string());
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
    fn test_validate_dictionary_format_only_commas() {
        let result = validate_dictionary_format(",,,".to_string());
        assert!(result.is_err());
        match result.unwrap_err() {
            DictionaryError::EmptyDictionary => {}
            _ => panic!("Expected EmptyDictionary error"),
        }
    }
}
