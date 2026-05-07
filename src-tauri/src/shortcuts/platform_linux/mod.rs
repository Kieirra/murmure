mod wayland;
mod x11;

use log::info;
use tauri::AppHandle;

use crate::utils::platform::is_wayland_session;

pub fn init(app: AppHandle, use_wayland_portal: bool) {
    if is_wayland_session() {
        if use_wayland_portal {
            info!("Wayland Portal mode: registering shortcuts via XDG GlobalShortcuts portal");
            wayland::init(app);
        } else {
            // CLI mode: user binds shortcuts at OS level, Murmure stays passive.
            info!("Wayland CLI mode: shortcuts bound at OS level, Murmure dispatches actions via CLI args");
        }
    } else {
        info!("X11 session: listening to keyboard events via rdev for shortcut matching");
        x11::init(app);
    }
}
