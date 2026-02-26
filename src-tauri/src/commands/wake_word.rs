use tauri::{command, AppHandle, Manager};

#[command]
pub fn get_wake_word_enabled(app: AppHandle) -> Result<bool, String> {
    let s = crate::settings::load_settings(&app);
    Ok(s.wake_word_enabled)
}

#[command]
pub fn set_wake_word_enabled(app: AppHandle, enabled: bool) -> Result<(), String> {
    let mut s = crate::settings::load_settings(&app);

    if enabled && s.wake_word.trim().is_empty() {
        return Err("Wake word cannot be empty".to_string());
    }

    s.wake_word_enabled = enabled;
    crate::settings::save_settings(&app, &s)?;

    if enabled {
        crate::wake_word::start_listener(&app);
    } else {
        crate::wake_word::stop_listener(&app);
    }

    Ok(())
}

#[command]
pub fn get_wake_word(app: AppHandle) -> Result<String, String> {
    let s = crate::settings::load_settings(&app);
    Ok(s.wake_word)
}

#[command]
pub fn set_wake_word(app: AppHandle, word: String) -> Result<(), String> {
    let trimmed = word.trim().to_string();
    if trimmed.is_empty() {
        return Err("Wake word cannot be empty".to_string());
    }
    if trimmed.len() > 50 {
        return Err("Wake word is too long (max 50 characters)".to_string());
    }

    let mut s = crate::settings::load_settings(&app);
    s.wake_word = trimmed;
    crate::settings::save_settings(&app, &s)?;

    let state = app.state::<crate::wake_word::types::WakeWordState>();
    if state.is_active() || s.wake_word_enabled {
        crate::wake_word::stop_listener(&app);
        crate::wake_word::start_listener(&app);
    }

    Ok(())
}
