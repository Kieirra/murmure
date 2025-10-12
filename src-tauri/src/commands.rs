use crate::history::{self, HistoryEntry};
use crate::model::Model;
use crate::settings;
use crate::shortcuts::{keys_to_string, parse_binding_keys, ShortcutKeys};
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};

#[tauri::command]
pub fn is_model_available(model: State<Arc<Model>>) -> bool {
    model.is_available()
}

#[tauri::command]
pub fn get_model_path(model: State<Arc<Model>>) -> Result<String, String> {
    let path = model.get_model_path().map_err(|e| format!("{:#}", e))?;

    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn get_recent_transcriptions(app: AppHandle) -> Result<Vec<HistoryEntry>, String> {
    history::get_recent_transcriptions(&app).map_err(|e| format!("{:#}", e))
}

#[tauri::command]
pub fn get_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.shortcut)
}

#[tauri::command]
pub fn set_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    let keys = parse_binding_keys(&binding);
    if keys.is_empty() {
        return Err("Invalid shortcut".to_string());
    }
    let normalized = keys_to_string(&keys);

    let mut s = settings::load_settings(&app);
    s.shortcut = normalized.clone();
    settings::save_settings(&app, &s)?;

    app.state::<ShortcutKeys>().set(keys);

    Ok(normalized)
}
