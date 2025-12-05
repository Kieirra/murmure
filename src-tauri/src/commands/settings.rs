use tauri::{command, AppHandle};

#[command]
pub fn get_current_language(app: AppHandle) -> Result<String, String> {
    let s = crate::settings::load_settings(&app);
    Ok(s.language)
}

#[command]
pub fn set_current_language(app: AppHandle, lang: String) -> Result<(), String> {
    const SUPPORTED_LANGUAGES: &[&str] = &["default", "en", "fr"];

    if !SUPPORTED_LANGUAGES.contains(&lang.as_str()) {
        return Err(format!("Unsupported language code: {}", lang));
    }

    let mut s = crate::settings::load_settings(&app);
    s.language = lang;
    crate::settings::save_settings(&app, &s)
}

#[command]
pub fn get_sound_enabled(app: AppHandle) -> Result<bool, String> {
    let s = crate::settings::load_settings(&app);
    Ok(s.sound_enabled)
}

#[command]
pub fn set_sound_enabled(app: AppHandle, enabled: bool) -> Result<(), String> {
    let mut s = crate::settings::load_settings(&app);
    s.sound_enabled = enabled;
    crate::settings::save_settings(&app, &s)
}
