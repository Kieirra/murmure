use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};

use super::types::AppSettings;

fn settings_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    if let Err(e) = fs::create_dir_all(&dir) {
        return Err(format!("create_dir_all failed: {}", e));
    }
    Ok(dir.join("settings.json"))
}

pub fn load_settings(app: &AppHandle) -> AppSettings {
    let path = match settings_path(app) {
        Ok(p) => p,
        Err(_) => return AppSettings::default(),
    };

    match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str::<AppSettings>(&content).unwrap_or_default(),
        Err(_) => {
            let defaults = AppSettings::default();
            let _ = save_settings(app, &defaults);
            defaults
        }
    }
}

pub fn save_settings(app: &AppHandle, settings: &AppSettings) -> Result<(), String> {
    let mut settings_clone = settings.clone();
    // remove legacy dictionary from settings if still present
    if !settings_clone.dictionary.is_empty() {
        settings_clone.dictionary = Vec::new();
    }
    let path = settings_path(app)?;
    let content = serde_json::to_string_pretty(&settings_clone).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())
}
