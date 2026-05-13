use log::warn;
use tauri::image::Image;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager};

pub struct TrayIconState {
    pub icon: TrayIcon,
    pub idle_image: Image<'static>,
    pub recording_image: Image<'static>,
}

/// Show + focus the main window. On Linux, prepend `unminimize()`:
/// Mutter / KWin 6.4 keep the hidden-to-tray window flagged as
/// minimised, so `show()` alone leaves the webview frozen. Pattern
/// borrowed from cjpais/Handy.
fn restore_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        // Linux only: Mutter / KWin 6.4 keep the hidden-to-tray
        // window flagged as minimised; calling `show()` without a
        // prior `unminimize()` leaves the webview frozen.
        #[cfg(target_os = "linux")]
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn copy_last_transcript(app: &AppHandle) {
    let app = app.clone();
    std::thread::spawn(move || {
        let entries = match crate::history::get_recent_transcriptions(&app) {
            Ok(entries) => entries,
            Err(e) => {
                warn!("tray copy-last-transcript: history read failed ({})", e);
                return;
            }
        };
        let Some(entry) = entries.first() else {
            warn!("tray copy-last-transcript: history is empty");
            return;
        };
        if let Err(e) = crate::clipboard::copy_to_clipboard(&entry.text, &app) {
            warn!("tray copy-last-transcript: clipboard write failed ({})", e);
        }
    });
}

pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show_i = MenuItem::with_id(app, "show", "Open Murmure", true, None::<&str>)?;
    let copy_last_i = MenuItem::with_id(
        app,
        "copy_last_transcript",
        "Copy last transcript",
        true,
        None::<&str>,
    )?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_i, &copy_last_i, &quit_i])?;

    let recording_image =
        Image::from_bytes(include_bytes!("../../icons/tray-recording.png"))?.to_owned();

    #[cfg(target_os = "macos")]
    let idle_image = Image::from_bytes(include_bytes!("../../icons/tray-template.png"))?.to_owned();
    #[cfg(not(target_os = "macos"))]
    let idle_image = {
        let default_icon = app
            .default_window_icon()
            .ok_or("default_window_icon unavailable for tray idle")?;
        Image::new_owned(
            default_icon.rgba().to_vec(),
            default_icon.width(),
            default_icon.height(),
        )
    };

    let builder = TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => restore_main_window(app),
            "copy_last_transcript" => copy_last_transcript(app),
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: tauri::tray::MouseButton::Left,
                ..
            } = event
            {
                restore_main_window(tray.app_handle());
            }
        });

    let builder = builder.icon(idle_image.clone());
    #[cfg(target_os = "linux")]
    let builder = builder.show_menu_on_left_click(true);
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let builder = builder.icon_as_template(true);

    let tray = builder.build(app)?;

    app.manage(TrayIconState {
        icon: tray,
        idle_image,
        recording_image,
    });

    Ok(())
}

pub fn set_tray_recording(app: &AppHandle) {
    let Some(state) = app.try_state::<TrayIconState>() else {
        warn!("tray state not initialized");
        return;
    };
    if let Err(e) = state.icon.set_icon(Some(state.recording_image.clone())) {
        warn!("set_icon failed: {}", e);
    }
    // Template mode forces monochrome rendering and would destroy the red REC dot.
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        if let Err(e) = state.icon.set_icon_as_template(false) {
            warn!("set_icon_as_template(false) failed: {}", e);
        }
    }
}

pub fn set_tray_idle(app: &AppHandle) {
    let Some(state) = app.try_state::<TrayIconState>() else {
        warn!("tray state not initialized");
        return;
    };
    if let Err(e) = state.icon.set_icon(Some(state.idle_image.clone())) {
        warn!("set_icon failed: {}", e);
    }
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        if let Err(e) = state.icon.set_icon_as_template(true) {
            warn!("set_icon_as_template(true) failed: {}", e);
        }
    }
}
