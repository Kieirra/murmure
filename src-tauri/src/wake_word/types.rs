use crate::audio::types::RecordingMode;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub enum WakeWordAction {
    Record(RecordingMode),
    RecordLlmMode(usize),
    Cancel,
    Validate,
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
    /// Handle to the listener thread (for cleanup)
    pub thread_handle: Mutex<Option<std::thread::JoinHandle<()>>>,
}

impl WakeWordState {
    pub fn new() -> Self {
        Self {
            active: Arc::new(AtomicBool::new(false)),
            stop_signal: Arc::new(AtomicBool::new(false)),
            thread_handle: Mutex::new(None),
        }
    }

    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::SeqCst)
    }
}
