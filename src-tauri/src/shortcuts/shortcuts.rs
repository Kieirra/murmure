use crate::shortcuts::types::{
    ActivationMode, RecordingSource, ShortcutAction, ShortcutRegistry, ShortcutRegistryState,
    ShortcutState,
};
use log::{debug, error, info, warn};
use parking_lot::Mutex;
use std::collections::HashSet;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Manager};

#[cfg(any(target_os = "linux", target_os = "windows"))]
use rdev::{listen, Event, EventType, Key};

#[derive(Debug)]
enum KeyEvent {
    Pressed(i32),
    Released(i32),
}

struct ShortcutManager {
    app_handle: AppHandle,
    pressed_keys: Mutex<HashSet<i32>>,
    recording_source: Mutex<RecordingSource>,
    active_recording_keys: Mutex<Vec<i32>>,
    last_trigger_times: Mutex<Vec<Instant>>,
    last_mode_switch: Mutex<Instant>,
}

impl ShortcutManager {
    fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            pressed_keys: Mutex::new(HashSet::new()),
            recording_source: Mutex::new(RecordingSource::None),
            active_recording_keys: Mutex::new(Vec::new()),
            last_trigger_times: Mutex::new(Vec::new()),
            last_mode_switch: Mutex::new(Instant::now() - Duration::from_secs(1)),
        }
    }

    fn handle_key_event(&self, event: KeyEvent) {
        match event {
            KeyEvent::Pressed(key) => {
                self.pressed_keys.lock().insert(key);
                self.check_bindings();
            }
            KeyEvent::Released(key) => {
                self.pressed_keys.lock().remove(&key);
                self.check_release_stop(key);
            }
        }
    }

    fn check_bindings(&self) {
        let shortcut_state = self.app_handle.state::<ShortcutState>();
        if shortcut_state.is_suspended() {
            return;
        }

        let registry_state = self.app_handle.state::<ShortcutRegistryState>();
        let registry = registry_state.0.read();
        let pressed = self.pressed_keys.lock();
        let mut trigger_times = self.last_trigger_times.lock();

        while trigger_times.len() < registry.bindings.len() {
            trigger_times.push(Instant::now() - Duration::from_secs(1));
        }

        for (i, binding) in registry.bindings.iter().enumerate() {
            if binding.keys.is_empty() {
                continue;
            }

            let all_pressed = binding.keys.iter().all(|k| pressed.contains(k));
            if !all_pressed {
                continue;
            }

            if trigger_times[i].elapsed() < Duration::from_millis(150) {
                continue;
            }

            debug!("Shortcut triggered: {:?}", binding.action);
            trigger_times[i] = Instant::now();
            drop(pressed);
            drop(trigger_times);

            self.execute_action(&binding.action, &binding.activation_mode, &binding.keys);
            return;
        }
    }

    fn execute_action(&self, action: &ShortcutAction, mode: &ActivationMode, keys: &[i32]) {
        let shortcut_state = self.app_handle.state::<ShortcutState>();
        let mut recording_source = self.recording_source.lock();

        match action {
            ShortcutAction::StartRecording => {
                self.handle_recording(
                    &mut recording_source,
                    RecordingSource::Standard,
                    mode,
                    &shortcut_state,
                    keys,
                    || crate::audio::record_audio(&self.app_handle),
                );
            }
            ShortcutAction::StartRecordingLLM => {
                self.handle_recording(
                    &mut recording_source,
                    RecordingSource::Llm,
                    mode,
                    &shortcut_state,
                    keys,
                    || crate::audio::record_audio_with_llm(&self.app_handle),
                );
            }
            ShortcutAction::StartRecordingCommand => {
                self.handle_recording(
                    &mut recording_source,
                    RecordingSource::Command,
                    mode,
                    &shortcut_state,
                    keys,
                    || crate::audio::record_audio_with_command(&self.app_handle),
                );
            }
            ShortcutAction::PasteLastTranscript => {
                if let Ok(transcript) = crate::history::get_last_transcription(&self.app_handle) {
                    let _ = crate::audio::write_last_transcription(&self.app_handle, &transcript);
                }
            }
            ShortcutAction::SwitchLLMMode(index) => {
                let mut last_switch = self.last_mode_switch.lock();
                if last_switch.elapsed() > Duration::from_millis(300) {
                    crate::llm::switch_active_mode(&self.app_handle, *index);
                    *last_switch = Instant::now();
                    info!("Switched to LLM mode {}", index);
                }
            }
        }
    }

    fn handle_recording<F>(
        &self,
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
                    self.stop_recording(recording_source);
                } else if *recording_source == RecordingSource::None {
                    shortcut_state.set_toggled(true);
                    self.start_recording(recording_source, target, keys, start_fn);
                }
            }
            ActivationMode::PushToTalk => {
                if *recording_source == RecordingSource::None {
                    self.start_recording(recording_source, target, keys, start_fn);
                }
            }
        }
    }

    fn start_recording<F>(
        &self,
        recording_source: &mut RecordingSource,
        target: RecordingSource,
        keys: &[i32],
        start_fn: F,
    ) where
        F: FnOnce(),
    {
        crate::onboarding::onboarding::capture_focus_at_record_start(&self.app_handle);
        start_fn();
        *recording_source = target;
        *self.active_recording_keys.lock() = keys.to_vec();
        info!("Started {:?} recording", target);
    }

    fn stop_recording(&self, recording_source: &mut RecordingSource) {
        let audio_state = self.app_handle.state::<crate::audio::types::AudioState>();
        if audio_state.is_limit_reached() {
            force_stop_recording(&self.app_handle);
        } else {
            let _ = crate::audio::stop_recording(&self.app_handle);
        }
        *recording_source = RecordingSource::None;
        self.active_recording_keys.lock().clear();
        info!("Stopped recording");
    }

    fn check_release_stop(&self, released_key: i32) {
        let shortcut_state = self.app_handle.state::<ShortcutState>();
        if shortcut_state.is_toggled() {
            return;
        }

        let mut recording_source = self.recording_source.lock();
        if *recording_source == RecordingSource::None {
            return;
        }

        let active_keys = self.active_recording_keys.lock();
        if !active_keys.contains(&released_key) {
            return;
        }

        let pressed = self.pressed_keys.lock();
        let still_active = active_keys.iter().all(|k| pressed.contains(k));
        drop(pressed);
        drop(active_keys);

        if !still_active {
            self.stop_recording(&mut recording_source);
        }
    }
}

