use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager};

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

pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show_i = MenuItem::with_id(app, "show", "Open Murmure", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

    let builder = TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => restore_main_window(app),
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

    #[cfg(target_os = "linux")]
    let builder = builder
        .icon(app.default_window_icon().unwrap().clone())
        .show_menu_on_left_click(true)
        .icon_as_template(true);
    #[cfg(target_os = "windows")]
    let builder = builder.icon(app.default_window_icon().unwrap().clone());
    // On macOS, use a dedicated monochrome template icon so the menu bar
    // renders it as a template (adapts to Light/Dark mode and matte/full
    // menu bar styles), matching Apple HIG for status items.
    #[cfg(target_os = "macos")]
    let builder = {
        let tray_icon_bytes = include_bytes!("../../icons/tray-template.png");
        let tray_icon = tauri::image::Image::from_bytes(tray_icon_bytes)?;
        builder.icon(tray_icon).icon_as_template(true)
    };

    let _tray = builder.build(app)?;

    Ok(())
}
