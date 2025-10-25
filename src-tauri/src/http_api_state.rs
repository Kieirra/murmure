use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;

/// Shared state for HTTP API server lifecycle management
#[derive(Clone)]
pub struct HttpApiState {
    /// Channel to signal the server to stop
    /// None = server not running, Some = server is running
    shutdown_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

impl HttpApiState {
    pub fn new() -> Self {
        Self {
            shutdown_tx: Arc::new(Mutex::new(None)),
        }
    }

    /// Store the shutdown sender when server starts
    pub fn set_shutdown_sender(&self, tx: oneshot::Sender<()>) {
        let mut guard = self.shutdown_tx.lock().unwrap();
        *guard = Some(tx);
    }

    /// Signal the server to stop by sending shutdown signal
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
