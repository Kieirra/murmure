use crate::settings;
use tauri::{AppHandle, Emitter, Manager, WebviewWindowBuilder};

const OVERLAY_WIDTH: f64 = 80.0;
const OVERLAY_HEIGHT: f64 = 18.0;
const OVERLAY_TOP_OFFSET_PCT: f64 = 0.03;
const OVERLAY_BOTTOM_OFFSET_PCT: f64 = 0.03;

fn get_primary_monitor(app_handle: &AppHandle) -> Option<tauri::Monitor> {
    app_handle.primary_monitor().ok().flatten()
}

fn calculate_overlay_position(app_handle: &AppHandle) -> Option<(f64, f64)> {
    if let Some(monitor) = get_primary_monitor(app_handle) {
        let work_area = monitor.work_area();
        let scale = monitor.scale_factor();
        let work_w = work_area.size.width as f64 / scale;
        let work_h = work_area.size.height as f64 / scale;
        let work_x = work_area.position.x as f64 / scale;
        let work_y = work_area.position.y as f64 / scale;

        let x = work_x + (work_w - OVERLAY_WIDTH) / 2.0;
        let s = settings::load_settings(app_handle);
        let y = match s.overlay_position.as_str() {
            "top" => work_y + work_h * OVERLAY_TOP_OFFSET_PCT,
            _ => work_y + work_h * (1.0 - OVERLAY_BOTTOM_OFFSET_PCT) - OVERLAY_HEIGHT,
        };
        return Some((x, y));
    }
    None
}

pub fn create_recording_overlay(app_handle: &AppHandle) {
    if let Some((x, y)) = calculate_overlay_position(app_handle) {
        let res = WebviewWindowBuilder::new(
            app_handle,
            "recording_overlay",
            tauri::WebviewUrl::App("src/overlay/index.html".into()),
        )
        .title("Recording")
        .position(x, y)
        .resizable(false)
        .inner_size(OVERLAY_WIDTH, OVERLAY_HEIGHT)
        .shadow(false)
        .maximizable(false)
        .minimizable(false)
        .closable(false)
        .accept_first_mouse(true)
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .transparent(true)
        .focused(false)
        .visible(false)
        .build();
        if let Err(e) = res {
            println!("Failed to create recording overlay window: {}", e);
        } else {
            println!("Recording overlay window created (hidden)");
        }
    }
}

fn ensure_overlay(app_handle: &AppHandle) {
    if app_handle.get_webview_window("recording_overlay").is_none() {
        create_recording_overlay(app_handle);
    }
}

pub fn show_recording_overlay(app_handle: &AppHandle) {
    ensure_overlay(app_handle);
    if let Some(window) = app_handle.get_webview_window("recording_overlay") {
        let _ = window.show();
        let _ = window.set_ignore_cursor_events(true);
        let _ = window.emit("show-overlay", "recording");
    } else {
        println!("recording_overlay window not found on show_recording_overlay");
    }
}

// pub fn show_transcribing_overlay(app_handle: &AppHandle) {
//     ensure_overlay(app_handle);
//     if let Some(window) = app_handle.get_webview_window("recording_overlay") {
//         let _ = window.show();
//         let _ = window.emit("show-overlay", "transcribing");
//     } else {
//         println!("recording_overlay window not found on show_transcribing_overlay");
//     }
// }

pub fn update_overlay_position(app_handle: &AppHandle) {
    ensure_overlay(app_handle);
    if let Some((x, y)) = calculate_overlay_position(app_handle) {
        if let Some(window) = app_handle.get_webview_window("recording_overlay") {
            let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition { x, y }));
        }
    }
}

pub fn hide_recording_overlay(app_handle: &AppHandle) {
    if let Some(window) = app_handle.get_webview_window("recording_overlay") {
        let _ = window.emit("hide-overlay", ());
        let win_clone = window.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(300));
            let _ = win_clone.hide();
        });
    } else {
        println!("recording_overlay window not found on hide_recording_overlay");
    }
}
