use super::types::SmartMicState;
use super::websocket;
use anyhow::Result;
use axum::{
    extract::{Query, State, WebSocketUpgrade},
    http::{header, HeaderValue, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use log::info;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

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
        .route("/smartmic.js", get(serve_js))
        .route("/smartmic.css", get(serve_css))
        .route("/icon-192.png", get(serve_icon_192))
        .route("/icon-512.png", get(serve_icon_512))
        .route("/ws", get(ws_upgrade))
        .layer(middleware::from_fn(security_headers))
        .with_state(server_state);

    // Bind to the machine's local IP instead of 0.0.0.0 to reduce attack surface
    let local_ip: std::net::Ipv4Addr = super::qr::get_local_ip()
        .unwrap_or_else(|_| "0.0.0.0".to_string())
        .parse()
        .unwrap_or(std::net::Ipv4Addr::UNSPECIFIED);
    let addr = SocketAddr::from((local_ip, port));

    // Ensure TLS certificate exists and load it
    let (cert_path, key_path) = super::cert::ensure_cert(&app)?;

    let rustls_config =
        axum_server::tls_rustls::RustlsConfig::from_pem_file(&cert_path, &key_path).await?;

    // Bind with SO_REUSEADDR so the server can restart immediately
    // without waiting for TCP TIME_WAIT to expire
    let socket = socket2::Socket::new(
        socket2::Domain::IPV4,
        socket2::Type::STREAM,
        Some(socket2::Protocol::TCP),
    )?;
    socket.set_reuse_address(true)?;
    socket.bind(&addr.into())?;
    socket.listen(128)?;
    let listener: std::net::TcpListener = socket.into();
    listener.set_nonblocking(true)?;

    info!(
        "SmartMic HTTPS server listening on https://{}:{}",
        local_ip, port
    );

    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    smartmic_state.set_shutdown_sender(shutdown_tx);

    let server = axum_server::from_tcp_rustls(listener, rustls_config)
        .serve(router.into_make_service());

    tokio::select! {
        result = server => {
            smartmic_state.is_running.store(false, std::sync::atomic::Ordering::SeqCst);
            if let Err(e) = result {
                return Err(anyhow::anyhow!("SmartMic server error: {}", e));
            }
            info!("SmartMic HTTPS server ended normally");
        }
        _ = shutdown_rx => {
            smartmic_state.is_running.store(false, std::sync::atomic::Ordering::SeqCst);
            info!("SmartMic HTTPS server shutdown signal received");
        }
    }

    Ok(())
}

/// Serve index.html
async fn serve_index(State(state): State<ServerState>) -> impl IntoResponse {
    serve_resource(
        &state.app,
        "smartmic/index.html",
        "text/html; charset=utf-8",
    )
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
    serve_resource(&state.app, "smartmic/sw.js", "application/javascript")
}

/// Serve SmartMic React JS bundle
async fn serve_js(State(state): State<ServerState>) -> impl IntoResponse {
    serve_resource(&state.app, "smartmic/smartmic.js", "application/javascript")
}

/// Serve SmartMic React CSS bundle
async fn serve_css(State(state): State<ServerState>) -> impl IntoResponse {
    serve_resource(&state.app, "smartmic/smartmic.css", "text/css")
}

/// Serve PWA icon 192x192
async fn serve_icon_192(State(state): State<ServerState>) -> impl IntoResponse {
    serve_resource(&state.app, "smartmic/icon-192.png", "image/png")
}

/// Serve PWA icon 512x512
async fn serve_icon_512(State(state): State<ServerState>) -> impl IntoResponse {
    serve_resource(&state.app, "smartmic/icon-512.png", "image/png")
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

/// Add security headers to all responses
async fn security_headers(
    request: axum::http::Request<axum::body::Body>,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert("x-frame-options", HeaderValue::from_static("DENY"));
    headers.insert(
        "x-content-type-options",
        HeaderValue::from_static("nosniff"),
    );
    response
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
