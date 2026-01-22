use crate::audio;
use crate::history::get_last_transcription;
use crate::settings;
use log::{error, info, warn};
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

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
            match event.state() {
                ShortcutState::Pressed => {
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
                ShortcutState::Released => {
                    // No action on shortcut release
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

pub fn init_shortcuts(app: AppHandle) {
    let s = settings::load_settings(&app);
    app.manage(crate::shortcuts::types::ShortcutState::new(
        false,
        s.record_mode == "toggle_to_talk",
        false,
    ));

    // macOS: Use tauri-plugin-global-shortcut (event-driven)
    // Parse and register record shortcut
    match s.record_shortcut.parse::<Shortcut>() {
        Ok(record_shortcut) => match register_record_shortcut(&app, record_shortcut) {
            Ok(_) => {
                info!("Registered record shortcut: {}", s.record_shortcut);
            }
            Err(e) => {
                error!("Failed to register record shortcut: {}", e);
            }
        },
        Err(_) => {
            warn!("Invalid record shortcut format: {}", s.record_shortcut);
        }
    }

    // Parse and register last transcript shortcut
    match s.last_transcript_shortcut.parse::<Shortcut>() {
        Ok(last_shortcut) => match register_last_transcript_shortcut(&app, last_shortcut) {
            Ok(_) => {
                info!(
                    "Registered last transcript shortcut: {}",
                    s.last_transcript_shortcut
                );
            }
            Err(e) => {
                error!("Failed to register last transcript shortcut: {}", e);
            }
        },
        Err(_) => {
            warn!(
                "Invalid last transcript shortcut format: {}",
                s.last_transcript_shortcut
            );
        }
    }

    // Parse and register LLM record shortcut
    match s.llm_record_shortcut.parse::<Shortcut>() {
        Ok(llm_shortcut) => match register_llm_record_shortcut(&app, llm_shortcut) {
            Ok(_) => {
                info!("Registered LLM record shortcut: {}", s.llm_record_shortcut);
            }
            Err(e) => {
                error!("Failed to register LLM record shortcut: {}", e);
            }
        },
        Err(_) => {
            warn!(
                "Invalid LLM record shortcut format: {}",
                s.llm_record_shortcut
            );
        }
    }

    // Parse and register Command record shortcut
    match s.command_shortcut.parse::<Shortcut>() {
        Ok(cmd_shortcut) => match register_command_shortcut(&app, cmd_shortcut) {
            Ok(_) => {
                info!("Registered command shortcut: {}", s.command_shortcut);
            }
            Err(e) => {
                error!("Failed to register command shortcut: {}", e);
            }
        },
        Err(_) => {
            warn!("Invalid command shortcut format: {}", s.command_shortcut);
        }
    }

    let mode_shortcuts = [
        (&s.llm_mode_1_shortcut, 0),
        (&s.llm_mode_2_shortcut, 1),
        (&s.llm_mode_3_shortcut, 2),
        (&s.llm_mode_4_shortcut, 3),
    ];

    for (shortcut_str, mode_index) in mode_shortcuts {
        if let Ok(shortcut) = shortcut_str.parse::<Shortcut>() {
            match register_mode_switch_shortcut(&app, shortcut, mode_index) {
                Ok(_) => {
                    info!("Registered mode switch shortcut: {}", shortcut_str);
                }
                Err(e) => {
                    error!(
                        "Failed to register mode switch shortcut {}: {}",
                        shortcut_str, e
                    );
                }
            }
        }
    }
}
