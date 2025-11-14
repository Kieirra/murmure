use crate::audio;
use crate::history::get_last_transcription;
use crate::settings;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use super::TranscriptionSuspended;

/// Register the record shortcut handler
pub fn register_record_shortcut(app: &AppHandle, shortcut: Shortcut) -> Result<(), String> {
    let app_clone = app.clone();
    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            match event.state() {
                ShortcutState::Pressed => {
                    crate::onboarding::capture_focus_at_record_start(&app_clone);
                    audio::record_audio(&app_clone);
                    let _ = app_clone.emit("shortcut:record", ());
                }
                ShortcutState::Released => {
                    // Stop recording on shortcut release
                    let _ = audio::stop_recording(&app_clone);
                    let _ = app_clone.emit("shortcut:record-released", ());
                }
            }
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

pub fn init_shortcuts(app: AppHandle) {
    app.manage(TranscriptionSuspended::new(false));

    let s = settings::load_settings(&app);

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
}
