use crate::audio::recorder::AudioRecorder;
use crate::engine::engine::ParakeetEngine;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct AudioState {
    pub recorder: Mutex<Option<AudioRecorder>>,
    pub engine: Mutex<Option<ParakeetEngine>>,
    pub current_file_name: Mutex<Option<String>>,
    use_llm_shortcut: AtomicBool,
}

impl AudioState {
    pub fn new() -> Self {
        Self {
            recorder: Mutex::new(None),
            engine: Mutex::new(None),
            current_file_name: Mutex::new(None),
            use_llm_shortcut: AtomicBool::new(false),
        }
    }

    pub fn set_use_llm_shortcut(&self, use_llm: bool) {
        self.use_llm_shortcut.store(use_llm, Ordering::SeqCst);
    }

    pub fn get_use_llm_shortcut(&self) -> bool {
        self.use_llm_shortcut.load(Ordering::SeqCst)
    }
}
