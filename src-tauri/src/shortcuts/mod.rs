pub mod helpers;
pub mod registry;
pub mod shortcuts;
pub mod types;

#[cfg(target_os = "linux")]
mod platform_linux;

#[cfg(target_os = "windows")]
mod platform_windows;

#[cfg(target_os = "macos")]
mod platform_macos;

pub use helpers::{keys_to_string, parse_binding_keys};
pub use registry::ShortcutRegistryState;
pub use shortcuts::{force_stop_recording, handle_shortcut_event, init_shortcuts};
pub use types::{ActivationMode, ShortcutAction, ShortcutState};
