use crate::audio::types::AudioState;
use crate::overlay::overlay::{FinalResultPayload, PendingFlashState};
use crate::settings;
use serde::Serialize;
use tauri::{command, AppHandle, Emitter, Manager};

#[command]
pub fn get_recording_mode(app: AppHandle) -> String {
    let state = app.state::<AudioState>();
    state.get_recording_mode().as_str().to_string()
}

#[command]
pub fn get_active_llm_prompt_name(app: AppHandle) -> Option<String> {
    crate::llm::active_prompt_name(&app)
}

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

#[command]
pub fn set_tray_menu_labels(
    app: AppHandle,
    show: String,
    copy_last_transcript: String,
    quit: String,
) -> Result<(), String> {
    crate::overlay::tray::set_tray_menu_labels(&app, &show, &copy_last_transcript, &quit)
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

#[command]
pub fn set_overlay_input_region(
    app: AppHandle,
    rects: Vec<crate::overlay::input_region::InputRect>,
) -> Result<(), String> {
    let Some(window) = app.get_webview_window("recording_overlay") else {
        return Ok(());
    };
    app.run_on_main_thread(move || {
        crate::overlay::input_region::apply_input_region(&window, &rects);
    })
    .map_err(|e| e.to_string())
}

#[command]
pub fn consume_pending_mode_flash(state: tauri::State<PendingFlashState>) -> Option<String> {
    state.0.lock().take()
}

#[command]
pub fn consume_pending_result() -> Option<FinalResultPayload> {
    crate::overlay::overlay::take_pending_result()
}

// Sent by the overlay webview as soon as it renders the card: past this point
// the React timer owns the lifetime, including the pause on hover, and the
// watchdog must no longer hide anything.
#[command]
pub fn ack_result_panel_shown() -> Result<(), String> {
    crate::overlay::overlay::acknowledge_result_panel_shown();
    Ok(())
}

#[command]
pub fn flash_text_in_overlay(app: AppHandle, text: String) {
    crate::overlay::overlay::flash_text_in_overlay_internal(&app, text);
}

#[command]
pub fn hide_overlay_if_idle(app: AppHandle) -> Result<(), String> {
    crate::overlay::overlay::hide_overlay_if_idle(&app);
    Ok(())
}
