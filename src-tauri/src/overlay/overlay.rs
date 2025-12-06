use crate::settings;
use enigo::{Enigo, Mouse};
use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, WebviewWindowBuilder};

const OVERLAY_BASE_WIDTH: f64 = 80.0;
const OVERLAY_BASE_HEIGHT: f64 = 18.0;
const OVERLAY_TOP_OFFSET_PCT: f64 = 0.03;
const OVERLAY_BOTTOM_OFFSET_PCT: f64 = 0.03;

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
    get_cursor_monitor(app_handle).or_else(|| app_handle.primary_monitor().ok().flatten())
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

fn calculate_overlay_geometry(app_handle: &AppHandle) -> Option<(f64, f64, f64, f64)> {
    if let Some(monitor) = get_active_monitor(app_handle) {
        let work_area = monitor.work_area();
        let scale = monitor.scale_factor();
        let work_w = work_area.size.width as f64 / scale;
        let work_h = work_area.size.height as f64 / scale;
        let work_x = work_area.position.x as f64 / scale;
        let work_y = work_area.position.y as f64 / scale;

        let overlay_w = OVERLAY_BASE_WIDTH;
        let overlay_h = OVERLAY_BASE_HEIGHT;

        let x = work_x + (work_w - overlay_w) / 2.0;
        let s = settings::load_settings(app_handle);
        let y = match s.overlay_position.as_str() {
            "top" => work_y + work_h * OVERLAY_TOP_OFFSET_PCT,
            _ => work_y + work_h * (1.0 - OVERLAY_BOTTOM_OFFSET_PCT) - overlay_h,
        };
        return Some((x, y, overlay_w, overlay_h));
    }
    None
}

pub fn create_recording_overlay(app_handle: &AppHandle) {
    if let Some((x, y, w, h)) = calculate_overlay_geometry(app_handle) {
        let res = WebviewWindowBuilder::new(
            app_handle,
            "recording_overlay",
            tauri::WebviewUrl::App("src/overlay/index.html".into()),
        )
        .title("Recording")
        .position(x, y)
        .resizable(false)
        .inner_size(w, h)
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
        update_overlay_position(app_handle);
        let _ = window.show();
        let _ = window.set_ignore_cursor_events(true);
        let _ = window.emit("show-overlay", "recording");
    } else {
        println!("recording_overlay window not found on show_recording_overlay");
    }
}

pub fn update_overlay_position(app_handle: &AppHandle) {
    ensure_overlay(app_handle);
    if let Some((x, y, w, h)) = calculate_overlay_geometry(app_handle) {
        if let Some(window) = app_handle.get_webview_window("recording_overlay") {
            let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition { x, y }));
            let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize {
                width: w,
                height: h,
            }));
        }
    }
}

pub fn hide_recording_overlay(app_handle: &AppHandle) {
    if let Some(window) = app_handle.get_webview_window("recording_overlay") {
        let _ = window.emit("hide-overlay", ());
        let _ = window.hide();
    } else {
        println!("recording_overlay window not found on hide_recording_overlay");
    }
}
