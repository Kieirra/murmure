mod wayland;
mod x11;

use tauri::AppHandle;

use crate::utils::platform::{get_linux_session_type, LinuxSessionType};

pub fn init(app: AppHandle) {
    match get_linux_session_type() {
        Some(LinuxSessionType::Wayland) => wayland::init(app),
        _ => x11::init(app),
    }
}
