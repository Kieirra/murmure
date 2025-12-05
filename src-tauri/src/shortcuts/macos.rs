use crate::audio;
use crate::history::get_last_transcription;
use crate::settings;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use super::{IsToggleRequiredForRecording, TranscriptionSuspended};

fn handle_recording_shortcut<F>(
    app: &AppHandle,
    state: ShortcutState,
    is_recording_toggled: &mut bool,
    record_fn: F,
    start_event: &str,
    stop_event: &str,
) where
    F: Fn(&AppHandle),
{
    let is_toggle_required = app.state::<IsToggleRequiredForRecording>().get();
    let mut should_record = false;

    match state {
        ShortcutState::Pressed => {
            if !is_toggle_required {
                should_record = true;
            }
        }
        ShortcutState::Released => {
            if is_toggle_required {
                *is_recording_toggled = !*is_recording_toggled;
                should_record = *is_recording_toggled;
            } else {
                should_record = false;
            }
        }
    }

    if should_record {
        crate::onboarding::capture_focus_at_record_start(app);
        record_fn(app);
        let _ = app.emit(start_event, ());
    } else {
        let _ = audio::stop_recording(app);
        let _ = app.emit(stop_event, ());
    }
}

/// Register the record shortcut handler
pub fn register_record_shortcut(app: &AppHandle, shortcut: Shortcut) -> Result<(), String> {
    let app_clone = app.clone();
    let mut is_recording_toggled = false;

    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            handle_recording_shortcut(
                &app_clone,
                event.state(),
                &mut is_recording_toggled,
                audio::record_audio,
                "shortcut:record",
                "shortcut:record-released",
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
                                eprintln!("Failed to paste last transcription: {}", err);
                            }
                        }
                        Err(e) => {
                            eprintln!("No transcription history available: {}", e);
                        }
                    }
                    let _ = app_clone.emit("shortcut:last-transcript", ());
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
    let mut is_recording_toggled = false;

    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            handle_recording_shortcut(
                &app_clone,
                event.state(),
                &mut is_recording_toggled,
                audio::record_audio_with_llm,
                "shortcut:llm-record",
                "shortcut:llm-record-released",
            );
        })
        .map_err(|e| format!("Failed to register LLM record shortcut: {}", e))?;
    Ok(())
}

pub fn init_shortcuts(app: AppHandle) {
    app.manage(TranscriptionSuspended::new(false));

    let s = settings::load_settings(&app);
    app.manage(IsToggleRequiredForRecording::new(s.record_mode == "toggle_to_talk"));

    // macOS: Use tauri-plugin-global-shortcut (event-driven)
    // Parse and register record shortcut
    match s.record_shortcut.parse::<Shortcut>() {
        Ok(record_shortcut) => match register_record_shortcut(&app, record_shortcut) {
            Ok(_) => {
                println!("Registered record shortcut: {}", s.record_shortcut);
            }
            Err(e) => {
                eprintln!("Failed to register record shortcut: {}", e);
            }
        },
        Err(_) => {
            eprintln!("Invalid record shortcut format: {}", s.record_shortcut);
        }
    }

    // Parse and register last transcript shortcut
    match s.last_transcript_shortcut.parse::<Shortcut>() {
        Ok(last_shortcut) => match register_last_transcript_shortcut(&app, last_shortcut) {
            Ok(_) => {
                println!(
                    "Registered last transcript shortcut: {}",
                    s.last_transcript_shortcut
                );
            }
            Err(e) => {
                eprintln!("Failed to register last transcript shortcut: {}", e);
            }
        },
        Err(_) => {
            eprintln!(
                "Invalid last transcript shortcut format: {}",
                s.last_transcript_shortcut
            );
        }
    }

    // Parse and register LLM record shortcut
    match s.llm_record_shortcut.parse::<Shortcut>() {
        Ok(llm_shortcut) => match register_llm_record_shortcut(&app, llm_shortcut) {
            Ok(_) => {
                println!("Registered LLM record shortcut: {}", s.llm_record_shortcut);
            }
            Err(e) => {
                eprintln!("Failed to register LLM record shortcut: {}", e);
            }
        },
        Err(_) => {
            eprintln!(
                "Invalid LLM record shortcut format: {}",
                s.llm_record_shortcut
            );
        }
    }
}
