use super::types::FormattingSettings;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const STORE_FILE: &str = "formatting_rules.json";
const SETTINGS_KEY: &str = "settings";

/// Load formatting settings from the store
pub fn load(app: &AppHandle) -> Result<FormattingSettings, String> {
    let store = app.store(STORE_FILE).map_err(|e| e.to_string())?;

    match store.get(SETTINGS_KEY) {
        Some(value) => serde_json::from_value::<FormattingSettings>(value)
            .map_err(|e| format!("Failed to parse formatting settings: {}", e)),
        None => Ok(FormattingSettings::default()),
    }
}

/// Save formatting settings to the store
pub fn save(app: &AppHandle, settings: &FormattingSettings) -> Result<(), String> {
    let store = app.store(STORE_FILE).map_err(|e| e.to_string())?;

    let value = serde_json::to_value(settings)
        .map_err(|e| format!("Failed to serialize formatting settings: {}", e))?;

    store.set(SETTINGS_KEY, value);

    Ok(())
}
