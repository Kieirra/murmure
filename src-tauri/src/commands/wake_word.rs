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
    set_wake_word_field(
        &app,
        word,
        |s| {
            vec![
                &s.wake_word_command,
                &s.wake_word_cancel,
                &s.wake_word_validate,
            ]
        },
        |s, w| s.wake_word_record = w,
    )
}

#[command]
pub fn get_wake_word_command(app: AppHandle) -> Result<String, String> {
    let s = crate::settings::load_settings(&app);
    Ok(s.wake_word_command)
}

#[command]
pub fn set_wake_word_command(app: AppHandle, word: String) -> Result<(), String> {
    set_wake_word_field(
        &app,
        word,
        |s| {
            vec![
                &s.wake_word_record,
                &s.wake_word_cancel,
                &s.wake_word_validate,
            ]
        },
        |s, w| s.wake_word_command = w,
    )
}

#[command]
pub fn get_wake_word_cancel(app: AppHandle) -> Result<String, String> {
    let s = crate::settings::load_settings(&app);
    Ok(s.wake_word_cancel)
}

#[command]
pub fn set_wake_word_cancel(app: AppHandle, word: String) -> Result<(), String> {
    set_wake_word_field(
        &app,
        word,
        |s| {
            vec![
                &s.wake_word_record,
                &s.wake_word_command,
                &s.wake_word_validate,
            ]
        },
        |s, w| s.wake_word_cancel = w,
    )
}

#[command]
pub fn get_wake_word_validate(app: AppHandle) -> Result<String, String> {
    let s = crate::settings::load_settings(&app);
    Ok(s.wake_word_validate)
}

#[command]
pub fn set_wake_word_validate(app: AppHandle, word: String) -> Result<(), String> {
    set_wake_word_field(
        &app,
        word,
        |s| {
            vec![
                &s.wake_word_record,
                &s.wake_word_command,
                &s.wake_word_cancel,
            ]
        },
        |s, w| s.wake_word_validate = w,
    )
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

#[command]
pub fn get_llm_mode_wake_word(app: AppHandle, index: usize) -> Result<String, String> {
    let settings = crate::llm::helpers::load_llm_connect_settings(&app);
    settings
        .modes
        .get(index)
        .map(|m| m.wake_word.clone())
        .ok_or_else(|| format!("LLM mode {} not found", index))
}

#[command]
pub fn set_llm_mode_wake_word(app: AppHandle, index: usize, word: String) -> Result<(), String> {
    let trimmed = word.trim().to_string();
    if trimmed.len() > 50 {
        return Err("Wake word is too long (max 50 characters)".to_string());
    }

    let app_settings = crate::settings::load_settings(&app);
    let llm_settings = crate::llm::helpers::load_llm_connect_settings(&app);

    let mut all_others: Vec<String> = vec![
        app_settings.wake_word_record.clone(),
        app_settings.wake_word_command.clone(),
        app_settings.wake_word_cancel.clone(),
        app_settings.wake_word_validate.clone(),
    ];
    for (i, mode) in llm_settings.modes.iter().enumerate() {
        if i != index {
            all_others.push(mode.wake_word.clone());
        }
    }

    validate_wake_word_unique(&trimmed, &all_others)?;

    let mut llm_settings = llm_settings;
    match llm_settings.modes.get_mut(index) {
        Some(mode) => mode.wake_word = trimmed,
        None => return Err(format!("LLM mode {} not found", index)),
    }
    crate::llm::helpers::save_llm_connect_settings(&app, &llm_settings)
        .map_err(|e| format!("Failed to save settings: {}", e))?;

    restart_listener_if_active(&app, &app_settings);
    Ok(())
}

fn set_wake_word_field(
    app: &AppHandle,
    word: String,
    get_others: fn(&crate::settings::types::AppSettings) -> Vec<&str>,
    set_field: fn(&mut crate::settings::types::AppSettings, String),
) -> Result<(), String> {
    let trimmed = word.trim().to_string();
    if trimmed.len() > 50 {
        return Err("Wake word is too long (max 50 characters)".to_string());
    }
    let s = crate::settings::load_settings(app);
    let app_others = get_others(&s);

    let llm_settings = crate::llm::helpers::load_llm_connect_settings(app);
    let mut all_others: Vec<String> = app_others.iter().map(|o| o.to_string()).collect();
    for mode in &llm_settings.modes {
        all_others.push(mode.wake_word.clone());
    }

    validate_wake_word_unique(&trimmed, &all_others)?;

    let mut s = s;
    set_field(&mut s, trimmed);
    crate::settings::save_settings(app, &s)?;
    restart_listener_if_active(app, &s);
    Ok(())
}

fn validate_wake_word_unique(word: &str, others: &[String]) -> Result<(), String> {
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
