
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;


pub fn load_dictionary(app: &AppHandle) -> Result<Vec<String>, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let value = store.get("dictionary");
    match value {
        Some(value) => {
            let content = serde_json::from_value::<Vec<String>>(value).map_err(|e| e.to_string())?;
            return Ok(content);
        }
        None => {
            return Ok(Vec::new());
        }
    }
}

pub fn save_dictionary(app: &AppHandle, dictionary: &Vec<String>) -> Result<(), String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let content = serde_json::to_value(dictionary).map_err(|e| e.to_string())?;
    store.set("dictionary", content);
    return Ok(());
}
