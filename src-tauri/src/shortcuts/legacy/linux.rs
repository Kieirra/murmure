use crate::audio::{record_audio, stop_recording, write_last_transcription};
use crate::history::get_last_transcription;
use crate::shortcuts::{
    initialize_shortcut_states, CommandShortcutKeys, LLMMode1ShortcutKeys, LLMMode2ShortcutKeys,
    LLMMode3ShortcutKeys, LLMMode4ShortcutKeys, LLMRecordShortcutKeys, LastTranscriptShortcutKeys,
    RecordShortcutKeys,
};
use log::error;
use parking_lot::RwLock;
use rdev::{listen, Event, EventType, Key};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Manager};

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

pub fn init_shortcuts(app: AppHandle) {
    let pressed_keys: Arc<RwLock<HashSet<i32>>> = Arc::new(RwLock::new(HashSet::new()));
    let pressed_keys_listener = pressed_keys.clone();
    let pressed_keys_checker = pressed_keys.clone();

    initialize_shortcut_states(&app);

    std::thread::spawn(move || {
        if let Err(error) = listen(move |event: Event| match event.event_type {
            EventType::KeyPress(key) => {
                if let Some(vk) = rdev_key_to_vk(&key) {
                    pressed_keys_listener.write().insert(vk);
                }
            }
            EventType::KeyRelease(key) => {
                if let Some(vk) = rdev_key_to_vk(&key) {
                    pressed_keys_listener.write().remove(&vk);
                }
            }
            _ => {}
        }) {
            error!("Error starting keyboard listener: {:?}", error);
        }
    });

    std::thread::spawn(move || {
        let app_handle = app.clone();
        #[derive(PartialEq)]
        enum RecordingSource {
            None,
            Standard,
            Llm,
            Command,
        }
        let mut recording_source = RecordingSource::None;
        let mut last_transcript_pressed = false;
        let mut last_mode_switch_time = std::time::Instant::now();

        loop {
            let shortcut_state = app_handle.state::<crate::shortcuts::types::ShortcutState>();
            if shortcut_state.is_suspended() {
                std::thread::sleep(Duration::from_millis(32));
                continue;
            }

            let record_required_keys = app_handle.state::<RecordShortcutKeys>().get();
            let llm_record_required_keys = app_handle.state::<LLMRecordShortcutKeys>().get();
            let last_transcript_required_keys =
                app_handle.state::<LastTranscriptShortcutKeys>().get();
            let command_required_keys = app_handle.state::<CommandShortcutKeys>().get();
            let shortcut_state = app_handle.state::<crate::shortcuts::types::ShortcutState>();

            let llm_mode_1_keys = app_handle.state::<LLMMode1ShortcutKeys>().get();
            let llm_mode_2_keys = app_handle.state::<LLMMode2ShortcutKeys>().get();
            let llm_mode_3_keys = app_handle.state::<LLMMode3ShortcutKeys>().get();
            let llm_mode_4_keys = app_handle.state::<LLMMode4ShortcutKeys>().get();

            {
                let pressed = pressed_keys_checker.read();

                if last_mode_switch_time.elapsed() > Duration::from_millis(300) {
                    let mode_1_pressed = !llm_mode_1_keys.is_empty()
                        && llm_mode_1_keys.iter().all(|k| pressed.contains(k));
                    let mode_2_pressed = !llm_mode_2_keys.is_empty()
                        && llm_mode_2_keys.iter().all(|k| pressed.contains(k));
                    let mode_3_pressed = !llm_mode_3_keys.is_empty()
                        && llm_mode_3_keys.iter().all(|k| pressed.contains(k));
                    let mode_4_pressed = !llm_mode_4_keys.is_empty()
                        && llm_mode_4_keys.iter().all(|k| pressed.contains(k));

                    let target_mode = if mode_1_pressed {
                        Some(0)
                    } else if mode_2_pressed {
                        Some(1)
                    } else if mode_3_pressed {
                        Some(2)
                    } else if mode_4_pressed {
                        Some(3)
                    } else {
                        None
                    };

                    if let Some(index) = target_mode {
                        crate::llm::switch_active_mode(&app_handle, index);
                        last_mode_switch_time = std::time::Instant::now();
                    }
                }
            }

            if record_required_keys.is_empty()
                && llm_record_required_keys.is_empty()
                && last_transcript_required_keys.is_empty()
                && command_required_keys.is_empty()
            {
                std::thread::sleep(Duration::from_millis(32));
                continue;
            }

            let pressed = pressed_keys_checker.read();
            let all_record_keys_down = !record_required_keys.is_empty()
                && record_required_keys.iter().all(|k| pressed.contains(k));
            let all_llm_record_keys_down = !llm_record_required_keys.is_empty()
                && llm_record_required_keys.iter().all(|k| pressed.contains(k));
            let all_command_keys_down = !command_required_keys.is_empty()
                && command_required_keys.iter().all(|k| pressed.contains(k));

            let all_last_transcript_keys_down = !last_transcript_required_keys.is_empty()
                && last_transcript_required_keys
                    .iter()
                    .all(|k| pressed.contains(k));

            if (all_record_keys_down || all_llm_record_keys_down || all_command_keys_down)
                && shortcut_state.is_toggle_required()
            {
                let current_toggle = shortcut_state.is_toggled();
                shortcut_state.set_toggled(!current_toggle);
                std::thread::sleep(Duration::from_millis(150));
            }

            let should_record = if shortcut_state.is_toggle_required() {
                shortcut_state.is_toggled()
            } else {
                true
            };

            match recording_source {
                RecordingSource::None => {
                    // Priority: LLM record > Standard record
                    if all_llm_record_keys_down && should_record {
                        crate::onboarding::onboarding::capture_focus_at_record_start(&app_handle);
                        crate::audio::record_audio_with_llm(&app_handle);
                        recording_source = RecordingSource::Llm;
                    } else if all_command_keys_down && should_record {
                        crate::onboarding::onboarding::capture_focus_at_record_start(&app_handle);
                        crate::audio::record_audio_with_command(&app_handle);
                        recording_source = RecordingSource::Command;
                    } else if all_record_keys_down && should_record {
                        crate::onboarding::onboarding::capture_focus_at_record_start(&app_handle);
                        record_audio(&app_handle);
                        recording_source = RecordingSource::Standard;
                    }
                }
                RecordingSource::Standard => {
                    // Check if recording limit was reached
                    let audio_state = app_handle.state::<crate::audio::types::AudioState>();
                    if audio_state.is_limit_reached() {
                        crate::shortcuts::actions::force_stop_recording(&app_handle);
                        recording_source = RecordingSource::None;
                    } else if !all_record_keys_down && !shortcut_state.is_toggled() {
                        let _ = stop_recording(&app_handle);
                        recording_source = RecordingSource::None;
                    }
                }
                RecordingSource::Llm => {
                    // Check if recording limit was reached
                    let audio_state = app_handle.state::<crate::audio::types::AudioState>();
                    if audio_state.is_limit_reached() {
                        crate::shortcuts::actions::force_stop_recording(&app_handle);
                        recording_source = RecordingSource::None;
                    } else if !all_llm_record_keys_down && !shortcut_state.is_toggled() {
                        let _ = stop_recording(&app_handle);
                        recording_source = RecordingSource::None;
                    }
                }
                RecordingSource::Command => {
                    // Check if recording limit was reached
                    let audio_state = app_handle.state::<crate::audio::types::AudioState>();
                    if audio_state.is_limit_reached() {
                        crate::shortcuts::actions::force_stop_recording(&app_handle);
                        recording_source = RecordingSource::None;
                    } else if !all_command_keys_down && !shortcut_state.is_toggled() {
                        let _ = stop_recording(&app_handle);
                        recording_source = RecordingSource::None;
                    }
                }
            }

            if !last_transcript_pressed && all_last_transcript_keys_down {
                if let Ok(last_transcript) = get_last_transcription(&app_handle) {
                    let _ = write_last_transcription(&app_handle, &last_transcript);
                }
                last_transcript_pressed = true;
            }
            if last_transcript_pressed && !all_last_transcript_keys_down {
                last_transcript_pressed = false;
            }

            std::thread::sleep(Duration::from_millis(32));
        }
    });
}
