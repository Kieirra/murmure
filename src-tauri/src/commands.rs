use crate::dictionary::Dictionary;
use crate::history::{self, HistoryEntry};
use crate::http_api::spawn_http_api_thread;
use crate::http_api::HttpApiState;
use crate::model::Model;
use crate::settings;
use crate::settings::OnboardingState;
use crate::shortcuts::TranscriptionSuspended;
#[cfg(any(target_os = "linux", target_os = "windows"))]
use crate::shortcuts::{
    keys_to_string, parse_binding_keys, LastTranscriptShortcutKeys, RecordShortcutKeys,
};
use crate::stats::UsageStats;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager, State};
#[cfg(target_os = "macos")]
use crate::shortcuts::{register_last_transcript_shortcut, register_record_shortcut};
#[cfg(target_os = "macos")]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

#[tauri::command]
pub fn is_model_available(model: State<Arc<Model>>) -> bool {
    model.is_available()
}

#[tauri::command]
pub fn get_model_path(model: State<Arc<Model>>) -> Result<String, String> {
    let path = model.get_model_path().map_err(|e| format!("{:#}", e))?;

    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn get_recent_transcriptions(app: AppHandle) -> Result<Vec<HistoryEntry>, String> {
    history::get_recent_transcriptions(&app).map_err(|e| format!("{:#}", e))
}

#[tauri::command]
pub fn clear_history(app: AppHandle) -> Result<(), String> {
    history::clear_history(&app).map_err(|e| format!("{:#}", e))
}

#[tauri::command]
pub fn get_record_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.record_shortcut)
}

#[tauri::command]
pub fn set_record_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    return set_record_shortcut_linux_windows(app, binding);
    #[cfg(target_os = "macos")]
    return set_record_shortcut_macos(app, binding);
}

#[cfg(target_os = "macos")]
pub fn set_record_shortcut_macos(app: AppHandle, binding: String) -> Result<String, String> {
    if binding.is_empty() {
        return Err("Shortcut binding cannot be empty".to_string());
    }

    let mut s = settings::load_settings(&app);

    if let Ok(new_shortcut) = binding.parse::<Shortcut>() {
        // Step 1: Unregister the old shortcut handler
        if let Ok(old_shortcut) = s.record_shortcut.parse::<Shortcut>() {
            let _ = app.global_shortcut().unregister(old_shortcut);
        }

        // Step 2: Register the new shortcut with its handler
        register_record_shortcut(&app, new_shortcut)?;

        // Step 3: Save the new binding to settings
        s.record_shortcut = binding.clone();
        settings::save_settings(&app, &s)?;

        Ok(binding)
    } else {
        Err("Invalid shortcut".to_string())
    }
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub fn set_record_shortcut_linux_windows(
    app: AppHandle,
    binding: String,
) -> Result<String, String> {
    let keys = parse_binding_keys(&binding);
    if keys.is_empty() {
        return Err("Invalid shortcut".to_string());
    }
    let normalized = keys_to_string(&keys);

    let mut s = settings::load_settings(&app);
    s.record_shortcut = normalized.clone();
    settings::save_settings(&app, &s)?;

    app.state::<RecordShortcutKeys>().set(keys);

    Ok(normalized)
}

#[tauri::command]
pub fn set_dictionary(app: AppHandle, dictionary: Vec<String>) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    s.dictionary = dictionary.clone();
    if !s.onboarding.added_dictionary_word && !dictionary.is_empty() {
        s.onboarding.added_dictionary_word = true;
    }
    settings::save_settings(&app, &s)?;

    app.state::<Dictionary>().set(dictionary.clone());

    // Emit event so frontend can react (onboarding, UI refresh)
    let _ = app.emit("dictionary:updated", ());

    Ok(())
}

#[tauri::command]
pub fn get_dictionary(app: AppHandle) -> Result<Vec<String>, String> {
    let s = settings::load_settings(&app);
    Ok(s.dictionary)
}

