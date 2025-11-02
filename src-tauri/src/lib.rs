mod audio;
mod clipboard;
mod commands;
mod dictionary;
mod engine;
mod history;
mod http_api;
mod model;
mod overlay;
mod settings;
#[cfg(target_os = "windows")]
mod shortcuts;
mod tray_icon;

use audio::preload_engine;
use commands::*;
use dictionary::Dictionary;
use http_api::HttpApiState;
use model::Model;
use std::sync::Arc;
use tauri::{DeviceEventFilter, Manager, Emitter};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState, Shortcut};
use tray_icon::setup_tray;

#[cfg(target_os = "windows")]
use {
    crate::shortcuts::{LastTranscriptShortcutKeys, RecordShortcutKeys, TranscriptionSuspended},
    shortcuts::init_shortcuts,
};


fn show_main_window(app: &tauri::AppHandle) {
    if let Some(main_window) = app.get_webview_window("main") {
        if let Err(e) = main_window.show() {
            eprintln!("Failed to show window: {}", e);
        }
        if let Err(e) = main_window.set_focus() {
            eprintln!("Failed to focus window: {}", e);
        }
    } else {
        eprintln!("Main window not found");
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            show_main_window(app);
        }))
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .device_event_filter(DeviceEventFilter::Never)
        .setup(|app| {
            let model =
                Arc::new(Model::new(app.handle().clone()).expect("Failed to initialize model"));
            app.manage(model);

            let s = settings::load_settings(&app.handle());
            app.manage(Dictionary::new(s.dictionary.clone()));
            app.manage(HttpApiState::new());

            match preload_engine(&app.handle()) {
                Ok(_) => println!("Transcription engine ready"),
                Err(e) => println!("Transcription engine will be loaded on first use: {}", e),
            }

            setup_tray(&app.handle())?;

            overlay::create_recording_overlay(&app.handle());
            if s.overlay_mode.as_str() == "always" {
                if let Some(overlay_window) = app.get_webview_window("recording_overlay") {
                    let _ = overlay_window.show();
                    let _ = overlay_window.set_ignore_cursor_events(true);
                }
            }

            #[cfg(target_os = "windows")]
            {
                // Windows: Use custom polling-based handler
                let record_keys = shortcuts::parse_binding_keys(&s.record_shortcut);
                app.manage(RecordShortcutKeys::new(record_keys));
    
                let last_transcript_keys = shortcuts::parse_binding_keys(&s.last_transcript_shortcut);
                app.manage(LastTranscriptShortcutKeys::new(last_transcript_keys));
    
                app.manage(TranscriptionSuspended::new(false));
    
                init_shortcuts(app.handle().clone());
            }

            #[cfg(not(target_os = "windows"))]
            {
                           // Register global shortcuts
                let app_handle = app.handle().clone();

                // macOS/Linux: Use tauri-plugin-global-shortcut (event-driven)
                // Parse and register record shortcut
                if let Ok(record_shortcut) = s.record_shortcut.parse::<Shortcut>() {
                    let app_clone = app_handle.clone();
                    if let Err(e) = app_handle.global_shortcut().on_shortcut(record_shortcut, move |_app, _shortcut, event| {
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
                    let app_clone = app_handle.clone();
                    if let Err(e) = app_handle.global_shortcut().on_shortcut(last_shortcut, move |_app, _shortcut, event| {
                        if event.state() == ShortcutState::Pressed {
                            // Paste last transcript on shortcut press
                            match history::get_last_transcription(&app_clone) {
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

            if s.api_enabled {
                let app_handle = app.handle().clone();
                let state = app_handle.state::<HttpApiState>().inner().clone();
                crate::http_api::spawn_http_api_thread(app_handle, s.api_port, state);
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![
            is_model_available,
            get_model_path,
            get_recent_transcriptions,
            clear_history,
            get_record_shortcut,
            set_record_shortcut,
            set_dictionary,
            get_dictionary,
            get_last_transcript_shortcut,
            set_last_transcript_shortcut,
            get_overlay_mode,
            set_overlay_mode,
            get_overlay_position,
            set_overlay_position,
            suspend_transcription,
            resume_transcription,
            get_api_enabled,
            set_api_enabled,
            get_api_port,
            set_api_port,
            start_http_api_server,
            stop_http_api_server,
            get_copy_to_clipboard,
            set_copy_to_clipboard,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
