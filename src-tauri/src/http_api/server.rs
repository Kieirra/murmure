use super::prompt_mode::{
    custom_prompt_handler, prompt_mode_prompts_handler, saved_prompt_handler,
};
use super::types::{CancelOnDrop, HttpApiState, TempWav, TranscribeState};
use crate::audio;
use anyhow::Result;
use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Multipart, Request},
    http::{header, HeaderName, HeaderValue, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use log::info;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::AppHandle;

const MURMURE_WEBVIEW_ORIGINS: [&str; 3] = [
    "tauri://localhost",
    "http://tauri.localhost",
    "https://tauri.localhost",
];
const DEV_SERVER_ORIGIN: &str = "http://localhost:1420";

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

fn is_local_origin(origin: &str) -> bool {
    let origin = origin.trim();
    if origin.eq_ignore_ascii_case("null") {
        return false;
    }
    let Ok(url) = url::Url::parse(origin) else {
        return false;
    };
    match url.host() {
        Some(url::Host::Domain(host)) => {
            host.eq_ignore_ascii_case("localhost") || host.eq_ignore_ascii_case("tauri.localhost")
        }
        Some(url::Host::Ipv4(ip)) => ip.is_loopback(),
        Some(url::Host::Ipv6(ip)) => ip.is_loopback(),
        None => false,
    }
}

async fn reject_foreign_origin(request: Request, next: Next) -> axum::response::Response {
    if header_allowed(&request, header::ORIGIN, is_local_origin) {
        next.run(request).await
    } else {
        error_response(StatusCode::FORBIDDEN, "Origin is not allowed".to_string())
    }
}

fn is_local_host(host: &str) -> bool {
    let host = host.trim();
    let name = match host.strip_prefix('[') {
        Some(bracketed) => match bracketed.split_once(']') {
            Some((name, _)) => name,
            None => return false,
        },
        None => host.split_once(':').map_or(host, |(name, _)| name),
    };
    name.eq_ignore_ascii_case("localhost")
        || name.parse::<IpAddr>().is_ok_and(|ip| ip.is_loopback())
}

async fn reject_foreign_host(request: Request, next: Next) -> Response {
    if header_allowed(&request, header::HOST, is_local_host) {
        next.run(request).await
    } else {
        error_response(StatusCode::FORBIDDEN, "Host is not allowed".to_string())
    }
}

fn header_allowed(request: &Request, header: HeaderName, is_allowed: fn(&str) -> bool) -> bool {
    match request.headers().get(header) {
        None => true,
        Some(value) => value.to_str().is_ok_and(is_allowed),
    }
}

fn is_murmure_webview_origin(origin: &str) -> bool {
    MURMURE_WEBVIEW_ORIGINS.contains(&origin)
        || (cfg!(debug_assertions) && origin == DEV_SERVER_ORIGIN)
}

async fn allow_murmure_webview_origin(request: Request, next: Next) -> Response {
    let webview_origin = request
        .headers()
        .get(header::ORIGIN)
        .filter(|value| value.to_str().is_ok_and(is_murmure_webview_origin))
        .cloned();
    let mut response = next.run(request).await;
    if let Some(origin) = webview_origin {
        let headers = response.headers_mut();
        headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin);
        headers.append(header::VARY, HeaderValue::from_static("Origin"));
    }
    response
}