#[tauri::command]
pub fn get_last_transcript_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.last_transcript_shortcut)
}

#[tauri::command]
pub fn set_last_transcript_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    return set_last_transcript_shortcut_linux_windows(app, binding);
    #[cfg(target_os = "macos")]
    return set_last_transcript_shortcut_macos(app, binding);
}

#[cfg(target_os = "macos")]
pub fn set_last_transcript_shortcut_macos(
    app: AppHandle,
    binding: String,
) -> Result<String, String> {
    if binding.is_empty() {
        return Err("Shortcut binding cannot be empty".to_string());
    }

    let mut s = settings::load_settings(&app);

    if let Ok(new_shortcut) = binding.parse::<Shortcut>() {
        // Step 1: Unregister the old shortcut handler
        if let Ok(old_shortcut) = s.last_transcript_shortcut.parse::<Shortcut>() {
            let _ = app.global_shortcut().unregister(old_shortcut);
        }

        // Step 2: Register the new shortcut with its handler
        register_last_transcript_shortcut(&app, new_shortcut)?;

        // Step 3: Save the new binding to settings
        s.last_transcript_shortcut = binding.clone();
        settings::save_settings(&app, &s)?;

        Ok(binding)
    } else {
        Err("Invalid shortcut".to_string())
    }
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub fn set_last_transcript_shortcut_linux_windows(
    app: AppHandle,
    binding: String,
) -> Result<String, String> {
    let keys = parse_binding_keys(&binding);
    if keys.is_empty() {
        return Err("Invalid shortcut".to_string());
    }
    let normalized = keys_to_string(&keys);

    let mut s = settings::load_settings(&app);
    s.last_transcript_shortcut = normalized.clone();
    settings::save_settings(&app, &s)?;

    app.state::<LastTranscriptShortcutKeys>().set(keys);

    Ok(normalized)
}

#[tauri::command]
pub fn suspend_transcription(_app: AppHandle) -> Result<(), String> {
    _app.state::<TranscriptionSuspended>().set(true);
    Ok(())
}

#[tauri::command]
pub fn resume_transcription(_app: AppHandle) -> Result<(), String> {
    _app.state::<TranscriptionSuspended>().set(false);
    Ok(())
}

#[tauri::command]
pub fn get_overlay_mode(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.overlay_mode)
}

#[tauri::command]
pub fn set_overlay_mode(app: AppHandle, mode: String) -> Result<(), String> {
    let allowed = ["hidden", "recording", "always"];
    if !allowed.contains(&mode.as_str()) {
        return Err("Invalid overlay mode".to_string());
    }
    let mut s = settings::load_settings(&app);
    s.overlay_mode = mode;
    let res = settings::save_settings(&app, &s);
    match s.overlay_mode.as_str() {
        "always" => {
            crate::overlay::show_recording_overlay(&app);
        }
        "hidden" | "recording" => {
            crate::overlay::hide_recording_overlay(&app);
        }
        _ => {}
    }
    res
}

#[tauri::command]
pub fn get_overlay_position(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.overlay_position)
}

#[tauri::command]
pub fn set_overlay_position(app: AppHandle, position: String) -> Result<(), String> {
    let allowed = ["top", "bottom"];
    if !allowed.contains(&position.as_str()) {
        return Err("Invalid overlay position".to_string());
    }
    let mut s = settings::load_settings(&app);
    s.overlay_position = position;
    let res = settings::save_settings(&app, &s);
    crate::overlay::update_overlay_position(&app);
    res
}

#[tauri::command]
pub fn get_api_enabled(app: AppHandle) -> Result<bool, String> {
    let s = settings::load_settings(&app);
    Ok(s.api_enabled)
}

#[tauri::command]
pub fn set_api_enabled(app: AppHandle, enabled: bool) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    s.api_enabled = enabled;
    settings::save_settings(&app, &s)
}

