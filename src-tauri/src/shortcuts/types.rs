use parking_lot::Mutex;
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyEventType {
    Pressed,
    Released,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShortcutAction {
    StartRecording,
    StartRecordingLLM,
    StartRecordingCommand,
    StartRecordingSecondary,
    PasteLastTranscript,
    SwitchLLMMode(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActivationMode {
    PushToTalk,
    ToggleToTalk,
}

#[derive(Debug, Clone)]
pub struct ShortcutBinding {
    pub keys: Vec<i32>,
    pub action: ShortcutAction,
    pub activation_mode: ActivationMode,
}

// === Registry ===

#[derive(Debug, Clone)]
pub struct ShortcutRegistry {
    pub bindings: Vec<ShortcutBinding>,
}

// === States ===

pub struct ShortcutState {
    pub suspended: AtomicBool,
    pub is_toggled: AtomicBool,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RecordingSource {
    None,
    Standard,
    Llm,
    Command,
    Secondary,
}

pub struct RecordingState {
    pub(crate) source: Mutex<RecordingSource>,
    pub(crate) last_mode_switch: Mutex<Instant>,
}

impl RecordingState {
    pub fn new() -> Self {
        Self {
            source: Mutex::new(RecordingSource::None),
            last_mode_switch: Mutex::new(Instant::now() - Duration::from_secs(1)),
        }
    }
}

static RECORDING_STATE: once_cell::sync::Lazy<RecordingState> =
    once_cell::sync::Lazy::new(RecordingState::new);

pub fn recording_state() -> &'static RecordingState {
    &RECORDING_STATE
}
