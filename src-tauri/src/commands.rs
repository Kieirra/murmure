use crate::dictionary::Dictionary;
use crate::history::{self, HistoryEntry};
use crate::model::Model;
use crate::settings;
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};
use crate::http_api::HttpApiState;
use crate::http_api::spawn_http_api_thread;

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
    if binding.is_empty() {
        return Err("Shortcut binding cannot be empty".to_string());
    }

    // Save new binding to settings
    // Note: Dynamic shortcut changes require app restart to take effect with tauri-plugin-global-shortcut
    let mut s = settings::load_settings(&app);
    s.record_shortcut = binding.clone();
    settings::save_settings(&app, &s)?;

    Ok(binding)
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
    if binding.is_empty() {
        return Err("Shortcut binding cannot be empty".to_string());
    }

    // Save new binding to settings
    // Note: Dynamic shortcut changes require app restart to take effect with tauri-plugin-global-shortcut
    let mut s = settings::load_settings(&app);
    s.last_transcript_shortcut = binding.clone();
    settings::save_settings(&app, &s)?;

    Ok(binding)
}

#[tauri::command]
pub fn suspend_transcription(_app: AppHandle) -> Result<(), String> {
    // Note: With tauri-plugin-global-shortcut, we can suspend by unregistering shortcuts if needed
    // For now, this is a placeholder for future enhancement
    Ok(())
}

#[tauri::command]
pub fn resume_transcription(_app: AppHandle) -> Result<(), String> {
    // Note: With tauri-plugin-global-shortcut, we can resume by re-registering shortcuts if needed
    // For now, this is a placeholder for future enhancement
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
