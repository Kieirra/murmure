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
    {
        if !murmure_lib::utils::platform::is_wayland() {
            std::env::set_var("GDK_BACKEND", "x11");
        }
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }

    murmure_lib::run()
}
