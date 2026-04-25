use crate::audio::types::RecordingMode;
use crate::shortcuts::registry::ShortcutRegistryState;
use crate::shortcuts::types::{
    recording_state, ActivationMode, KeyEventType, RecordingSource, ShortcutAction,
    ShortcutRegistry, ShortcutState,
};
use log::info;
use parking_lot::Mutex;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Manager};

const SHORTCUT_COOLDOWN: Duration = Duration::from_millis(250);

fn within_cooldown(last: &Mutex<Instant>) -> bool {
    last.lock().elapsed() < SHORTCUT_COOLDOWN
}

pub fn handle_shortcut_event(
    app: &AppHandle,
    action: &ShortcutAction,
    mode: &ActivationMode,
    event_type: KeyEventType,
) {
    let shortcut_state = app.state::<ShortcutState>();

    match action {
        ShortcutAction::StartRecording => {
            let app_for_fn = app.clone();
            handle_recording_event(
                app,
                RecordingSource::Standard,
                mode,
                event_type,
                &shortcut_state,
                move || crate::audio::record_audio(&app_for_fn, RecordingMode::Standard),
            );
        }
        ShortcutAction::StartRecordingLLM => {
            let app_for_fn = app.clone();
            handle_recording_event(
                app,
                RecordingSource::Llm,
                mode,
                event_type,
                &shortcut_state,
                move || crate::audio::record_audio(&app_for_fn, RecordingMode::Llm),
            );
        }
        ShortcutAction::StartRecordingCommand => {
            let app_for_fn = app.clone();
            handle_recording_event(
                app,
                RecordingSource::Command,
                mode,
                event_type,
                &shortcut_state,
                move || crate::audio::record_audio(&app_for_fn, RecordingMode::Command),
            );
        }
        ShortcutAction::PasteLastTranscript => {
            if event_type == KeyEventType::Pressed {
                if let Ok(transcript) = crate::history::get_last_transcription(app) {
                    let _ = crate::audio::write_last_transcription(app, &transcript);
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
        ShortcutAction::CancelRecording => {
            if event_type == KeyEventType::Pressed {
                let recording_source = recording_state().source.lock();
                if *recording_source != RecordingSource::None {
                    drop(recording_source);
                    force_cancel_recording(app);
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
    F: FnOnce() + Send + 'static,
{
    let mut recording_source = recording_state().source.lock();

    match mode {
        ActivationMode::PushToTalk => match event_type {
            KeyEventType::Pressed => {
                if *recording_source == RecordingSource::None {
                    if within_cooldown(&recording_state().last_toggle_stop) {
                        info!("PushToTalk press ignored (cooldown after stop)");
                        return;
                    }
                    start_recording(app, &mut recording_source, target, start_fn);
                }
            }
            KeyEventType::Released => {
                if *recording_source == target {
                    // Symmetric with ToggleToTalk: drop Release events within the
                    // start cooldown so synthetic Release+Press pairs (X11 auto-repeat,
                    // Wayland portal rafales) cannot stop recording mid-utterance.
                    if within_cooldown(&recording_state().last_toggle_start) {
                        info!("PushToTalk release ignored (cooldown after start)");
                        return;
                    }
                    pre_stop(app, &mut recording_source);
                    *recording_state().last_toggle_stop.lock() = Instant::now();
                    drop(recording_source);
                    finish_stop(app);
                }
            }
        },
        ActivationMode::ToggleToTalk => {
            if event_type == KeyEventType::Released {
                if *recording_source == target {
                    // Cooldown after a recent start absorbs X11 auto-repeat:
                    // holding the key past ~500ms emits synthetic Release events
                    // that would otherwise toggle recording off immediately.
                    if within_cooldown(&recording_state().last_toggle_start) {
                        info!("ToggleToTalk stop ignored (cooldown after start)");
                        return;
                    }
                    shortcut_state.set_toggled(false);
                    pre_stop(app, &mut recording_source);
                    *recording_state().last_toggle_stop.lock() = Instant::now();
                    drop(recording_source);
                    finish_stop(app);
                } else if *recording_source == RecordingSource::None {
                    if within_cooldown(&recording_state().last_toggle_stop) {
                        info!("ToggleToTalk start ignored (cooldown after stop)");
                        return;
                    }
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
    F: FnOnce() + Send + 'static,
{
    crate::onboarding::onboarding::capture_focus_at_record_start(app);
    *recording_source = target;
    *recording_state().last_toggle_start.lock() = Instant::now();
    // Run off-thread so the shortcut processor stays reactive during the
    // ~100ms CPAL init and doesn't queue up auto-repeat events.
    std::thread::spawn(move || {
        start_fn();
        info!("Started {:?} recording", target);
    });
}

fn pre_stop(app: &AppHandle, recording_source: &mut RecordingSource) {
    let audio_state = app.state::<crate::audio::types::AudioState>();
    if audio_state.is_limit_reached() {
        let shortcut_state = app.state::<ShortcutState>();
        shortcut_state.set_toggled(false);
    }
    *recording_source = RecordingSource::None;
}

fn finish_stop(app: &AppHandle) {
    // Off-thread because stop_recording blocks on the LLM request and paste
    // (~1s), during which the processor must keep handling keyboard events.
    let app = app.clone();
    std::thread::spawn(move || {
        let _ = crate::audio::stop_recording(&app);
        info!("Stopped recording");
    });
}

pub fn force_stop_recording(app: &AppHandle) {
    let shortcut_state = app.state::<ShortcutState>();
    shortcut_state.set_toggled(false);
    {
        let mut recording_source = recording_state().source.lock();
        *recording_source = RecordingSource::None;
    }
    let _ = crate::audio::stop_recording(app);
}

pub fn force_cancel_recording(app: &AppHandle) {
    let shortcut_state = app.state::<ShortcutState>();
    shortcut_state.set_toggled(false);
    {
        let mut recording_source = recording_state().source.lock();
        *recording_source = RecordingSource::None;
    }
    crate::audio::cancel_recording(app);
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

#[cfg(test)]
mod tests {
    use super::{within_cooldown, SHORTCUT_COOLDOWN};
    use parking_lot::Mutex;
    use std::time::{Duration, Instant};

    #[test]
    fn cooldown_active_for_recent_instant() {
        let recent = Mutex::new(Instant::now());
        assert!(within_cooldown(&recent));
    }

    #[test]
    fn cooldown_inactive_past_threshold() {
        let past = Mutex::new(Instant::now() - SHORTCUT_COOLDOWN - Duration::from_millis(50));
        assert!(!within_cooldown(&past));
    }

    #[test]
    fn cooldown_inactive_at_exact_threshold() {
        let at_boundary = Mutex::new(Instant::now() - SHORTCUT_COOLDOWN);
        assert!(!within_cooldown(&at_boundary));
    }

    #[test]
    fn cooldown_threshold_matches_documented_value() {
        // PTT/Toggle behaviour assumes 250 ms. Larger lets auto-repeat noise
        // through, smaller drops legit very-short taps.
        assert_eq!(SHORTCUT_COOLDOWN, Duration::from_millis(250));
    }
}
