use crate::shortcuts::types::{
    ActivationMode, RecordingSource, ShortcutAction, ShortcutRegistry, ShortcutRegistryState,
    ShortcutState,
};
use log::info;
use parking_lot::Mutex;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Manager};

struct RecordingState {
    source: Mutex<RecordingSource>,
    active_keys: Mutex<Vec<i32>>,
    last_mode_switch: Mutex<Instant>,
}

impl RecordingState {
    fn new() -> Self {
        Self {
            source: Mutex::new(RecordingSource::None),
            active_keys: Mutex::new(Vec::new()),
            last_mode_switch: Mutex::new(Instant::now() - Duration::from_secs(1)),
        }
    }
}

static RECORDING_STATE: once_cell::sync::Lazy<RecordingState> =
    once_cell::sync::Lazy::new(RecordingState::new);

pub fn execute_action(
    app: &AppHandle,
    action: &ShortcutAction,
    mode: &ActivationMode,
    keys: &[i32],
) {
    let shortcut_state = app.state::<ShortcutState>();
    let mut recording_source = RECORDING_STATE.source.lock();

    match action {
        ShortcutAction::StartRecording => {
            handle_recording(
                app,
                &mut recording_source,
                RecordingSource::Standard,
                mode,
                &shortcut_state,
                keys,
                || crate::audio::record_audio(app),
            );
        }
        ShortcutAction::StartRecordingLLM => {
            handle_recording(
                app,
                &mut recording_source,
                RecordingSource::Llm,
                mode,
                &shortcut_state,
                keys,
                || crate::audio::record_audio_with_llm(app),
            );
        }
        ShortcutAction::StartRecordingCommand => {
            handle_recording(
                app,
                &mut recording_source,
                RecordingSource::Command,
                mode,
                &shortcut_state,
                keys,
                || crate::audio::record_audio_with_command(app),
            );
        }
        ShortcutAction::PasteLastTranscript => {
            if let Ok(transcript) = crate::history::get_last_transcription(app) {
                let _ = crate::audio::write_last_transcription(app, &transcript);
            }
        }
        ShortcutAction::SwitchLLMMode(index) => {
            let mut last_switch = RECORDING_STATE.last_mode_switch.lock();
            if last_switch.elapsed() > Duration::from_millis(300) {
                crate::llm::switch_active_mode(app, *index);
                *last_switch = Instant::now();
                info!("Switched to LLM mode {}", index);
            }
        }
    }
}

fn handle_recording<F>(
    app: &AppHandle,
    recording_source: &mut RecordingSource,
    target: RecordingSource,
    mode: &ActivationMode,
    shortcut_state: &ShortcutState,
    keys: &[i32],
    start_fn: F,
) where
    F: FnOnce(),
{
    match mode {
        ActivationMode::ToggleToTalk => {
            if *recording_source == target {
                shortcut_state.set_toggled(false);
                stop_recording(app, recording_source);
            } else if *recording_source == RecordingSource::None {
                shortcut_state.set_toggled(true);
                start_recording(app, recording_source, target, keys, start_fn);
            }
        }
        ActivationMode::PushToTalk => {
            if *recording_source == RecordingSource::None {
                start_recording(app, recording_source, target, keys, start_fn);
            }
        }
    }
}

fn start_recording<F>(
    app: &AppHandle,
    recording_source: &mut RecordingSource,
    target: RecordingSource,
    keys: &[i32],
    start_fn: F,
) where
    F: FnOnce(),
{
    crate::onboarding::onboarding::capture_focus_at_record_start(app);
    start_fn();
    *recording_source = target;
    *RECORDING_STATE.active_keys.lock() = keys.to_vec();
    info!("Started {:?} recording", target);
}

fn stop_recording(app: &AppHandle, recording_source: &mut RecordingSource) {
    let audio_state = app.state::<crate::audio::types::AudioState>();
    if audio_state.is_limit_reached() {
        force_stop_recording(app);
    } else {
        let _ = crate::audio::stop_recording(app);
    }
    *recording_source = RecordingSource::None;
    RECORDING_STATE.active_keys.lock().clear();
    info!("Stopped recording");
}

pub fn check_release_stop(app: &AppHandle, released_key: i32) {
    let shortcut_state = app.state::<ShortcutState>();
    if shortcut_state.is_toggled() {
        return;
    }

    let mut recording_source = RECORDING_STATE.source.lock();
    if *recording_source == RecordingSource::None {
        return;
    }

    let active_keys = RECORDING_STATE.active_keys.lock();
    if !active_keys.contains(&released_key) {
        return;
    }

    drop(active_keys);
    stop_recording(app, &mut recording_source);
}

pub fn force_stop_recording(app: &AppHandle) {
    let shortcut_state = app.state::<ShortcutState>();
    shortcut_state.set_toggled(false);
    crate::audio::stop_recording(app);
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub fn init_shortcuts(app: AppHandle) {
    let settings = crate::settings::load_settings(&app);
    let registry = ShortcutRegistry::from_settings(&settings);

    app.manage(ShortcutState::new());
    app.manage(ShortcutRegistryState::new(registry));

    crate::shortcuts::platform_rdev::init(app);
}

#[cfg(target_os = "macos")]
pub fn init_shortcuts(app: AppHandle) {
    let settings = crate::settings::load_settings(&app);
    let registry = ShortcutRegistry::from_settings(&settings);

    app.manage(ShortcutState::new());
    app.manage(ShortcutRegistryState::new(registry));

    crate::shortcuts::platform_macos::init(app);
}
