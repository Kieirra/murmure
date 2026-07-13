use crate::audio::types::RecordingMode;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub enum WakeWordAction {
    Record(RecordingMode),
    RecordLlmMode(usize),
    Cancel,
    Validate,
    Submit,
}

pub struct WakeWordEntry {
    pub word: String,
    pub action: WakeWordAction,
}

pub struct WakeWordState {
    /// Whether the wake word listener is currently running
    pub active: Arc<AtomicBool>,
    /// Signal to stop the listener thread
    pub stop_signal: Arc<AtomicBool>,
    /// Consecutive short-lived listener deaths, used to back off and eventually
    /// give up when the microphone is unavailable.
    pub consecutive_failures: Arc<AtomicU32>,
    /// Handle to the listener thread (for cleanup)
    pub thread_handle: Mutex<Option<std::thread::JoinHandle<()>>>,
}

impl WakeWordState {
    pub fn new() -> Self {
        Self {
            active: Arc::new(AtomicBool::new(false)),
            stop_signal: Arc::new(AtomicBool::new(false)),
            consecutive_failures: Arc::new(AtomicU32::new(0)),
            thread_handle: Mutex::new(None),
        }
    }

    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::SeqCst)
    }
}
