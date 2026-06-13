use crate::settings;
use crate::shortcuts::types::{recording_state, RecordingSource};
use crate::shortcuts::ShortcutState;
use crate::shortcuts::{keys_to_string, parse_binding_keys, ShortcutAction, ShortcutRegistryState};
use tauri::{command, AppHandle, Manager};

/// Normalise un binding et l'applique aux settings + registry runtime.
/// Un binding vide ou blanc desactive l'action (stocke "" et retire le hotkey).
/// Un binding non vide mais non parsable retourne une erreur.
fn apply_shortcut<F>(
    app: &AppHandle,
    binding: &str,
    action: ShortcutAction,
    get_field: F,
) -> Result<String, String>
where
    F: Fn(&mut crate::settings::types::AppSettings) -> &mut String,
{
    let keys = parse_binding_keys(binding);
    if keys.is_empty() && !binding.trim().is_empty() {
        return Err("Invalid shortcut".to_string());
    }
    let normalized = keys_to_string(&keys);

    let mut s = settings::load_settings(app);
    *get_field(&mut s) = normalized.clone();
    settings::save_settings(app, &s)?;

    app.state::<ShortcutRegistryState>()
        .update_binding(action, keys);

    Ok(normalized)
}

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
    apply_shortcut(&app, &binding, ShortcutAction::StartRecording, |s| {
        &mut s.record_shortcut
    })
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
    apply_shortcut(&app, &binding, ShortcutAction::PasteLastTranscript, |s| {
        &mut s.last_transcript_shortcut
    })
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
    apply_shortcut(&app, &binding, ShortcutAction::StartRecordingCommand, |s| {
        &mut s.command_shortcut
    })
}

// ============================================================================
// Cancel Recording Shortcut
// ============================================================================

#[command]
pub fn get_cancel_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.cancel_shortcut)
}

#[command]
pub fn set_cancel_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    apply_shortcut(&app, &binding, ShortcutAction::CancelRecording, |s| {
        &mut s.cancel_shortcut
    })
}

// ============================================================================
// Cancel Recording (IPC command for overlay button)
// ============================================================================

#[command]
pub fn cancel_recording(app: AppHandle) {
    let shortcut_state = app.state::<ShortcutState>();
    shortcut_state.set_toggled(false);
    {
        let mut source = recording_state().source.lock();
        *source = RecordingSource::None;
    }
    crate::audio::cancel_recording(&app);
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
    apply_shortcut(
        &app,
        &binding,
        ShortcutAction::StartRecordingLlmMode(0),
        |s| &mut s.llm_mode_1_shortcut,
    )
}

#[command]
pub fn get_llm_mode_2_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.llm_mode_2_shortcut)
}

#[command]
pub fn set_llm_mode_2_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    apply_shortcut(
        &app,
        &binding,
        ShortcutAction::StartRecordingLlmMode(1),
        |s| &mut s.llm_mode_2_shortcut,
    )
}

#[command]
pub fn get_llm_mode_3_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.llm_mode_3_shortcut)
}

#[command]
pub fn set_llm_mode_3_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    apply_shortcut(
        &app,
        &binding,
        ShortcutAction::StartRecordingLlmMode(2),
        |s| &mut s.llm_mode_3_shortcut,
    )
}

#[command]
pub fn get_llm_mode_4_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.llm_mode_4_shortcut)
}

#[command]
pub fn set_llm_mode_4_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    apply_shortcut(
        &app,
        &binding,
        ShortcutAction::StartRecordingLlmMode(3),
        |s| &mut s.llm_mode_4_shortcut,
    )
}

// ============================================================================
// Voice Mode Toggle Shortcut
// ============================================================================

#[command]
pub fn get_voice_mode_toggle_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.voice_mode_toggle_shortcut)
}

#[command]
pub fn set_voice_mode_toggle_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    apply_shortcut(&app, &binding, ShortcutAction::ToggleVoiceMode, |s| {
        &mut s.voice_mode_toggle_shortcut
    })
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