pub fn force_stop_recording(app: &AppHandle) {
    let shortcut_state = app.state::<ShortcutState>();
    shortcut_state.set_toggled(false);
    crate::audio::stop_recording(app);
}

pub fn initialize_shortcut_states(app_handle: &AppHandle) {
    app_handle.manage(ShortcutState::new());
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub fn init_shortcuts(app: AppHandle) {
    let settings = crate::settings::load_settings(&app);
    let registry = ShortcutRegistry::from_settings(&settings);

    initialize_shortcut_states(&app);
    app.manage(ShortcutRegistryState::new(registry));

    let manager = Arc::new(ShortcutManager::new(app));
    let (tx, rx) = channel::<KeyEvent>();

    // Listener thread (rdev)
    std::thread::spawn(move || {
        debug!("Starting rdev keyboard listener");
        if let Err(e) = listen(move |event: Event| {
            if let Some(evt) = convert_rdev_event(&event) {
                let _ = tx.send(evt);
            }
        }) {
            error!("rdev listener error: {:?}", e);
        }
    });

    // Processor thread (no async runtime needed)
    std::thread::spawn(move || {
        debug!("Starting shortcut processor");
        while let Ok(event) = rx.recv() {
            manager.handle_key_event(event);
        }
        warn!("Shortcut processor stopped");
    });
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
fn convert_rdev_event(event: &Event) -> Option<KeyEvent> {
    match event.event_type {
        EventType::KeyPress(key) => rdev_key_to_vk(&key).map(KeyEvent::Pressed),
        EventType::KeyRelease(key) => rdev_key_to_vk(&key).map(KeyEvent::Released),
        _ => None,
    }
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
fn rdev_key_to_vk(key: &Key) -> Option<i32> {
    match key {
        Key::MetaLeft | Key::MetaRight => Some(0x5B),
        Key::ControlLeft | Key::ControlRight => Some(0x11),
        Key::Alt | Key::AltGr => Some(0x12),
        Key::ShiftLeft | Key::ShiftRight => Some(0x10),
        Key::KeyA => Some(0x41),
        Key::KeyB => Some(0x42),
        Key::KeyC => Some(0x43),
        Key::KeyD => Some(0x44),
        Key::KeyE => Some(0x45),
        Key::KeyF => Some(0x46),
        Key::KeyG => Some(0x47),
        Key::KeyH => Some(0x48),
        Key::KeyI => Some(0x49),
        Key::KeyJ => Some(0x4A),
        Key::KeyK => Some(0x4B),
        Key::KeyL => Some(0x4C),
        Key::KeyM => Some(0x4D),
        Key::KeyN => Some(0x4E),
        Key::KeyO => Some(0x4F),
        Key::KeyP => Some(0x50),
        Key::KeyQ => Some(0x51),
        Key::KeyR => Some(0x52),
        Key::KeyS => Some(0x53),
        Key::KeyT => Some(0x54),
        Key::KeyU => Some(0x55),
        Key::KeyV => Some(0x56),
        Key::KeyW => Some(0x57),
        Key::KeyX => Some(0x58),
        Key::KeyY => Some(0x59),
        Key::KeyZ => Some(0x5A),
        Key::Num0 => Some(0x30),
        Key::Num1 => Some(0x31),
        Key::Num2 => Some(0x32),
        Key::Num3 => Some(0x33),
        Key::Num4 => Some(0x34),
        Key::Num5 => Some(0x35),
        Key::Num6 => Some(0x36),
        Key::Num7 => Some(0x37),
        Key::Num8 => Some(0x38),
        Key::Num9 => Some(0x39),
        Key::F1 => Some(0x70),
        Key::F2 => Some(0x71),
        Key::F3 => Some(0x72),
        Key::F4 => Some(0x73),
        Key::F5 => Some(0x74),
        Key::F6 => Some(0x75),
        Key::F7 => Some(0x76),
        Key::F8 => Some(0x77),
        Key::F9 => Some(0x78),
        Key::F10 => Some(0x79),
        Key::F11 => Some(0x7A),
        Key::F12 => Some(0x7B),
        Key::Space => Some(0x20),
        Key::Return => Some(0x0D),
        Key::Escape => Some(0x1B),
        Key::Tab => Some(0x09),
        Key::Backspace => Some(0x08),
        Key::Delete => Some(0x2E),
        Key::Insert => Some(0x2D),
        Key::Home => Some(0x24),
        Key::End => Some(0x23),
        Key::PageUp => Some(0x21),
        Key::PageDown => Some(0x22),
        Key::UpArrow => Some(0x26),
        Key::DownArrow => Some(0x28),
        Key::LeftArrow => Some(0x25),
        Key::RightArrow => Some(0x27),
        _ => None,
    }
}
