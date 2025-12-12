use std::{collections::HashMap, path::PathBuf};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;
use tauri::Manager;
use std::fs;

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

fn get_filename(download_dir: &PathBuf) -> String {

    let filename = "murmure-dictionary.csv";
    if (download_dir.join(filename).exists()) {
        let mut i = 1;
        while download_dir.join(&format!("murmure-dictionary-{}.csv", i)).exists() {
            i += 1;
        }
        return format!("murmure-dictionary-{}.csv", i);
    } 
    return filename.to_string();    
}

pub fn export_dictionary(app: &AppHandle) -> Result<(), String> {
    let dictionary = load(&app)?;
    let words: Vec<String> = dictionary.into_keys().collect();

    // Use the download directpry
    let download_dir = app
    .path()
    .download_dir()
    .map_err(|e| e.to_string())?;

    let filename = get_filename(&download_dir);
    let file_path = download_dir.join(filename);

    let csv_content = words.join(",");
    fs::write(&file_path, csv_content)
        .map_err(|e| e.to_string())?;

    Ok(())
}
