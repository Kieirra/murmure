#[tauri::command]
pub fn is_wayland_session() -> bool {
    #[cfg(target_os = "linux")]
    {
        crate::utils::wayland::is_wayland_session()
    }
    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}
