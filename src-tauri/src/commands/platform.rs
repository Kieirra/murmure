use tauri::command;

#[command]
pub fn get_is_wayland() -> bool {
    #[cfg(target_os = "linux")]
    {
        crate::utils::platform::is_wayland()
    }
    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}
