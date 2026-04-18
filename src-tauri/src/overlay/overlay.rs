use crate::formatting_rules::highlighter::HighlightRange;
use crate::settings;
use enigo::{Enigo, Mouse};
use log::{debug, error, warn};
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, WebviewWindowBuilder};

#[derive(Serialize)]
struct EmptyStreamingTranscript {
    text: String,
    highlights: Vec<HighlightRange>,
}

const OVERLAY_HEIGHT: f64 = 36.0;
const OVERLAY_WIDTH: f64 = 350.0;
const OVERLAY_TOP_OFFSET_PCT: f64 = 0.05;
const OVERLAY_BOTTOM_OFFSET_PCT: f64 = 0.05;

fn get_cursor_monitor(app_handle: &AppHandle) -> Option<tauri::Monitor> {
    let enigo = match Enigo::new(&Default::default()) {
        Ok(e) => e,
        Err(_) => return None,
    };
    let mouse_location = match enigo.location() {
        Ok(loc) => loc,
        Err(_) => return None,
    };
    let monitors = match app_handle.available_monitors() {
        Ok(m) => m,
        Err(_) => return None,
    };
    monitors
        .into_iter()
        .find(|monitor| is_mouse_within_monitor(mouse_location, monitor.position(), monitor.size()))
}

fn get_active_monitor(app_handle: &AppHandle) -> Option<tauri::Monitor> {
    get_cursor_monitor(app_handle)
        .or_else(|| app_handle.primary_monitor().ok().flatten())
        .or_else(|| {
            warn!("No cursor or primary monitor found, using first available monitor");
            app_handle
                .available_monitors()
                .ok()
                .and_then(|m| m.into_iter().next())
        })
}

fn is_mouse_within_monitor(
    mouse_pos: (i32, i32),
    monitor_pos: &PhysicalPosition<i32>,
    monitor_size: &PhysicalSize<u32>,
) -> bool {
    let (mouse_x, mouse_y) = mouse_pos;
    let PhysicalPosition {
        x: monitor_x,
        y: monitor_y,
    } = *monitor_pos;
    let PhysicalSize {
        width: monitor_width,
        height: monitor_height,
    } = *monitor_size;
    mouse_x >= monitor_x
        && mouse_x < (monitor_x + monitor_width as i32)
        && mouse_y >= monitor_y
        && mouse_y < (monitor_y + monitor_height as i32)
}

fn calculate_overlay_geometry(app_handle: &AppHandle) -> Option<(i32, i32, u32, u32)> {
    if let Some(monitor) = get_active_monitor(app_handle) {
        let monitor_size = monitor.size();
        let monitor_pos = monitor.position();
        let scale = monitor.scale_factor();

        let work_w = monitor_size.width as f64;
        let work_h = monitor_size.height as f64;
        let work_x = monitor_pos.x as f64;
        let work_y = monitor_pos.y as f64;

        let s = settings::load_settings(app_handle);
        let overlay_w = OVERLAY_WIDTH.max(s.streaming_text_width as f64) * scale;
        let overlay_h = OVERLAY_HEIGHT * scale;

        let x = work_x + (work_w - overlay_w) / 2.0;
        let y = match s.overlay_position.as_str() {
            "top" => work_y + work_h * OVERLAY_TOP_OFFSET_PCT,
            _ => work_y + work_h * (1.0 - OVERLAY_BOTTOM_OFFSET_PCT) - overlay_h,
        };
        return Some((x as i32, y as i32, overlay_w as u32, overlay_h as u32));
    }
    None
}

pub fn create_recording_overlay(app_handle: &AppHandle) {
    let Some((x, y, w, h)) = calculate_overlay_geometry(app_handle) else {
        warn!("Could not determine overlay geometry (no monitor found), skipping overlay creation");
        return;
    };
    let res = WebviewWindowBuilder::new(
        app_handle,
        "recording_overlay",
        tauri::WebviewUrl::App("src/overlay/index.html".into()),
    )
    .title("Recording")
    .resizable(false)
    .shadow(false)
    .maximizable(false)
    .minimizable(false)
    .closable(false)
    .accept_first_mouse(false)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .transparent(true)
    .focused(false)
    .visible(false)
    .build();
    match res {
        Ok(window) => {
            let _ =
                window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x, y }));
            let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                width: w,
                height: h,
            }));
            debug!("Recording overlay window created (hidden)");
        }
        Err(e) => {
            error!("Failed to create recording overlay window: {}", e);
        }
    }
}

