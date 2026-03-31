use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, oneshot};

/// Main state for the SmartMic feature, managed by Tauri
#[derive(Clone)]
pub struct SmartMicState {
    pub shutdown_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
    pub connected_device: Arc<Mutex<Option<ConnectedDevice>>>,
    pub paired_devices: Arc<Mutex<Vec<PairedDevice>>>,
    pub recording_buffer: Arc<Mutex<Vec<i16>>>,
    pub recording_mode: Arc<Mutex<String>>,
    pub sample_rate: Arc<Mutex<u32>>,
}

impl SmartMicState {
    pub fn new() -> Self {
        Self {
            shutdown_tx: Arc::new(Mutex::new(None)),
            connected_device: Arc::new(Mutex::new(None)),
            paired_devices: Arc::new(Mutex::new(Vec::new())),
            recording_buffer: Arc::new(Mutex::new(Vec::new())),
            recording_mode: Arc::new(Mutex::new("stt".to_string())),
            sample_rate: Arc::new(Mutex::new(16000)),
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
    Pair { token: String },
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
}

impl ServerMessage {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}
