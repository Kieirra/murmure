use axum::body::Bytes;
use axum::http::StatusCode;
use axum::Json;
use log::warn;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use tokio::sync::oneshot;

#[derive(Clone)]
pub struct TranscribeState {
    pub app: Arc<tauri::AppHandle>,
    pub transcribe_lock: Arc<tokio::sync::Mutex<()>>,
}

pub(super) struct CancelOnDrop(pub(super) Arc<AtomicBool>);

impl Drop for CancelOnDrop {
    fn drop(&mut self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

pub(super) struct TempWav(pub(super) PathBuf);

impl Drop for TempWav {
    fn drop(&mut self) {
        if let Err(e) = std::fs::remove_file(&self.0) {
            warn!("HTTP API: failed to remove temp audio file: {}", e);
        }
    }
}

#[derive(Serialize)]
pub struct PromptModeResponse {
    pub text: String,
    pub transcription: String,
}

#[derive(Serialize)]
pub struct PromptModeListResponse {
    pub prompts: Vec<String>,
}

#[derive(Serialize)]
pub struct PromptModeErrorResponse {
    pub error: String,
    pub code: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription: Option<String>,
}

pub(super) type PromptModeRejection = (StatusCode, Json<PromptModeErrorResponse>);

pub(super) struct PromptModeForm {
    pub(super) audio: Option<Bytes>,
    pub(super) texts: HashMap<&'static str, String>,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HttpApiServerState {
    Stopped,
    Running,
    PortInUse,
    Failed,
}

#[derive(Clone, Debug, Serialize)]
pub struct HttpApiStatus {
    pub state: HttpApiServerState,
    pub port: u16,
    pub message: Option<String>,
}

#[derive(Debug)]
pub enum HttpApiStartError {
    PortInUse(u16),
    Other(String),
}

impl HttpApiStartError {
    pub fn server_state(&self) -> HttpApiServerState {
        match self {
            HttpApiStartError::PortInUse(_) => HttpApiServerState::PortInUse,
            HttpApiStartError::Other(_) => HttpApiServerState::Failed,
        }
    }
}

impl fmt::Display for HttpApiStartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpApiStartError::PortInUse(port) => write!(f, "Port {} is already in use", port),
            HttpApiStartError::Other(message) => f.write_str(message),
        }
    }
}

#[derive(Clone)]
pub struct HttpApiState {
    shutdown_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
    pub is_running: Arc<AtomicBool>,
    status: Arc<Mutex<HttpApiStatus>>,
}

impl HttpApiState {
    pub fn new() -> Self {
        Self {
            shutdown_tx: Arc::new(Mutex::new(None)),
            is_running: Arc::new(AtomicBool::new(false)),
            status: Arc::new(Mutex::new(HttpApiStatus {
                state: HttpApiServerState::Stopped,
                port: 0,
                message: None,
            })),
        }
    }

    pub fn status(&self) -> HttpApiStatus {
        self.status
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    pub fn set_status(&self, state: HttpApiServerState, port: u16, message: Option<String>) {
        let mut guard = self.status.lock().unwrap_or_else(PoisonError::into_inner);
        *guard = HttpApiStatus {
            state,
            port,
            message,
        };
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

impl Default for HttpApiState {
    fn default() -> Self {
        Self::new()
    }
}
