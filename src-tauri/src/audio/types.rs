use crate::audio::recorder::AudioRecorder;
use crate::engine::MedAsrEngine;
use cpal::Device;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct AudioState {
    pub recorder: Mutex<Option<AudioRecorder>>,
    pub engine: Mutex<Option<MedAsrEngine>>,
    pub current_file_name: Mutex<Option<String>>,
    recording_mode: std::sync::atomic::AtomicU8,
    /// Flag indicating recording duration limit has been reached
    pub limit_reached: std::sync::Arc<AtomicBool>,
    /// Cached audio input device to avoid re-enumerating devices on each recording
    pub cached_device: Mutex<Option<Device>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RecordingMode {
    Standard = 0,
    Llm = 1,
    Command = 2,
}

impl From<u8> for RecordingMode {
    fn from(val: u8) -> Self {
        match val {
            0 => RecordingMode::Standard,
            1 => RecordingMode::Llm,
            2 => RecordingMode::Command,
            _ => RecordingMode::Standard,
        }
    }
}

impl AudioState {
    pub fn new() -> Self {
        Self {
            recorder: Mutex::new(None),
            engine: Mutex::new(None),
            current_file_name: Mutex::new(None),
            recording_mode: std::sync::atomic::AtomicU8::new(RecordingMode::Standard as u8),
            limit_reached: std::sync::Arc::new(AtomicBool::new(false)),
            cached_device: Mutex::new(None),
        }
    }

    pub fn set_recording_mode(&self, mode: RecordingMode) {
        self.recording_mode.store(mode as u8, Ordering::SeqCst);
    }

    pub fn get_recording_mode(&self) -> RecordingMode {
        self.recording_mode.load(Ordering::SeqCst).into()
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
