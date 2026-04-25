// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};
        unsafe {
            AttachConsole(ATTACH_PARENT_PROCESS);
        }
    }

    if murmure_lib::cli::try_handle_early_args() {
        return;
    }

    #[cfg(target_os = "linux")]
    setup_linux_env();

    murmure_lib::run()
}

#[cfg(target_os = "linux")]
fn setup_linux_env() {
    // Blank window on hybrid GPUs (#294).
    if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none() {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }
    // Must match the installed `.desktop` basename. Set before any
    // Tauri thread spawns. `std::env::set_var` is not thread-safe.
    if std::env::var_os("GLOBAL_HOTKEY_APP_ID").is_none() {
        std::env::set_var("GLOBAL_HOTKEY_APP_ID", "murmure");
    }
}
