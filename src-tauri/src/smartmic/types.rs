use serde::{Deserialize, Serialize};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, oneshot};

/// Recording mode for SmartMic, parsed from client strings.
#[derive(Clone, Debug)]
pub enum SmartMicMode {
    Stt,
    Llm(usize),
}

impl SmartMicMode {
    pub fn from_client(mode: &str) -> Self {
        if mode.starts_with("llm_") {
            if let Ok(idx) = mode[4..].parse::<usize>() {
                return Self::Llm(idx);
            }
        }
        Self::Stt
    }

    pub fn to_recording_mode(&self) -> crate::audio::types::RecordingMode {
        match self {
            Self::Stt => crate::audio::types::RecordingMode::Standard,
            Self::Llm(_) => crate::audio::types::RecordingMode::Llm,
        }
    }
}

/// Main state for the SmartMic feature, managed by Tauri
#[derive(Clone)]
pub struct SmartMicState {
    pub shutdown_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
    pub connected_device: Arc<Mutex<Option<ConnectedDevice>>>,
    pub paired_devices: Arc<Mutex<Vec<PairedDevice>>>,
    pub recording_buffer: Arc<Mutex<Vec<i16>>>,
    pub recording_mode: Arc<Mutex<SmartMicMode>>,
    pub sample_rate: Arc<Mutex<u32>>,
    pub is_running: Arc<AtomicBool>,
}

impl SmartMicState {
    pub fn new() -> Self {
        Self {
            shutdown_tx: Arc::new(Mutex::new(None)),
            connected_device: Arc::new(Mutex::new(None)),
            paired_devices: Arc::new(Mutex::new(Vec::new())),
            recording_buffer: Arc::new(Mutex::new(Vec::new())),
            recording_mode: Arc::new(Mutex::new(SmartMicMode::Stt)),
            sample_rate: Arc::new(Mutex::new(16000)),
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn set_shutdown_sender(&self, tx: oneshot::Sender<()>) {
        let mut guard = self.shutdown_tx.lock().unwrap();
        *guard = Some(tx);
    }

    pub fn stop(&self) {
        let mut guard = self.shutdown_tx.lock().unwrap();
        if let Some(tx) = guard.take() {
            let _ = tx.send(());
        }
    }
}

impl Default for SmartMicState {
    fn default() -> Self {
        Self::new()
    }
}

/// A paired device that has been authorized to connect
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PairedDevice {
    pub token: String,
    pub name: String,
    pub last_connected: String,
}

/// A currently connected device with its message sender
pub struct ConnectedDevice {
    pub token: String,
    pub name: String,
    pub tx: mpsc::UnboundedSender<String>,
}

/// Messages received from the smartphone client (text JSON)
#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    MouseMove { dx: f64, dy: f64 },
    Click { button: String },
    Scroll { dy: f64 },
    RecStart { mode: String },
    RecStop,
    RecCancel,
    KeyPress { key: String },
    Pair { token: String },
    ForceConnect,
}

/// Messages sent from the server to the smartphone client (text JSON)
#[derive(Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    Transcription { text: String },
    Status { recording: bool },
    MicLevel { level: f32 },
    Modes { modes: Vec<String> },
    Error { message: String },
    DeviceAlreadyConnected { device_name: String },
    ForceDisconnect,
}

impl ServerMessage {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}
