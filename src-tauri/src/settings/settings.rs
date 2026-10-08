use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};

use super::helpers::{migrate_legacy_paste_method, normalize_result_panel_mode};
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
        Ok(content) => {
            let mut settings = parse_settings(&content);
            settings.result_panel_mode =
                normalize_result_panel_mode(&settings.result_panel_mode).to_string();
            settings
        }
        Err(_) => {
            let defaults = AppSettings::default();
            let _ = save_settings(app, &defaults);
            defaults
        }
    }
}

pub fn save_settings(app: &AppHandle, settings: &AppSettings) -> Result<(), String> {
    let path = settings_path(app)?;
    let content = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())
}

pub fn remove_dictionary_from_settings(
    app: &AppHandle,
    mut settings: AppSettings,
) -> Result<AppSettings, String> {
    settings.dictionary = Vec::new();
    save_settings(app, &settings)?;
    Ok(settings)
}

fn parse_settings(content: &str) -> AppSettings {
    let mut value = match serde_json::from_str::<serde_json::Value>(content) {
        Ok(value) => value,
        Err(_) => return AppSettings::default(),
    };
    migrate_legacy_paste_method(&mut value);
    serde_json::from_value(value).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::PasteMethod;

    #[test]
    fn legacy_none_paste_method_disables_auto_insert_and_keeps_other_settings() {
        for legacy in ["none", "None"] {
            let content = format!(
                r#"{{"paste_method":"{}","copy_to_clipboard":true,"api_port":4900}}"#,
                legacy
            );

            let settings = parse_settings(&content);

            assert_eq!(settings.paste_method, PasteMethod::CtrlV);
            assert!(!settings.auto_insert);
            assert!(settings.copy_to_clipboard);
            assert_eq!(settings.api_port, 4900);
        }
    }

    #[test]
    fn missing_auto_insert_defaults_to_true() {
        let settings = parse_settings(r#"{"paste_method":"direct"}"#);

        assert_eq!(settings.paste_method, PasteMethod::Direct);
        assert!(settings.auto_insert);
    }
}
