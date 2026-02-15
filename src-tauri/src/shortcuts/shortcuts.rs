use crate::audio::types::RecordingMode;
use crate::shortcuts::registry::ShortcutRegistryState;
use crate::shortcuts::types::{
    recording_state, ActivationMode, KeyEventType, RecordingSource, ShortcutAction,
    ShortcutRegistry, ShortcutState,
};
use log::info;
use std::time::Duration;
use tauri::{AppHandle, Manager};

pub fn handle_shortcut_event(
    app: &AppHandle,
    action: &ShortcutAction,
    mode: &ActivationMode,
    event_type: KeyEventType,
) {
    let shortcut_state = app.state::<ShortcutState>();

    match action {
        ShortcutAction::StartRecording => {
            handle_recording_event(
                app,
                RecordingSource::Standard,
                mode,
                event_type,
                &shortcut_state,
                || crate::audio::record_audio(app, RecordingMode::Standard),
            );
        }
        ShortcutAction::StartRecordingLLM => {
            handle_recording_event(
                app,
                RecordingSource::Llm,
                mode,
                event_type,
                &shortcut_state,
                || crate::audio::record_audio(app, RecordingMode::Llm),
            );
        }
        ShortcutAction::StartRecordingCommand => {
            handle_recording_event(
                app,
                RecordingSource::Command,
                mode,
                event_type,
                &shortcut_state,
                || crate::audio::record_audio(app, RecordingMode::Command),
            );
        }
        ShortcutAction::PasteLastTranscript => {
            if event_type == KeyEventType::Pressed {
                if let Ok(transcript) = crate::history::get_last_transcription(app) {
                    let _ = crate::audio::write_last_transcription(app, &transcript);
                }
            }
        }
        ShortcutAction::CancelRecording => {
            if event_type == KeyEventType::Pressed {
                let mut recording_source = recording_state().source.lock();
                if *recording_source != RecordingSource::None {
                    crate::audio::cancel_recording(app);
                    *recording_source = RecordingSource::None;
                    info!("Recording cancelled by user");
                }
            }
        }
        ShortcutAction::SwitchLLMMode(index) => {
            if event_type == KeyEventType::Pressed {
                let mut last_switch = recording_state().last_mode_switch.lock();
                if last_switch.elapsed() > Duration::from_millis(300) {
                    crate::llm::switch_active_mode(app, *index);
                    *last_switch = std::time::Instant::now();
                    info!("Switched to LLM mode {}", index);
                }
            }
        }
    }
}

fn handle_recording_event<F>(
    app: &AppHandle,
    target: RecordingSource,
    mode: &ActivationMode,
    event_type: KeyEventType,
    shortcut_state: &ShortcutState,
    start_fn: F,
) where
    F: FnOnce(),
{
    let mut recording_source = recording_state().source.lock();

    match mode {
        ActivationMode::PushToTalk => match event_type {
            KeyEventType::Pressed => {
                if *recording_source == RecordingSource::None {
                    start_recording(app, &mut recording_source, target, start_fn);
                }
            }
            KeyEventType::Released => {
                if *recording_source == target {
                    stop_recording(app, &mut recording_source);
                }
            }
        },
        ActivationMode::ToggleToTalk => {
            if event_type == KeyEventType::Released {
                if *recording_source == target {
                    shortcut_state.set_toggled(false);
                    stop_recording(app, &mut recording_source);
                } else if *recording_source == RecordingSource::None {
                    shortcut_state.set_toggled(true);
                    start_recording(app, &mut recording_source, target, start_fn);
                }
            }
        }
    }
}

fn start_recording<F>(
    app: &AppHandle,
    recording_source: &mut RecordingSource,
    target: RecordingSource,
    start_fn: F,
) where
    F: FnOnce(),
{
    crate::onboarding::onboarding::capture_focus_at_record_start(app);
    start_fn();
    *recording_source = target;
    info!("Started {:?} recording", target);
}

fn stop_recording(app: &AppHandle, recording_source: &mut RecordingSource) {
    let audio_state = app.state::<crate::audio::types::AudioState>();
    if audio_state.is_limit_reached() {
        // Reset toggle state when limit is reached (relevant for ToggleToTalk mode).
        // We do NOT call force_stop_recording() here because recording_source
        // is already locked by our caller — re-locking would deadlock.
        let shortcut_state = app.state::<ShortcutState>();
        shortcut_state.set_toggled(false);
    }
    let _ = crate::audio::stop_recording(app);
    *recording_source = RecordingSource::None;
    info!("Stopped recording");
}

pub fn force_stop_recording(app: &AppHandle) {
    let shortcut_state = app.state::<ShortcutState>();
    shortcut_state.set_toggled(false);
    let mut recording_source = recording_state().source.lock();
    *recording_source = RecordingSource::None;
    let _ = crate::audio::stop_recording(app);
}

#[cfg(target_os = "linux")]
pub fn init_shortcuts(app: AppHandle) {
    let settings = crate::settings::load_settings(&app);
    let registry = ShortcutRegistry::from_settings(&settings);

    app.manage(ShortcutState::new());
    app.manage(ShortcutRegistryState::new(registry));

    crate::shortcuts::platform_linux::init(app);
}

#[cfg(target_os = "windows")]
pub fn init_shortcuts(app: AppHandle) {
    let settings = crate::settings::load_settings(&app);
    let registry = ShortcutRegistry::from_settings(&settings);

    app.manage(ShortcutState::new());
    app.manage(ShortcutRegistryState::new(registry));

    crate::shortcuts::platform_windows::init(app);
}

#[cfg(target_os = "macos")]
pub fn init_shortcuts(app: AppHandle) {
    let settings = crate::settings::load_settings(&app);
    let registry = ShortcutRegistry::from_settings(&settings);

    app.manage(ShortcutState::new());
    app.manage(ShortcutRegistryState::new(registry));

    crate::shortcuts::platform_macos::init(app);
}
