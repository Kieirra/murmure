use std::collections::HashMap;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

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
    for (word, languages) in dictionary {
        store.set(
            word,
            serde_json::to_value(languages).map_err(|e| e.to_string())?,
        );
    }
    Ok(())
}
