use crate::settings::AppSettings;
use tauri::{command, AppHandle};

#[command]
pub fn get_all_settings(app: AppHandle) -> Result<AppSettings, String> {
    Ok(crate::settings::load_settings(&app))
}

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

    crate::settings::update_settings(&app, |s| {
        s.language = lang;
        Ok(())
    })
}

#[command]
pub fn get_current_mic_id(app: AppHandle) -> Result<Option<String>, String> {
    let s = crate::settings::load_settings(&app);
    Ok(s.mic_id)
}

#[command]
pub fn set_current_mic_id(
    app: AppHandle,
    mic_id: Option<String>,
    mic_label: Option<String>,
) -> Result<(), String> {
    crate::settings::update_settings(&app, |s| {
        s.mic_id = mic_id.clone();
        s.mic_label = mic_label;
        Ok(())
    })?;
    crate::audio::microphone::update_mic_cache(&app, mic_id);
    Ok(())
}

#[command]
pub fn get_current_mic_label(app: AppHandle) -> Result<Option<String>, String> {
    let s = crate::settings::load_settings(&app);
    Ok(s.mic_label)
}

#[command]
pub fn get_mic_list() -> Result<Vec<crate::audio::types::MicInfo>, String> {
    let mic_list = crate::audio::microphone::get_mic_list();
    Ok(mic_list)
}

#[command]
pub fn set_sound_enabled(app: AppHandle, enabled: bool) -> Result<(), String> {
    crate::settings::update_settings(&app, |s| {
        s.sound_enabled = enabled;
        Ok(())
    })
}

#[command]
pub fn set_sound_volume(app: AppHandle, percent: u8) -> Result<(), String> {
    crate::settings::update_settings(&app, |s| {
        s.sound_volume = percent.clamp(
            crate::audio::sound::MIN_SOUND_VOLUME_PERCENT,
            crate::audio::sound::MAX_SOUND_VOLUME_PERCENT,
        );
        Ok(())
    })
}

#[command]
pub fn set_lower_output_while_recording(app: AppHandle, enabled: bool) -> Result<(), String> {
    crate::settings::update_settings(&app, |s| {
        s.lower_output_while_recording = enabled;
        Ok(())
    })
}

#[command]
pub fn set_output_volume_while_recording(app: AppHandle, percent: u8) -> Result<(), String> {
    crate::settings::update_settings(&app, |s| {
        s.output_volume_while_recording =
            percent.min(crate::audio::output_volume::MAX_LOWERED_PERCENT);
        Ok(())
    })
}

#[command]
pub fn set_keep_recordings(app: AppHandle, enabled: bool) -> Result<(), String> {
    crate::settings::update_settings(&app, |s| {
        s.keep_recordings = enabled;
        Ok(())
    })
}

#[command]
pub fn set_remove_hesitations(app: AppHandle, enabled: bool) -> Result<(), String> {
    crate::settings::update_settings(&app, |s| {
        s.remove_hesitations = enabled;
        Ok(())
    })
}

#[command]
pub fn get_recordings_dir(app: AppHandle) -> Result<String, String> {
    crate::audio::helpers::ensure_recordings_dir(&app)
        .map(|dir| dir.to_string_lossy().into_owned())
        .map_err(|e| e.to_string())
}

#[command]
pub fn set_log_level(app: AppHandle, level: String) -> Result<(), String> {
    let valid_levels = ["off", "error", "warn", "info", "debug", "trace"];
    if !valid_levels.contains(&level.to_lowercase().as_str()) {
        return Err(format!("Invalid log level: {}", level));
    }

    crate::settings::update_settings(&app, |s| {
        s.log_level = level.clone();
        Ok(())
    })?;

    if let Ok(level_filter) = std::str::FromStr::from_str(&level) {
        log::set_max_level(level_filter);
    }

    Ok(())
}

#[command]
pub fn set_show_in_dock(app: AppHandle, show: bool) -> Result<(), String> {
    crate::settings::update_settings(&app, |s| {
        s.show_in_dock = show;
        Ok(())
    })
}
