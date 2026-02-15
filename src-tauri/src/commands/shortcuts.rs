use crate::settings;
use crate::shortcuts::ShortcutState;
use crate::shortcuts::{keys_to_string, parse_binding_keys, ShortcutAction, ShortcutRegistryState};
use tauri::{command, AppHandle, Manager};

// ============================================================================
// Record Shortcut
// ============================================================================

#[command]
pub fn get_record_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.record_shortcut)
}

#[command]
pub fn set_record_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    let keys = parse_binding_keys(&binding);
    if keys.is_empty() {
        return Err("Invalid shortcut".to_string());
    }
    let normalized = keys_to_string(&keys);

    let mut s = settings::load_settings(&app);
    s.record_shortcut = normalized.clone();
    settings::save_settings(&app, &s)?;

    app.state::<ShortcutRegistryState>()
        .update_binding(ShortcutAction::StartRecording, keys);

    Ok(normalized)
}

// ============================================================================
// Last Transcript Shortcut
// ============================================================================

#[command]
pub fn get_last_transcript_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.last_transcript_shortcut)
}

#[command]
pub fn set_last_transcript_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    let keys = parse_binding_keys(&binding);
    if keys.is_empty() {
        return Err("Invalid shortcut".to_string());
    }
    let normalized = keys_to_string(&keys);

    let mut s = settings::load_settings(&app);
    s.last_transcript_shortcut = normalized.clone();
    settings::save_settings(&app, &s)?;

    app.state::<ShortcutRegistryState>()
        .update_binding(ShortcutAction::PasteLastTranscript, keys);

    Ok(normalized)
}

// ============================================================================
// LLM Record Shortcut
// ============================================================================

#[command]
pub fn get_llm_record_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.llm_record_shortcut)
}

#[command]
pub fn set_llm_record_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    if binding.is_empty() {
        return Err("Shortcut binding cannot be empty".to_string());
    }

    let keys = parse_binding_keys(&binding);
    if keys.is_empty() {
        return Err("Invalid shortcut".to_string());
    }
    let normalized = keys_to_string(&keys);

    let mut s = settings::load_settings(&app);
    s.llm_record_shortcut = normalized.clone();
    settings::save_settings(&app, &s)?;

    app.state::<ShortcutRegistryState>()
        .update_binding(ShortcutAction::StartRecordingLLM, keys);

    Ok(normalized)
}

// ============================================================================
// Command Shortcut
// ============================================================================

#[command]
pub fn get_command_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.command_shortcut)
}

#[command]
pub fn set_command_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    if binding.is_empty() {
        return Err("Shortcut binding cannot be empty".to_string());
    }

    let keys = parse_binding_keys(&binding);
    if keys.is_empty() {
        return Err("Invalid shortcut".to_string());
    }
    let normalized = keys_to_string(&keys);

    let mut s = settings::load_settings(&app);
    s.command_shortcut = normalized.clone();
    settings::save_settings(&app, &s)?;

    app.state::<ShortcutRegistryState>()
        .update_binding(ShortcutAction::StartRecordingCommand, keys);

    Ok(normalized)
}

// ============================================================================
// Suspend/Resume Transcription
// ============================================================================

#[command]
pub fn suspend_transcription(app_handle: AppHandle) {
    let state = app_handle.state::<ShortcutState>();
    state.set_suspended(true);
}

#[command]
pub fn resume_transcription(app_handle: AppHandle) {
    let state = app_handle.state::<ShortcutState>();
    state.set_suspended(false);
}

// ============================================================================
// LLM Mode Shortcuts (1-4)
// ============================================================================

#[command]
pub fn get_llm_mode_1_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.llm_mode_1_shortcut)
}

#[command]
pub fn set_llm_mode_1_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    set_llm_mode_shortcut(app, binding, 0, |s| &mut s.llm_mode_1_shortcut)
}

#[command]
pub fn get_llm_mode_2_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.llm_mode_2_shortcut)
}

#[command]
pub fn set_llm_mode_2_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    set_llm_mode_shortcut(app, binding, 1, |s| &mut s.llm_mode_2_shortcut)
}

#[command]
pub fn get_llm_mode_3_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.llm_mode_3_shortcut)
}

#[command]
pub fn set_llm_mode_3_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    set_llm_mode_shortcut(app, binding, 2, |s| &mut s.llm_mode_3_shortcut)
}

#[command]
pub fn get_llm_mode_4_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.llm_mode_4_shortcut)
}

#[command]
pub fn set_llm_mode_4_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    set_llm_mode_shortcut(app, binding, 3, |s| &mut s.llm_mode_4_shortcut)
}

fn set_llm_mode_shortcut<F>(
    app: AppHandle,
    binding: String,
    mode_index: usize,
    get_field: F,
) -> Result<String, String>
where
    F: Fn(&mut crate::settings::types::AppSettings) -> &mut String,
{
    if binding.is_empty() {
        return Err("Shortcut binding cannot be empty".to_string());
    }

    let keys = parse_binding_keys(&binding);
    if keys.is_empty() {
        return Err("Invalid shortcut".to_string());
    }
    let normalized = keys_to_string(&keys);

    let mut s = settings::load_settings(&app);
    *get_field(&mut s) = normalized.clone();
    settings::save_settings(&app, &s)?;

    app.state::<ShortcutRegistryState>()
        .update_binding(ShortcutAction::SwitchLLMMode(mode_index), keys);

    Ok(normalized)
}

// ============================================================================
// Cancel Recording Shortcut
// ============================================================================

#[command]
pub fn get_cancel_recording_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.cancel_recording_shortcut)
}

#[command]
pub fn set_cancel_recording_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    let keys = parse_binding_keys(&binding);
    if keys.is_empty() {
        return Err("Invalid shortcut".to_string());
    }
    let normalized = keys_to_string(&keys);

    let mut s = settings::load_settings(&app);
    s.cancel_recording_shortcut = normalized.clone();
    settings::save_settings(&app, &s)?;

    app.state::<ShortcutRegistryState>()
        .update_binding(ShortcutAction::CancelRecording, keys);

    Ok(normalized)
}

// ============================================================================
// Accessibility (macOS only)
// ============================================================================

#[cfg(target_os = "macos")]
#[command]
pub fn open_accessibility_settings() {
    crate::shortcuts::accessibility_macos::open_accessibility_settings();
}

#[cfg(target_os = "macos")]
#[command]
pub fn check_accessibility_permission() -> bool {
    crate::shortcuts::accessibility_macos::is_accessibility_enabled()
}

#[cfg(not(target_os = "macos"))]
#[command]
pub fn open_accessibility_settings() {
    // No-op on non-macOS platforms
}

#[cfg(not(target_os = "macos"))]
#[command]
pub fn check_accessibility_permission() -> bool {
    true // Always granted on non-macOS platforms
}
