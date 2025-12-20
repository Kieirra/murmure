use crate::audio::recorder::AudioRecorder;
use crate::engine::ParakeetEngine;
use cpal::Device;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct AudioState {
    pub recorder: Mutex<Option<AudioRecorder>>,
    pub engine: Mutex<Option<ParakeetEngine>>,
    pub current_file_name: Mutex<Option<String>>,
    use_llm_shortcut: AtomicBool,
    /// Flag indicating recording duration limit has been reached
    pub limit_reached: std::sync::Arc<AtomicBool>,
    /// Cached audio input device to avoid re-enumerating devices on each recording
    pub cached_device: Mutex<Option<Device>>,
}

impl AudioState {
    pub fn new() -> Self {
        Self {
            recorder: Mutex::new(None),
            engine: Mutex::new(None),
            current_file_name: Mutex::new(None),
            use_llm_shortcut: AtomicBool::new(false),
            limit_reached: std::sync::Arc::new(AtomicBool::new(false)),
            cached_device: Mutex::new(None),
        }
    }

    pub fn set_use_llm_shortcut(&self, use_llm: bool) {
        self.use_llm_shortcut.store(use_llm, Ordering::SeqCst);
    }

    pub fn get_use_llm_shortcut(&self) -> bool {
        self.use_llm_shortcut.load(Ordering::SeqCst)
    }

    pub fn is_limit_reached(&self) -> bool {
        self.limit_reached.load(Ordering::SeqCst)
    }

    pub fn get_limit_reached_arc(&self) -> std::sync::Arc<AtomicBool> {
        self.limit_reached.clone()
    }

    /// Sets the cached audio device
    pub fn set_cached_device(&self, device: Option<Device>) {
        *self.cached_device.lock() = device;
    }

    /// Gets a clone of the cached audio device
    pub fn get_cached_device(&self) -> Option<Device> {
        self.cached_device.lock().clone()
    }
}
