use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
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
