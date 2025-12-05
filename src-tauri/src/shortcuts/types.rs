use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

pub struct TranscriptionSuspended(pub Arc<AtomicBool>);

impl TranscriptionSuspended {
    pub fn new(suspended: bool) -> Self {
        Self(Arc::new(AtomicBool::new(suspended)))
    }
    pub fn get(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
    pub fn set(&self, value: bool) {
        self.0.store(value, Ordering::SeqCst)
    }
}

pub struct IsToggleRequiredForRecording(pub Arc<AtomicBool>);

impl IsToggleRequiredForRecording {
    pub fn new(required: bool) -> Self {
        Self(Arc::new(AtomicBool::new(required)))
    }
    pub fn get(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
    pub fn set(&self, value: bool) {
        self.0.store(value, Ordering::SeqCst)
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
