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
pub fn get_current_mic_id(app: AppHandle) -> Result<Option<String>, String> {
    let s = crate::settings::load_settings(&app);
    Ok(s.mic_id)
}

#[command]
pub fn set_current_mic_id(app: AppHandle, mic_id: Option<String>) -> Result<(), String> {
    let mut s = crate::settings::load_settings(&app);
    s.mic_id = mic_id.clone();
    crate::settings::save_settings(&app, &s)?;
    crate::audio::microphone::update_mic_cache(&app, mic_id);
    Ok(())
}

#[command]
pub fn get_mic_list() -> Result<Vec<String>, String> {
    let mic_list = crate::audio::microphone::get_mic_list();
    Ok(mic_list)
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

#[command]
pub fn get_log_level(app: AppHandle) -> Result<String, String> {
    let s = crate::settings::load_settings(&app);
    Ok(s.log_level)
}

#[command]
pub fn set_log_level(app: AppHandle, level: String) -> Result<(), String> {
    let valid_levels = ["off", "error", "warn", "info", "debug", "trace"];
    if !valid_levels.contains(&level.to_lowercase().as_str()) {
        return Err(format!("Invalid log level: {}", level));
    }

    let mut s = crate::settings::load_settings(&app);
    s.log_level = level.clone();
    crate::settings::save_settings(&app, &s)?;

    if let Ok(level_filter) = std::str::FromStr::from_str(&level) {
        log::set_max_level(level_filter);
    }

    Ok(())
}
