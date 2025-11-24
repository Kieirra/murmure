use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;

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
