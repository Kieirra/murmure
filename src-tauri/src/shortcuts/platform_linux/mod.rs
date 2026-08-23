mod x11;

use log::info;
use tauri::AppHandle;

use crate::utils::platform::is_wayland_session;

pub(crate) fn any_modifier_held() -> bool {
    !is_wayland_session() && x11::any_modifier_held()
}

pub fn init(app: AppHandle) {
    if is_wayland_session() {
        // CLI mode: user binds shortcuts at OS level, Murmure stays passive.
        info!("Wayland CLI mode: shortcuts bound at OS level, Murmure dispatches actions via CLI args");
    } else {
        info!("X11 session: listening to keyboard events via rdev for shortcut matching");
        x11::init(app);
    }
}
