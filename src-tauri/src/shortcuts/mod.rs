pub mod helpers;
pub mod shortcuts;
pub mod types;

#[allow(dead_code)]
mod legacy;

#[cfg(target_os = "macos")]
pub mod macos;

// Public exports
pub use helpers::{keys_to_string, parse_binding_keys};
pub use shortcuts::{force_stop_recording, init_shortcuts};
pub use types::{ActivationMode, ShortcutAction, ShortcutRegistryState, ShortcutState};

#[cfg(target_os = "macos")]
pub use macos::{
    register_command_shortcut, register_last_transcript_shortcut, register_llm_record_shortcut,
    register_mode_switch_shortcut, register_record_shortcut,
};