#[tauri::command]
pub fn get_api_port(app: AppHandle) -> Result<u16, String> {
    let s = settings::load_settings(&app);
    Ok(s.api_port)
}

#[tauri::command]
pub fn set_api_port(app: AppHandle, port: u16) -> Result<(), String> {
    if port < 1024 {
        return Err("Port must be >= 1024".to_string());
    }
    let mut s = settings::load_settings(&app);
    s.api_port = port;
    settings::save_settings(&app, &s)
}

#[tauri::command]
pub fn start_http_api_server(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    let port = s.api_port;
    let app_handle = app.clone();
    let state = app.state::<HttpApiState>().inner().clone();
    spawn_http_api_thread(app_handle, port, state);

    Ok(format!("HTTP API server starting on port {}", s.api_port))
}

#[tauri::command]
pub fn stop_http_api_server(app: AppHandle) -> Result<(), String> {
    let state = app.state::<HttpApiState>();
    state.stop();
    eprintln!("HTTP API server stop signal sent");
    Ok(())
}

#[tauri::command]
pub fn get_copy_to_clipboard(app: AppHandle) -> Result<bool, String> {
    let s = settings::load_settings(&app);
    Ok(s.copy_to_clipboard)
}

#[tauri::command]
pub fn set_copy_to_clipboard(app: AppHandle, enabled: bool) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    s.copy_to_clipboard = enabled;
    settings::save_settings(&app, &s)
}

#[tauri::command]
pub fn get_onboarding_state(app: AppHandle) -> Result<OnboardingState, String> {
    let s = settings::load_settings(&app);
    Ok(s.onboarding)
}

#[tauri::command]
pub fn set_onboarding_used_home_shortcut(app: AppHandle) -> Result<OnboardingState, String> {
    let mut s = settings::load_settings(&app);
    if !s.onboarding.used_home_shortcut {
        s.onboarding.used_home_shortcut = true;
        settings::save_settings(&app, &s)?;
    }
    Ok(s.onboarding)
}

#[tauri::command]
pub fn set_onboarding_transcribed_outside_app(app: AppHandle) -> Result<OnboardingState, String> {
    let mut s = settings::load_settings(&app);
    if !s.onboarding.transcribed_outside_app {
        s.onboarding.transcribed_outside_app = true;
        settings::save_settings(&app, &s)?;
    }
    Ok(s.onboarding)
}

#[tauri::command]
pub fn set_onboarding_added_dictionary_word(app: AppHandle) -> Result<OnboardingState, String> {
    let mut s = settings::load_settings(&app);
    if !s.onboarding.added_dictionary_word {
        s.onboarding.added_dictionary_word = true;
        settings::save_settings(&app, &s)?;
    }
    Ok(s.onboarding)
}

#[tauri::command]
pub fn get_usage_stats(app: AppHandle) -> Result<UsageStats, String> {
    crate::stats::compute_stats(&app).map_err(|e| format!("{:#}", e))
}

#[tauri::command]
pub fn get_persist_history(app: AppHandle) -> Result<bool, String> {
    let s = settings::load_settings(&app);
    Ok(s.persist_history)
}

#[tauri::command]
pub fn set_persist_history(app: AppHandle, enabled: bool) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    s.persist_history = enabled;
    settings::save_settings(&app, &s)?;
    if !enabled {
        let _ = history::clear_history(&app);
        let _ = history::purge_history_file(&app);
    }
    Ok(())
}

#[tauri::command]
pub fn get_current_language(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.language)
}

#[tauri::command]
pub fn set_current_language(app: AppHandle, lang: String) -> Result<(), String> {
    const SUPPORTED_LANGUAGES: &[&str] = &["default", "en", "fr"];

    if !SUPPORTED_LANGUAGES.contains(&lang.as_str()) {
        return Err(format!("Unsupported language code: {}", lang));
    }

    let mut s = settings::load_settings(&app);
    s.language = lang;
    settings::save_settings(&app, &s)
}
