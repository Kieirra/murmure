#![allow(clippy::module_inception)]

mod audio;
mod clipboard;
mod commands;
mod dictionary;
mod engine;
mod formatting_rules;
mod history;
mod http_api;
mod llm;
mod model;
mod onboarding;
mod overlay;
mod settings;
mod shortcuts;
mod stats;
mod utils;

use crate::shortcuts::init_shortcuts;
use audio::preload_engine;
use audio::types::AudioState;
use commands::*;
use dictionary::Dictionary;
use http_api::HttpApiState;
use llm::llm::pull_ollama_model;
use log::{error, info, warn};
use model::Model;
use overlay::tray::setup_tray;
use std::str::FromStr;
use std::sync::Arc;
use tauri::{DeviceEventFilter, Listener, Manager};
use tauri_plugin_log::{Target, TargetKind};

fn show_main_window(app: &tauri::AppHandle) {
    if let Some(main_window) = app.get_webview_window("main") {
        match main_window.show() {
            Ok(_) => (),
            Err(e) => error!("Failed to show window: {}", e),
        }
        match main_window.set_focus() {
            Ok(_) => (),
            Err(e) => error!("Failed to focus window: {}", e),
        }
    } else {
        warn!("Main window not found");
    }
}

pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Webview),
                ])
                .level(log::LevelFilter::Trace)
                .level_for("ort", log::LevelFilter::Warn)
                .level_for("ort::logging", log::LevelFilter::Warn)
                .level_for("zbus", log::LevelFilter::Warn)
                .level_for("tracing", log::LevelFilter::Warn)
                .level_for("symphonia_core", log::LevelFilter::Warn)
                .level_for("symphonia_bundle_mp3", log::LevelFilter::Warn)
                .level_for("enigo", log::LevelFilter::Info)
                .level_for("reqwest", log::LevelFilter::Info)
                .level_for("hyper_util", log::LevelFilter::Info)
                .level_for("tauri_plugin_updater", log::LevelFilter::Info)
                .level_for("arboard", log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            show_main_window(app);
        }))
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_process::init());

    #[cfg(target_os = "macos")]
    let builder = builder.plugin(tauri_plugin_global_shortcut::Builder::new().build());

    builder
        .device_event_filter(DeviceEventFilter::Never)
        .setup(|app| {
            let model =
                Arc::new(Model::new(app.handle().clone()).expect("Failed to initialize model"));
            app.manage(model);
            app.manage(AudioState::new());

            let mut s = settings::load_settings(app.handle());

            if let Ok(level) = log::LevelFilter::from_str(&s.log_level) {
                log::set_max_level(level);
            }

            let dictionary = if !s.dictionary.is_empty() {
                let dictionary_from_settings = s.dictionary.clone();
                s = settings::remove_dictionary_from_settings(app.handle(), s)?;
                dictionary::migrate_and_load(app.handle(), dictionary_from_settings)?
            } else {
                dictionary::load(app.handle())?
            };
            app.manage(Dictionary::new(dictionary.clone()));
            app.manage(HttpApiState::new());

            match preload_engine(app.handle()) {
                Ok(_) => info!("Transcription engine initialized and ready"),
                Err(e) => info!("Transcription engine will be loaded on first use: {}", e),
            }

            setup_tray(app.handle())?;

            overlay::overlay::create_recording_overlay(app.handle());
            if s.overlay_mode.as_str() == "always" {
                overlay::overlay::show_recording_overlay(app.handle());
            }

            init_shortcuts(app.handle().clone());

            audio::sound::init_sound_system(app.handle());

            audio::microphone::init_mic_cache_if_needed(app.handle(), s.mic_id.clone());

            if s.api_enabled {
                let app_handle = app.handle().clone();
                let state = app_handle.state::<HttpApiState>().inner().clone();
                crate::http_api::spawn_http_api_thread(app_handle, s.api_port, state);
            }

            let app_handle = app.handle().clone();
            app.handle().listen("recording-limit-reached", move |_| {
                warn!("Recording limit reached, stopping...");
                crate::shortcuts::actions::force_stop_recording(&app_handle);
            });

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
            export_dictionary,
            import_dictionary,
            get_last_transcript_shortcut,
            set_last_transcript_shortcut,
            get_llm_record_shortcut,
            set_llm_record_shortcut,
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
            get_usage_stats,
            get_persist_history,
            set_persist_history,
            get_current_language,
            set_current_language,
            get_current_mic_id,
            set_current_mic_id,
            get_mic_list,
            get_onboarding_state,
            set_onboarding_used_home_shortcut,
            set_onboarding_transcribed_outside_app,
            set_onboarding_added_dictionary_word,
            set_onboarding_congrats_dismissed,
            get_llm_connect_settings,
            set_llm_connect_settings,
            test_llm_connection,
            fetch_ollama_models,
            pull_ollama_model,
            get_sound_enabled,
            set_sound_enabled,
            get_record_mode,
            set_record_mode,
            get_formatting_settings,
            set_formatting_settings,
            get_log_level,
            set_log_level
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
