use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState, Shortcut};
use crate::settings;
use crate::audio;
use crate::history::get_last_transcription;

use super::TranscriptionSuspended;


pub fn init_shortcuts(app: AppHandle) {

    // Sinon ça crash ... 
    app.manage(TranscriptionSuspended::new(false));

    let s = settings::load_settings(&app);

    // macOS: Use tauri-plugin-global-shortcut (event-driven)
    // Parse and register record shortcut
    if let Ok(record_shortcut) = s.record_shortcut.parse::<Shortcut>() {
        let app_clone = app.clone();
        if let Err(e) = app.global_shortcut().on_shortcut(record_shortcut, move |_app, _shortcut, event| {
            if event.state() == ShortcutState::Pressed {
                // Start recording on shortcut press
                audio::record_audio(&app_clone);
                let _ = app_clone.emit("shortcut:record", ());
            } else if event.state() == ShortcutState::Released {
                // Stop recording on shortcut release
                let _ = audio::stop_recording(&app_clone);
                let _ = app_clone.emit("shortcut:record-released", ());
            }
        }) {
            eprintln!("Failed to register record shortcut: {}", e);
        } else {
            println!("Registered record shortcut: {}", s.record_shortcut);
        }
    } else {
        eprintln!("Invalid record shortcut format: {}", s.record_shortcut);
    }

    // Parse and register last transcript shortcut
    if let Ok(last_shortcut) = s.last_transcript_shortcut.parse::<Shortcut>() {
        let app_clone = app.clone();
        if let Err(e) = app.global_shortcut().on_shortcut(last_shortcut, move |_app, _shortcut, event| {
            if event.state() == ShortcutState::Pressed {
                // Paste last transcript on shortcut press
                match get_last_transcription(&app_clone) {
                    Ok(text) => {
                        if let Err(err) = audio::write_transcription(&app_clone, &text) {
                            eprintln!("Failed to paste last transcription: {}", err);
                        }
                    }
                    Err(e) => {
                        eprintln!("No transcription history available: {}", e);
                    }
                }
                let _ = app_clone.emit("shortcut:last-transcript", ());
            }
        }) {
            eprintln!("Failed to register last transcript shortcut: {}", e);
        } else {
            println!("Registered last transcript shortcut: {}", s.last_transcript_shortcut);
        }
    } else {
        eprintln!("Invalid last transcript shortcut format: {}", s.last_transcript_shortcut);
    }
}
