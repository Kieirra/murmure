pub mod actions;
pub mod helpers;
pub mod shortcuts;
pub mod types;

#[cfg(any(target_os = "linux", target_os = "windows"))]
mod rdev_common;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub use helpers::*;
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub use shortcuts::*;
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub use types::{LLMRecordShortcutKeys, LastTranscriptShortcutKeys, RecordShortcutKeys};

#[cfg(target_os = "linux")]
pub use linux::init_shortcuts;
#[cfg(target_os = "macos")]
pub use macos::{
    init_shortcuts, register_last_transcript_shortcut, register_llm_record_shortcut,
    register_record_shortcut,
};
#[cfg(target_os = "windows")]
pub use windows::init_shortcuts;