pub async fn start_http_api(
    app: AppHandle,
    listener: std::net::TcpListener,
    api_state: HttpApiState,
) -> Result<()> {
    let state = TranscribeState {
        app: Arc::new(app),
        transcribe_lock: Arc::new(tokio::sync::Mutex::new(())),
    };

    let router = Router::new()
        .route("/api/transcribe", post(transcribe_handler))
        .route("/api/prompt-mode/custom", post(custom_prompt_handler))
        .route("/api/prompt-mode", post(saved_prompt_handler))
        .route("/api/prompt-mode/prompts", get(prompt_mode_prompts_handler))
        .with_state(state)
        .layer(middleware::from_fn(allow_murmure_webview_origin))
        .layer(middleware::from_fn(reject_foreign_origin))
        .layer(middleware::from_fn(reject_foreign_host))
        .layer(DefaultBodyLimit::max(100_000_000));

    let listener = tokio::net::TcpListener::from_std(listener)?;
    let addr = listener.local_addr()?;

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

fn write_temp_wav(path: &std::path::Path, bytes: &[u8]) -> Result<(), String> {
    use std::io::Write;
    let mut file =
        crate::audio::helpers::create_owner_only_file(path).map_err(|e| e.to_string())?;
    file.write_all(bytes).map_err(|e| e.to_string())
}

async fn transcribe_bytes(state: &TranscribeState, bytes: Bytes) -> Response {
    let job = run_transcription_job(
        state,
        uuid::Uuid::new_v4(),
        bytes,
        audio::transcribe_file_chunked_cancellable,
    );
    match job.await {
        Ok(Some(text)) => (StatusCode::OK, Json(TranscriptionResponse { text })).into_response(),
        Ok(None) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Transcription cancelled".to_string(),
        ),
        Err(e) => error_response(StatusCode::INTERNAL_SERVER_ERROR, e),
    }
}

pub(super) async fn run_transcription_job<T, F>(
    state: &TranscribeState,
    id: uuid::Uuid,
    bytes: Bytes,
    job: F,
) -> Result<Option<T>, String>
where
    T: Send + 'static,
    F: FnOnce(&AppHandle, &Path, &Arc<AtomicBool>) -> Result<Option<T>> + Send + 'static,
{
    let temp = TempWav(std::env::temp_dir().join(format!("murmure-{}.wav", id)));
    let short_id = short_request_id(&id);

    write_temp_wav(&temp.0, &bytes).map_err(|e| format!("Failed to write audio file: {}", e))?;

    let transcribe_guard = state.transcribe_lock.clone().lock_owned().await;

    let cancelled = Arc::new(AtomicBool::new(false));
    let _cancel_on_drop = CancelOnDrop(cancelled.clone());

    info!("HTTP API transcription {}: starting", short_id);
    let started = std::time::Instant::now();

    let app = state.app.clone();
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
        let result =
            job(&app, &temp.0, &cancelled).map_err(|e| format!("Transcription failed: {}", e))?;
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
        Ok(Ok(Some(result))) => {
            info!(
                "HTTP API transcription {}: done in {} ms",
                short_id,
                started.elapsed().as_millis()
            );
            Ok(Some(result))
        }
        Ok(result) => result,
        Err(e) => Err(format!("Transcription task failed: {}", e)),
    }
}

pub(super) fn short_request_id(id: &uuid::Uuid) -> String {
    let mut short_id = id.to_string();
    short_id.truncate(8);
    short_id
}

#[cfg(test)]
mod tests {
    use super::{is_local_host, is_local_origin, is_murmure_webview_origin};

    #[test]
    fn local_origins_are_allowed() {
        assert!(is_local_origin("tauri://localhost"));
        assert!(is_local_origin("http://127.0.0.1:3000"));
        assert!(is_local_origin("http://localhost:1420"));
        assert!(is_local_origin("https://tauri.localhost"));
        assert!(is_local_origin("http://[::1]"));
    }

    #[test]
    fn foreign_origins_are_rejected() {
        assert!(!is_local_origin("https://example.com"));
        assert!(!is_local_origin("null"));
        assert!(!is_local_origin("http://192.168.1.10"));
        assert!(!is_local_origin("not a url"));
    }

    #[test]
    fn local_hosts_are_allowed() {
        assert!(is_local_host("127.0.0.1:4800"));
        assert!(is_local_host("LOCALHOST:4800"));
        assert!(is_local_host("[::1]:4800"));
    }

    #[test]
    fn foreign_hosts_are_rejected() {
        assert!(!is_local_host("evil.example.com:4800"));
        assert!(!is_local_host("localhost.evil.com:4800"));
        assert!(!is_local_host(""));
    }

    #[test]
    fn only_murmure_webview_origins_get_cors_headers() {
        assert!(is_murmure_webview_origin("tauri://localhost"));
        assert!(is_murmure_webview_origin("http://tauri.localhost"));
        assert!(!is_murmure_webview_origin("http://localhost:3000"));
        assert!(!is_murmure_webview_origin("tauri://evil"));
    }
}
