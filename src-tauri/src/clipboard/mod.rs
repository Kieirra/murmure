pub mod clipboard;

#[cfg(target_os = "linux")]
pub mod clipboard_wayland;

pub use clipboard::*;
