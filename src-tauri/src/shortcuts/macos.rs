//! macOS Shortcut Implementation
//!
//! This module provides keyboard shortcut handling for macOS using tauri-plugin-global-shortcut.
//! It uses the unified ShortcutRegistry for shortcut definitions.

use crate::audio;
use crate::history::get_last_transcription;
use crate::shortcuts::registry::{ActivationMode, ShortcutAction, ShortcutRegistry};
use log::{debug, error, info, warn};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

/// Handle a recording shortcut event (press or release)
fn handle_recording_shortcut<F>(
    app: &AppHandle,
    state: ShortcutState,
    shortcut_state: &crate::shortcuts::types::ShortcutState,
    record_fn: F,
) where
    F: Fn(&AppHandle),
{
    let is_toggle_required = shortcut_state.is_toggle_required();
    let mut should_record = false;

    match state {
        ShortcutState::Pressed => {
            if !is_toggle_required {
                should_record = true;
            }
        }
        ShortcutState::Released => {
            if is_toggle_required {
                let current_toggle = shortcut_state.is_toggled();
                shortcut_state.set_toggled(!current_toggle);
                should_record = !current_toggle;
            } else {
                should_record = false;
            }
        }
    }

    if should_record {
        crate::onboarding::onboarding::capture_focus_at_record_start(app);
        record_fn(app);
    } else {
        let _ = audio::stop_recording(app);
    }
}

/// Register the record shortcut handler
pub fn register_record_shortcut(app: &AppHandle, shortcut: Shortcut) -> Result<(), String> {
    let app_clone = app.clone();

    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            let shortcut_state = app_clone.state::<crate::shortcuts::types::ShortcutState>();
            handle_recording_shortcut(
                &app_clone,
                event.state(),
                &shortcut_state,
                audio::record_audio,
            );
        })
        .map_err(|e| format!("Failed to register record shortcut: {}", e))?;
    Ok(())
}

/// Register the last transcript shortcut handler
pub fn register_last_transcript_shortcut(
    app: &AppHandle,
    shortcut: Shortcut,
) -> Result<(), String> {
    let app_clone = app.clone();
    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            if let ShortcutState::Pressed = event.state() {
                // Paste last transcript on shortcut press
                match get_last_transcription(&app_clone) {
                    Ok(text) => {
                        if let Err(err) = audio::write_last_transcription(&app_clone, &text) {
                            error!("Failed to paste last transcription: {}", err);
                        }
                    }
                    Err(e) => {
                        warn!("No transcription history available: {}", e);
                    }
                }
            }
        })
        .map_err(|e| format!("Failed to register last transcript shortcut: {}", e))?;
    Ok(())
}

/// Register the LLM record shortcut handler
pub fn register_llm_record_shortcut(app: &AppHandle, shortcut: Shortcut) -> Result<(), String> {
    let app_clone = app.clone();

    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            let shortcut_state = app_clone.state::<crate::shortcuts::types::ShortcutState>();
            handle_recording_shortcut(
                &app_clone,
                event.state(),
                &shortcut_state,
                audio::record_audio_with_llm,
            );
        })
        .map_err(|e| format!("Failed to register LLM record shortcut: {}", e))?;
    Ok(())
}

/// Register the Command record shortcut handler
pub fn register_command_shortcut(app: &AppHandle, shortcut: Shortcut) -> Result<(), String> {
    let app_clone = app.clone();

    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            let shortcut_state = app_clone.state::<crate::shortcuts::types::ShortcutState>();
            handle_recording_shortcut(
                &app_clone,
                event.state(),
                &shortcut_state,
                audio::record_audio_with_command,
            );
        })
        .map_err(|e| format!("Failed to register command shortcut: {}", e))?;
    Ok(())
}

/// Register a mode switch shortcut handler
pub fn register_mode_switch_shortcut(
    app: &AppHandle,
    shortcut: Shortcut,
    mode_index: usize,
) -> Result<(), String> {
    let app_clone = app.clone();

    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            if let ShortcutState::Pressed = event.state() {
                crate::llm::switch_active_mode(&app_clone, mode_index);
            }
        })
        .map_err(|e| format!("Failed to register mode switch shortcut: {}", e))?;
    Ok(())
}

/// Initialize shortcut system for macOS
///
/// This function:
/// 1. Loads settings and creates a ShortcutRegistry
/// 2. Initializes the ShortcutState
/// 3. Registers each binding via tauri-plugin-global-shortcut
pub fn init_shortcuts(app: AppHandle) {
    let settings = crate::settings::load_settings(&app);
    let registry = Arc::new(ShortcutRegistry::from_settings(&settings));

    // Initialize shortcut state
    app.manage(crate::shortcuts::types::ShortcutState::new(
        false,
        settings.record_mode == "toggle_to_talk",
        false,
    ));

    debug!("Initializing macOS shortcuts with {} bindings", registry.bindings.len());

    // Register each binding using the registry
    for binding in registry.bindings.iter() {
        if binding.keys.is_empty() {
            continue;
        }

        // Convert keys to shortcut string format
        let shortcut_str = crate::shortcuts::helpers::keys_to_string(&binding.keys);

        // Parse the shortcut string
        let shortcut: Shortcut = match shortcut_str.parse() {
            Ok(s) => s,
            Err(_) => {
                warn!("Invalid shortcut format: {}", shortcut_str);
                continue;
            }
        };

        // Register based on action type
        let result = match &binding.action {
            ShortcutAction::StartRecording => {
                register_record_shortcut(&app, shortcut)
            }
            ShortcutAction::StartRecordingLLM => {
                register_llm_record_shortcut(&app, shortcut)
            }
            ShortcutAction::StartRecordingCommand => {
                register_command_shortcut(&app, shortcut)
            }
            ShortcutAction::PasteLastTranscript => {
                register_last_transcript_shortcut(&app, shortcut)
            }
            ShortcutAction::SwitchLLMMode(mode_index) => {
                register_mode_switch_shortcut(&app, shortcut, *mode_index)
            }
        };

        match result {
            Ok(_) => {
                info!("Registered macOS shortcut: {} for {:?}", shortcut_str, binding.action);
            }
            Err(e) => {
                error!("Failed to register shortcut {} for {:?}: {}", shortcut_str, binding.action, e);
            }
        }
    }
}
