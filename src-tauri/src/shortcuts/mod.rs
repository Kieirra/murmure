mod transaction_suspended;
pub use transaction_suspended::TranscriptionSuspended;

#[cfg(any(target_os = "linux", target_os = "windows"))]
mod lib_windows_linux;
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub use lib_windows_linux::{
    initialize_shortcut_states, keys_to_string, parse_binding_keys, LastTranscriptShortcutKeys,
    LLMRecordShortcutKeys, RecordShortcutKeys,
};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::init_shortcuts;
#[cfg(target_os = "macos")]
pub use macos::{
    init_shortcuts, register_last_transcript_shortcut, register_llm_record_shortcut,
    register_record_shortcut,
};
#[cfg(target_os = "windows")]
pub use windows::init_shortcuts;
