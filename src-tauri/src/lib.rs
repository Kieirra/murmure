mod audio;
mod build_info;
mod clipboard;
mod commands;
mod dictionary;
mod engine;
mod history;
mod http_api;
mod model;
mod overlay;
mod settings;
mod shortcuts;
mod tray_icon;

use audio::preload_engine;
use commands::*;
use dictionary::Dictionary;
use http_api::HttpApiState;
use model::Model;
use shortcuts::init_shortcuts;
use std::sync::Arc;
use tauri::{DeviceEventFilter, Manager};
use tray_icon::setup_tray;

extern crate lazy_static;

use crate::shortcuts::{LastTranscriptShortcutKeys, RecordShortcutKeys, TranscriptionSuspended};

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

            let record_keys = shortcuts::parse_binding_keys(&s.record_shortcut);
            app.manage(RecordShortcutKeys::new(record_keys));

            let last_transcript_keys = shortcuts::parse_binding_keys(&s.last_transcript_shortcut);
            app.manage(LastTranscriptShortcutKeys::new(last_transcript_keys));

            app.manage(TranscriptionSuspended::new(false));

            init_shortcuts(app.handle().clone());

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
            get_build_target,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
