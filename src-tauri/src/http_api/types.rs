use log::warn;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
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

#[derive(Clone)]
pub struct HttpApiState {
    shutdown_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

impl HttpApiState {
    pub fn new() -> Self {
        Self {
            shutdown_tx: Arc::new(Mutex::new(None)),
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

impl Default for HttpApiState {
    fn default() -> Self {
        Self::new()
    }
}
