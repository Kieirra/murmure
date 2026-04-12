use crate::settings;
use serde::Serialize;
use tauri::{command, AppHandle, Emitter, Manager};

#[command]
pub fn get_streaming_preview(app: AppHandle) -> Result<bool, String> {
    let s = settings::load_settings(&app);
    Ok(s.streaming_preview)
}

#[command]
pub fn set_streaming_preview(app: AppHandle, enabled: bool) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    s.streaming_preview = enabled;
    settings::save_settings(&app, &s)
}

#[command]
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
            crate::overlay::overlay::show_recording_overlay(&app);
        }
        "hidden" | "recording" => {
            crate::overlay::overlay::hide_recording_overlay(&app);
        }
        _ => {}
    }
    res
}

#[command]
pub fn set_overlay_size(app: AppHandle, size: String) -> Result<(), String> {
    let allowed = ["small", "medium", "large"];
    if !allowed.contains(&size.as_str()) {
        return Err("Invalid overlay size".to_string());
    }
    let mut s = settings::load_settings(&app);
    s.overlay_size = size.clone();
    let res = settings::save_settings(&app, &s);
    crate::overlay::overlay::update_overlay_position(&app);
    if let Some(window) = app.get_webview_window("recording_overlay") {
        let _ = window.emit("overlay-size-changed", &size);
    }
    res
}

#[command]
pub fn set_overlay_position(app: AppHandle, position: String) -> Result<(), String> {
    let allowed = ["top", "bottom"];
    if !allowed.contains(&position.as_str()) {
        return Err("Invalid overlay position".to_string());
    }
    let mut s = settings::load_settings(&app);
    s.overlay_position = position;
    let res = settings::save_settings(&app, &s);
    crate::overlay::overlay::update_overlay_position(&app);
    res
}

#[derive(Serialize, Clone)]
struct StreamingTextSettings {
    text_width: u32,
    font_size: u32,
    max_lines: u32,
}

#[command]
pub fn set_streaming_text_settings(
    app: AppHandle,
    text_width: u32,
    font_size: u32,
    max_lines: u32,
) -> Result<(), String> {
    if !(200..=600).contains(&text_width) {
        return Err("text_width must be between 200 and 600".to_string());
    }
    if !(8..=18).contains(&font_size) {
        return Err("font_size must be between 8 and 18".to_string());
    }
    if !(1..=8).contains(&max_lines) {
        return Err("max_lines must be between 1 and 8".to_string());
    }

    let mut s = settings::load_settings(&app);
    s.streaming_text_width = text_width;
    s.streaming_font_size = font_size;
    s.streaming_max_lines = max_lines;
    let res = settings::save_settings(&app, &s);

    crate::overlay::overlay::update_overlay_position(&app);

    if let Some(window) = app.get_webview_window("recording_overlay") {
        let payload = StreamingTextSettings {
            text_width,
            font_size,
            max_lines,
        };
        let _ = window.emit("streaming-text-settings-changed", &payload);
    }

    res
}
