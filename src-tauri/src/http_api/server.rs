use super::types::{CancelOnDrop, HttpApiState, TempWav, TranscribeState};
use crate::audio;
use anyhow::Result;
use axum::{
    extract::{DefaultBodyLimit, Multipart},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use log::info;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct TranscriptionResponse {
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

fn error_response(status: StatusCode, error: String) -> axum::response::Response {
    (status, Json(ErrorResponse { error })).into_response()
}

pub async fn start_http_api(
    app: tauri::AppHandle,
    port: u16,
    api_state: HttpApiState,
) -> Result<()> {
    let state = TranscribeState {
        app: Arc::new(app),
        transcribe_lock: Arc::new(tokio::sync::Mutex::new(())),
    };

    let router = Router::new()
        .route("/api/transcribe", post(transcribe_handler))
        .with_state(state)
        .layer(DefaultBodyLimit::max(100_000_000));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("HTTP API listening on http://{}", addr);

    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    api_state.set_shutdown_sender(shutdown_tx);

    let server = axum::serve(listener, router);

    tokio::select! {
        _ = server => {
            info!("HTTP API server ended normally");
        }
        _ = shutdown_rx => {
            info!("HTTP API server shutdown signal received");
        }
    }

    Ok(())
}

async fn transcribe_handler(
    axum::extract::State(state): axum::extract::State<TranscribeState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut audio_bytes = None;

    loop {
        match multipart.next_field().await {
            Ok(Some(field)) if field.name() == Some("audio") => match field.bytes().await {
                Ok(b) => audio_bytes = Some(b),
                Err(e) => {
                    return error_response(
                        StatusCode::BAD_REQUEST,
                        format!("Failed to read audio file: {}", e),
                    )
                }
            },
            Ok(Some(_)) => {}
            Ok(None) => break,
            Err(e) => {
                return error_response(
                    StatusCode::BAD_REQUEST,
                    format!("Failed to parse multipart: {}", e),
                )
            }
        }
    }

    match audio_bytes {
        Some(bytes) => transcribe_bytes(&state, bytes).await,
        None => error_response(
            StatusCode::BAD_REQUEST,
            "No 'audio' field in multipart request".to_string(),
        ),
    }
}

async fn transcribe_bytes(
    state: &TranscribeState,
    bytes: axum::body::Bytes,
) -> axum::response::Response {
    let id = uuid::Uuid::new_v4();
    let temp = TempWav(std::env::temp_dir().join(format!("murmure-{}.wav", id)));
    let mut short_id = id.to_string();
    short_id.truncate(8);

    if let Err(e) = std::fs::write(&temp.0, bytes) {
        return error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to write audio file: {}", e),
        );
    }

    let transcribe_guard = state.transcribe_lock.clone().lock_owned().await;

    let cancelled = Arc::new(AtomicBool::new(false));
    let _cancel_on_drop = CancelOnDrop(cancelled.clone());

    info!("HTTP API transcription {}: starting", short_id);
    let started = std::time::Instant::now();

    let app = state.app.clone();
    let cancelled = cancelled.clone();
    let log_id = short_id.clone();
    let joined = tokio::task::spawn_blocking(move || {
        // Owning the lock and the temp file here ties them to the real work, not to the connection.
        let _guard = transcribe_guard;
        let temp = temp;
        if cancelled.load(Ordering::SeqCst) {
            info!(
                "HTTP API transcription {}: cancelled by client disconnect",
                log_id
            );
            return Ok(None);
        }
        audio::preload_engine(&app).map_err(|e| format!("Model not available: {}", e))?;
        let result = audio::transcribe_file_chunked_cancellable(&app, &temp.0, &cancelled)
            .map_err(|e| format!("Transcription failed: {}", e))?;
        if result.is_none() {
            info!(
                "HTTP API transcription {}: cancelled by client disconnect",
                log_id
            );
        }
        Ok(result)
    })
    .await;

    match joined {
        Ok(Ok(Some(text))) => {
            info!(
                "HTTP API transcription {}: done in {} ms",
                short_id,
                started.elapsed().as_millis()
            );
            (StatusCode::OK, Json(TranscriptionResponse { text })).into_response()
        }
        Ok(Ok(None)) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Transcription cancelled".to_string(),
        ),
        Ok(Err(e)) => error_response(StatusCode::INTERNAL_SERVER_ERROR, e),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Transcription task failed: {}", e),
        ),
    }
}