fn ensure_overlay(app_handle: &AppHandle) {
    if app_handle.get_webview_window("recording_overlay").is_none() {
        create_recording_overlay(app_handle);
    }
}

fn present_recording_overlay(app_handle: &AppHandle) {
    update_overlay_position(app_handle);
    let Some(window) = app_handle.get_webview_window("recording_overlay") else {
        warn!("recording_overlay window not found on present_recording_overlay");
        return;
    };
    let state = app_handle.state::<crate::audio::types::AudioState>();
    let mode_str = match state.get_recording_mode() {
        crate::audio::types::RecordingMode::Standard => "standard",
        crate::audio::types::RecordingMode::Llm => "llm",
        crate::audio::types::RecordingMode::Command => "command",
    };
    let _ = window.emit("recording-mode", mode_str);
    let _ = window.emit(
        "streaming-transcript",
        &EmptyStreamingTranscript {
            text: String::new(),
            highlights: vec![],
        },
    );
    let _ = window.show();
    let _ = window.set_always_on_top(true);
    let _ = window.set_ignore_cursor_events(true);
}

pub fn show_recording_overlay(app_handle: &AppHandle) {
    // Only destroy the overlay if it was actively visible (used in a previous session).
    // This preserves the startup-created hidden overlay for instant first display,
    // avoiding a blank window while the new WebView loads.
    if let Some(window) = app_handle.get_webview_window("recording_overlay") {
        if window.is_visible().unwrap_or(false) {
            if let Err(e) = window.destroy() {
                warn!("recording_overlay destroy before show failed: {}", e);
            }
        }
    }

    // Always dispatch to main thread to avoid GTK threading assertions (SIGABRT)
    // when called from the shortcut handler thread.
    let app_for_thread = app_handle.clone();
    std::thread::spawn(move || {
        let app_for_main = app_for_thread.clone();
        if let Err(e) = app_for_thread.run_on_main_thread(move || {
            present_recording_overlay(&app_for_main);
        }) {
            error!("recording_overlay show scheduling failed: {}", e);
        }
    });
}

pub fn update_overlay_position(app_handle: &AppHandle) {
    ensure_overlay(app_handle);
    if let Some((x, y, w, h)) = calculate_overlay_geometry(app_handle) {
        if let Some(window) = app_handle.get_webview_window("recording_overlay") {
            let _ =
                window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x, y }));
            let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                width: w,
                height: h,
            }));
        }
    }
}

pub fn hide_recording_overlay(app_handle: &AppHandle) {
    if let Some(window) = app_handle.get_webview_window("recording_overlay") {
        if let Err(e) = window.destroy() {
            warn!("recording_overlay destroy on hide failed: {}", e);
        }
    } else {
        debug!("recording_overlay already absent on hide_recording_overlay");
    }
}

pub fn resize_overlay_for_streaming(app_handle: &AppHandle, lines_count: u32) {
    let app = app_handle.clone();
    let _ = app_handle.run_on_main_thread(move || {
        if let Some(monitor) = get_active_monitor(&app) {
            let scale = monitor.scale_factor();
            let s = settings::load_settings(&app);
            let line_height = s.streaming_font_size as f64 * 1.6 + 4.0;
            let h = ((OVERLAY_HEIGHT + line_height * lines_count as f64) * scale) as u32;

            if let Some(window) = app.get_webview_window("recording_overlay") {
                if let Ok(current_size) = window.outer_size() {
                    let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                        width: current_size.width,
                        height: h,
                    }));
                }
            }
        }
    });
}

pub fn reset_overlay_size(app_handle: &AppHandle) {
    let app = app_handle.clone();
    let _ = app_handle.run_on_main_thread(move || {
        if let Some(monitor) = get_active_monitor(&app) {
            let scale = monitor.scale_factor();
            let s = settings::load_settings(&app);
            let w = (OVERLAY_WIDTH.max(s.streaming_text_width as f64) * scale) as u32;
            let h = (OVERLAY_HEIGHT * scale) as u32;

            if let Some(window) = app.get_webview_window("recording_overlay") {
                let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                    width: w,
                    height: h,
                }));
            }
        }
    });
}
