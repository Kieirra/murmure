use crate::dictionary::Dictionary;
use crate::history::{self, HistoryEntry};
use crate::model::Model;
use crate::settings;
use crate::shortcuts::{
    keys_to_string, parse_binding_keys, LastTranscriptShortcutKeys, RecordShortcutKeys, TranscriptionSuspended,
};
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};
use crate::LinuxFormatState;

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
pub fn get_record_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.record_shortcut)
}

#[tauri::command]
pub fn set_record_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
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
    settings::save_settings(&app, &s)?;

    app.state::<Dictionary>().set(dictionary.clone());

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
pub fn suspend_transcription(app: AppHandle) -> Result<(), String> {
    app.state::<TranscriptionSuspended>().set(true);
    Ok(())
}

#[tauri::command]
pub fn resume_transcription(app: AppHandle) -> Result<(), String> {
    app.state::<TranscriptionSuspended>().set(false);
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
pub fn get_linux_format(state: State<LinuxFormatState>) -> String {
    state.format.to_target_string()
}
