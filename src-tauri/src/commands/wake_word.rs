use tauri::{command, AppHandle, Manager};

#[command]
pub fn get_wake_word_enabled(app: AppHandle) -> Result<bool, String> {
    let s = crate::settings::load_settings(&app);
    Ok(s.wake_word_enabled)
}

#[command]
pub fn set_wake_word_enabled(app: AppHandle, enabled: bool) -> Result<(), String> {
    let mut s = crate::settings::load_settings(&app);
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
pub fn get_wake_word_record(app: AppHandle) -> Result<String, String> {
    let s = crate::settings::load_settings(&app);
    Ok(s.wake_word_record)
}

#[command]
pub fn set_wake_word_record(app: AppHandle, word: String) -> Result<(), String> {
    let trimmed = word.trim().to_string();
    if trimmed.len() > 50 {
        return Err("Wake word is too long (max 50 characters)".to_string());
    }
    let s = crate::settings::load_settings(&app);
    validate_wake_word_unique(
        &trimmed,
        &[
            &s.wake_word_llm,
            &s.wake_word_command,
            &s.wake_word_cancel,
            &s.wake_word_validate,
        ],
    )?;

    let mut s = s;
    s.wake_word_record = trimmed;
    crate::settings::save_settings(&app, &s)?;
    restart_listener_if_active(&app, &s);
    Ok(())
}

#[command]
pub fn get_wake_word_llm(app: AppHandle) -> Result<String, String> {
    let s = crate::settings::load_settings(&app);
    Ok(s.wake_word_llm)
}

#[command]
pub fn set_wake_word_llm(app: AppHandle, word: String) -> Result<(), String> {
    let trimmed = word.trim().to_string();
    if trimmed.len() > 50 {
        return Err("Wake word is too long (max 50 characters)".to_string());
    }
    let s = crate::settings::load_settings(&app);
    validate_wake_word_unique(
        &trimmed,
        &[
            &s.wake_word_record,
            &s.wake_word_command,
            &s.wake_word_cancel,
            &s.wake_word_validate,
        ],
    )?;

    let mut s = s;
    s.wake_word_llm = trimmed;
    crate::settings::save_settings(&app, &s)?;
    restart_listener_if_active(&app, &s);
    Ok(())
}

#[command]
pub fn get_wake_word_command(app: AppHandle) -> Result<String, String> {
    let s = crate::settings::load_settings(&app);
    Ok(s.wake_word_command)
}

#[command]
pub fn set_wake_word_command(app: AppHandle, word: String) -> Result<(), String> {
    let trimmed = word.trim().to_string();
    if trimmed.len() > 50 {
        return Err("Wake word is too long (max 50 characters)".to_string());
    }
    let s = crate::settings::load_settings(&app);
    validate_wake_word_unique(
        &trimmed,
        &[
            &s.wake_word_record,
            &s.wake_word_llm,
            &s.wake_word_cancel,
            &s.wake_word_validate,
        ],
    )?;

    let mut s = s;
    s.wake_word_command = trimmed;
    crate::settings::save_settings(&app, &s)?;
    restart_listener_if_active(&app, &s);
    Ok(())
}

#[command]
pub fn get_wake_word_cancel(app: AppHandle) -> Result<String, String> {
    let s = crate::settings::load_settings(&app);
    Ok(s.wake_word_cancel)
}

#[command]
pub fn set_wake_word_cancel(app: AppHandle, word: String) -> Result<(), String> {
    let trimmed = word.trim().to_string();
    if trimmed.len() > 50 {
        return Err("Wake word is too long (max 50 characters)".to_string());
    }
    let s = crate::settings::load_settings(&app);
    validate_wake_word_unique(
        &trimmed,
        &[
            &s.wake_word_record,
            &s.wake_word_llm,
            &s.wake_word_command,
            &s.wake_word_validate,
        ],
    )?;

    let mut s = s;
    s.wake_word_cancel = trimmed;
    crate::settings::save_settings(&app, &s)?;
    restart_listener_if_active(&app, &s);
    Ok(())
}

#[command]
pub fn get_wake_word_validate(app: AppHandle) -> Result<String, String> {
    let s = crate::settings::load_settings(&app);
    Ok(s.wake_word_validate)
}

#[command]
pub fn set_wake_word_validate(app: AppHandle, word: String) -> Result<(), String> {
    let trimmed = word.trim().to_string();
    if trimmed.len() > 50 {
        return Err("Wake word is too long (max 50 characters)".to_string());
    }
    let s = crate::settings::load_settings(&app);
    validate_wake_word_unique(
        &trimmed,
        &[
            &s.wake_word_record,
            &s.wake_word_llm,
            &s.wake_word_command,
            &s.wake_word_cancel,
        ],
    )?;

    let mut s = s;
    s.wake_word_validate = trimmed;
    crate::settings::save_settings(&app, &s)?;
    restart_listener_if_active(&app, &s);
    Ok(())
}

#[command]
pub fn get_auto_enter_after_wake_word(app: AppHandle) -> Result<bool, String> {
    let s = crate::settings::load_settings(&app);
    Ok(s.auto_enter_after_wake_word)
}

#[command]
pub fn set_auto_enter_after_wake_word(app: AppHandle, enabled: bool) -> Result<(), String> {
    let mut s = crate::settings::load_settings(&app);
    s.auto_enter_after_wake_word = enabled;
    crate::settings::save_settings(&app, &s)?;
    Ok(())
}

fn validate_wake_word_unique(word: &str, others: &[&str]) -> Result<(), String> {
    if word.is_empty() {
        return Ok(());
    }
    let lower = word.to_lowercase();
    for other in others {
        if !other.is_empty() && other.to_lowercase() == lower {
            return Err("This trigger word is already used by another action".to_string());
        }
    }
    Ok(())
}

fn restart_listener_if_active(app: &AppHandle, settings: &crate::settings::types::AppSettings) {
    let state = app.state::<crate::wake_word::types::WakeWordState>();
    if state.is_active() || settings.wake_word_enabled {
        crate::wake_word::stop_listener(app);
        crate::wake_word::start_listener(app);
    }
}
