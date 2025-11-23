use crate::settings;
use tauri::{AppHandle, command};

#[command]
pub fn get_current_language(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.language)
}

#[command]
pub fn set_current_language(app: AppHandle, lang: String) -> Result<(), String> {
    const SUPPORTED_LANGUAGES: &[&str] = &["default", "en", "fr"];

    if !SUPPORTED_LANGUAGES.contains(&lang.as_str()) {
        return Err(format!("Unsupported language code: {}", lang));
    }

    let mut s = settings::load_settings(&app);
    s.language = lang;
    settings::save_settings(&app, &s)
}
