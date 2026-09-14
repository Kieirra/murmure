use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};

use super::types::AppSettings;

/// Serializes load-modify-save of `settings.json` so two commands cannot
/// overwrite each other. The mutex is not reentrant: do not call
/// `load_settings` / `save_settings` / `update_settings` from inside `update`.
static SETTINGS_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

fn settings_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    if let Err(e) = fs::create_dir_all(&dir) {
        return Err(format!("create_dir_all failed: {}", e));
    }
    Ok(dir.join("settings.json"))
}

fn load_settings_unlocked(app: &AppHandle) -> AppSettings {
    let path = match settings_path(app) {
        Ok(p) => p,
        Err(_) => return AppSettings::default(),
    };

    match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str::<AppSettings>(&content).unwrap_or_default(),
        Err(_) => {
            let defaults = AppSettings::default();
            let _ = save_settings_unlocked(app, &defaults);
            defaults
        }
    }
}

fn save_settings_unlocked(app: &AppHandle, settings: &AppSettings) -> Result<(), String> {
    let path = settings_path(app)?;
    let content = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())
}

pub fn load_settings(app: &AppHandle) -> AppSettings {
    let _guard = SETTINGS_LOCK.lock();
    load_settings_unlocked(app)
}

pub fn save_settings(app: &AppHandle, settings: &AppSettings) -> Result<(), String> {
    let _guard = SETTINGS_LOCK.lock();
    save_settings_unlocked(app, settings)
}

pub fn update_settings<F, T>(app: &AppHandle, update: F) -> Result<T, String>
where
    F: FnOnce(&mut AppSettings) -> Result<T, String>,
{
    let _guard = SETTINGS_LOCK.lock();
    let mut settings = load_settings_unlocked(app);
    let out = update(&mut settings)?;
    save_settings_unlocked(app, &settings)?;
    Ok(out)
}

pub fn remove_dictionary_from_settings(app: &AppHandle) -> Result<AppSettings, String> {
    update_settings(app, |s| {
        s.dictionary = Vec::new();
        Ok(s.clone())
    })
}
