use super::engine::KokoroEngine;
use super::player::TtsPlayer;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use tauri::{AppHandle, Manager};

pub struct TtsState {
    pub engine: Arc<tokio::sync::Mutex<Option<KokoroEngine>>>,
    pub player: TtsPlayer,
    pub generation_id: Arc<AtomicU32>,
}

pub fn init_tts(app: &AppHandle) {
    app.manage(TtsState {
        engine: Arc::new(tokio::sync::Mutex::new(None)),
        player: TtsPlayer::new(),
        generation_id: Arc::new(AtomicU32::new(0)),
    });
}
