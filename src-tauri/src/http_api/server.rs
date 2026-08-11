use super::types::{HttpApiState, TranscribeState};
use crate::audio;
use anyhow::Result;
use axum::{
    extract::{multipart::Field, DefaultBodyLimit, Multipart},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use log::info;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
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
    loop {
        match multipart.next_field().await {
            Ok(Some(field)) if field.name() == Some("audio") => {
                return transcribe_field(&state, field).await
            }
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

    error_response(
        StatusCode::BAD_REQUEST,
        "No 'audio' field in multipart request".to_string(),
    )
}

async fn transcribe_field(state: &TranscribeState, field: Field<'_>) -> axum::response::Response {
    let bytes = match field.bytes().await {
        Ok(b) => b,
        Err(e) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                format!("Failed to read audio file: {}", e),
            )
        }
    };

    let temp_path = std::env::temp_dir().join(format!("murmure-{}.wav", uuid::Uuid::new_v4()));

    if let Err(e) = std::fs::write(&temp_path, bytes) {
        return error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to write audio file: {}", e),
        );
    }

    let _guard = state.transcribe_lock.lock().await;

    let app = state.app.clone();
    let path = temp_path.clone();
    let joined = tokio::task::spawn_blocking(move || {
        audio::preload_engine(&app).map_err(|e| format!("Model not available: {}", e))?;
        audio::transcribe_file_chunked(&app, &path)
            .map_err(|e| format!("Transcription failed: {}", e))
    })
    .await;

    let _ = std::fs::remove_file(&temp_path);

    match joined {
        Ok(Ok(text)) => (StatusCode::OK, Json(TranscriptionResponse { text })).into_response(),
        Ok(Err(e)) => error_response(StatusCode::INTERNAL_SERVER_ERROR, e),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Transcription task failed: {}", e),
        ),
    }
}
