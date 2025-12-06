use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

pub struct ShortcutState {
    /// Whether transcription is currently suspended
    pub suspended: Arc<AtomicBool>,
    /// Whether "toggle to talk" mode is enabled (vs push to talk)
    pub is_toggle_required: Arc<AtomicBool>,
    /// Current toggle state (true if recording is active in toggle mode)
    pub is_toggled: Arc<AtomicBool>,
}

impl ShortcutState {
    pub fn new(suspended: bool, is_toggle_required: bool, is_toggled: bool) -> Self {
        Self {
            suspended: Arc::new(AtomicBool::new(suspended)),
            is_toggle_required: Arc::new(AtomicBool::new(is_toggle_required)),
            is_toggled: Arc::new(AtomicBool::new(is_toggled)),
        }
    }

    pub fn is_suspended(&self) -> bool {
        self.suspended.load(Ordering::SeqCst)
    }
    pub fn set_suspended(&self, value: bool) {
        self.suspended.store(value, Ordering::SeqCst)
    }

    pub fn is_toggle_required(&self) -> bool {
        self.is_toggle_required.load(Ordering::SeqCst)
    }
    pub fn set_toggle_required(&self, value: bool) {
        self.is_toggle_required.store(value, Ordering::SeqCst)
    }

    pub fn is_toggled(&self) -> bool {
        self.is_toggled.load(Ordering::SeqCst)
    }
    pub fn set_toggled(&self, value: bool) {
        self.is_toggled.store(value, Ordering::SeqCst)
    }
}

pub struct RecordShortcutKeys(pub Arc<Mutex<Vec<i32>>>);

impl RecordShortcutKeys {
    pub fn new(keys: Vec<i32>) -> Self {
        Self(Arc::new(Mutex::new(keys)))
    }
    pub fn get(&self) -> Vec<i32> {
        self.0.lock().unwrap().clone()
    }
    pub fn set(&self, keys: Vec<i32>) {
        *self.0.lock().unwrap() = keys;
    }
}

pub struct LastTranscriptShortcutKeys(pub Arc<Mutex<Vec<i32>>>);

impl LastTranscriptShortcutKeys {
    pub fn new(keys: Vec<i32>) -> Self {
        Self(Arc::new(Mutex::new(keys)))
    }
    pub fn get(&self) -> Vec<i32> {
        self.0.lock().unwrap().clone()
    }
    pub fn set(&self, keys: Vec<i32>) {
        *self.0.lock().unwrap() = keys;
    }
}

pub struct LLMRecordShortcutKeys(pub Arc<Mutex<Vec<i32>>>);

impl LLMRecordShortcutKeys {
    pub fn new(keys: Vec<i32>) -> Self {
        Self(Arc::new(Mutex::new(keys)))
    }
    pub fn get(&self) -> Vec<i32> {
        self.0.lock().unwrap().clone()
    }
    pub fn set(&self, keys: Vec<i32>) {
        *self.0.lock().unwrap() = keys;
    }
}
