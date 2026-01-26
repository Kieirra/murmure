pub mod helpers;
pub mod shortcuts;
pub mod types;

#[cfg(any(target_os = "linux", target_os = "windows"))]
mod platform_rdev;

#[cfg(target_os = "macos")]
mod platform_macos;

pub use helpers::{keys_to_string, parse_binding_keys};
pub use shortcuts::{check_release_stop, execute_action, force_stop_recording, init_shortcuts};
pub use types::{ActivationMode, ShortcutAction, ShortcutRegistryState, ShortcutState};
