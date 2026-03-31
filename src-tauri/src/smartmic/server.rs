use super::types::SmartMicState;
use super::websocket;
use anyhow::Result;
use axum::{
    extract::{Query, State, WebSocketUpgrade},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use log::info;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tauri::Manager;

/// Shared state for the Axum server
#[derive(Clone)]
pub struct ServerState {
    pub app: Arc<tauri::AppHandle>,
    pub smartmic: SmartMicState,
}

/// Start the SmartMic HTTPS server
pub async fn start_smartmic_server(
    app: tauri::AppHandle,
    port: u16,
    smartmic_state: SmartMicState,
) -> Result<()> {
    let app = Arc::new(app);

    let server_state = ServerState {
        app: app.clone(),
        smartmic: smartmic_state.clone(),
    };

    let router = Router::new()
        .route("/", get(serve_index))
        .route("/manifest.json", get(serve_manifest))
        .route("/sw.js", get(serve_sw))
        .route("/ws", get(ws_upgrade))
        .with_state(server_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // Get TLS certificate
    let (cert_der, key_der) = super::cert::get_or_create_cert(&app)?;

    let rustls_config = axum_server::tls_rustls::RustlsConfig::from_der(
        vec![cert_der],
        key_der,
    )
    .await?;

    info!("SmartMic HTTPS server listening on https://0.0.0.0:{}", port);

    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    smartmic_state.set_shutdown_sender(shutdown_tx);

    let server = axum_server::bind_rustls(addr, rustls_config).serve(router.into_make_service());

    tokio::select! {
        result = server => {
            if let Err(e) = result {
                return Err(anyhow::anyhow!("SmartMic server error: {}", e));
            }
            info!("SmartMic HTTPS server ended normally");
        }
        _ = shutdown_rx => {
            info!("SmartMic HTTPS server shutdown signal received");
        }
    }

    Ok(())
}

/// Serve index.html
async fn serve_index(State(state): State<ServerState>) -> impl IntoResponse {
    serve_resource(&state.app, "smartmic/index.html", "text/html; charset=utf-8")
}

/// Serve manifest.json
async fn serve_manifest(State(state): State<ServerState>) -> impl IntoResponse {
    serve_resource(
        &state.app,
        "smartmic/manifest.json",
        "application/manifest+json",
    )
}

/// Serve service worker
async fn serve_sw(State(state): State<ServerState>) -> impl IntoResponse {
    serve_resource(
        &state.app,
        "smartmic/sw.js",
        "application/javascript",
    )
}

/// WebSocket upgrade handler — validates token BEFORE upgrading the connection
async fn ws_upgrade(
    ws: WebSocketUpgrade,
    Query(params): Query<HashMap<String, String>>,
    State(state): State<ServerState>,
) -> Response {
    let token = match params.get("token") {
        Some(t) => t.clone(),
        None => {
            return (StatusCode::BAD_REQUEST, "Missing token parameter").into_response();
        }
    };

    if !super::pairing::validate_token(&state.smartmic, &token) {
        return (StatusCode::UNAUTHORIZED, "Invalid token").into_response();
    }

    ws.on_upgrade(move |socket| {
        websocket::handle_websocket(socket, token, state.app, state.smartmic)
    })
}

/// Read a resource file from the app's resource directory.
/// Only serves explicitly allowed paths — rejects any path traversal.
fn serve_resource(
    app: &Arc<tauri::AppHandle>,
    relative_path: &str,
    content_type: &str,
) -> Response {
    if relative_path.contains("..") {
        return (StatusCode::FORBIDDEN, "Forbidden").into_response();
    }

    let resource_path = crate::utils::resources::resolve_resource_path(app, relative_path);

    match resource_path {
        Some(path) if path.exists() => match std::fs::read(&path) {
            Ok(content) => (
                StatusCode::OK,
                [(header::CONTENT_TYPE, content_type)],
                content,
            )
                .into_response(),
            Err(e) => {
                log::error!("Failed to read resource {}: {}", relative_path, e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
        },
        _ => {
            log::error!("Resource not found: {}", relative_path);
            (StatusCode::NOT_FOUND, "Not found").into_response()
        }
    }
}
