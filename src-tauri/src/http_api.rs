use crate::audio;
use crate::dictionary::{fix_transcription_with_dictionary, get_cc_rules_path, Dictionary};
use anyhow::Result;
use axum::{
    extract::{DefaultBodyLimit, Multipart},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tauri::Manager;
use tower_http::cors::CorsLayer;

#[derive(Serialize, Deserialize)]
pub struct TranscriptionResponse {
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn start_http_api(
    app: tauri::AppHandle,
    port: u16,
) -> Result<()> {
    let app = Arc::new(app);

    // Create router with transcription endpoint
    let router = Router::new()
        .route("/api/transcribe", post(transcribe_handler))
        .with_state(app.clone())
        .layer(CorsLayer::permissive())
        .layer(DefaultBodyLimit::max(100_000_000)); // 100 MB limit

    // Bind to localhost
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    println!("HTTP API listening on http://{}", addr);

    axum::serve(listener, router).await?;

    Ok(())
}

async fn transcribe_handler(
    axum::extract::State(app): axum::extract::State<Arc<tauri::AppHandle>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    // Extract the audio file from multipart
    loop {
        match multipart.next_field().await {
            Ok(Some(field)) => {
                if field.name() == Some("audio") {
                    // Read file to temporary location
                    let bytes = match field.bytes().await {
                        Ok(b) => b,
                        Err(e) => {
                            return (
                                StatusCode::BAD_REQUEST,
                                Json(ErrorResponse {
                                    error: format!("Failed to read audio file: {}", e),
                                }),
                            )
                                .into_response()
                        }
                    };

                    // Write to temporary WAV file
                    let temp_path = std::env::temp_dir().join(format!(
                        "murmure-{}.wav",
                        uuid::Uuid::new_v4()
                    ));

                    if let Err(e) = std::fs::write(&temp_path, bytes) {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ErrorResponse {
                                error: format!("Failed to write audio file: {}", e),
                            }),
                        )
                            .into_response()
                    }

                    // Perform transcription
                    let result = match audio::preload_engine(&app) {
                        Ok(_) => match audio::transcribe_audio(&temp_path) {
                            Ok(raw_text) => {
                                // Fix transcription with dictionary
                                let text = match get_cc_rules_path(&app) {
                                    Ok(cc_rules_path) => {
                                        let dictionary = app.state::<Dictionary>().get();
                                        fix_transcription_with_dictionary(
                                            raw_text,
                                            dictionary,
                                            cc_rules_path,
                                        )
                                    }
                                    Err(_) => raw_text, // Use raw text if rules not available
                                };

                                Ok(text)
                            }
                            Err(e) => Err(format!("Transcription failed: {}", e)),
                        },
                        Err(e) => Err(format!("Model not available: {}", e)),
                    };

                    // Clean up temporary file
                    let _ = std::fs::remove_file(&temp_path);

                    return match result {
                        Ok(text) => (
                            StatusCode::OK,
                            Json(TranscriptionResponse { text }),
                        )
                            .into_response(),
                        Err(e) => (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ErrorResponse { error: e }),
                        )
                            .into_response(),
                    };
                }
            }
            Ok(None) => break,
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: format!("Failed to parse multipart: {}", e),
                    }),
                )
                    .into_response()
            }
        }
    }

    (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            error: "No 'audio' field in multipart request".to_string(),
        }),
    )
        .into_response()
}
