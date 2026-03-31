use super::audio_bridge;
use super::input_bridge;
use super::pairing;
use super::types::{ClientMessage, ConnectedDevice, PairedDevice, ServerMessage, SmartMicState};
use axum::extract::ws::{Message, WebSocket};
use log::{error, info, warn};
use std::sync::Arc;
use tokio::sync::mpsc;

/// Handle a WebSocket connection from a smartphone client
pub async fn handle_websocket(
    mut socket: WebSocket,
    token: String,
    app: Arc<tauri::AppHandle>,
    state: SmartMicState,
) {
    // Validate token
    if !pairing::validate_token(&state, &token) {
        let msg = ServerMessage::Error {
            message: "Token invalide. Scannez a nouveau le QR code.".to_string(),
        };
        let _ = socket.send(Message::Text(msg.to_json().into())).await;
        return;
    }

    // Check if another device is already connected
    let already_connected = {
        let connected = state.connected_device.lock().unwrap();
        connected.is_some()
    };
    if already_connected {
        let msg = ServerMessage::Error {
            message: "Another device is already connected".to_string(),
        };
        let _ = socket.send(Message::Text(msg.to_json().into())).await;
        return;
    }

    // Create channel for sending messages to this client
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    // Register connected device
    let device_name = format!("SmartMic-{}", token.get(..8).unwrap_or(&token));
    {
        let mut connected = state.connected_device.lock().unwrap();
        *connected = Some(ConnectedDevice {
            token: token.clone(),
            name: device_name.clone(),
            tx: tx.clone(),
        });
    }

    // Update paired device last_connected timestamp
    {
        let device = PairedDevice {
            token: token.clone(),
            name: device_name.clone(),
            last_connected: chrono::Utc::now().to_rfc3339(),
        };
        if let Err(e) = pairing::add_paired_device(&state, &app, device) {
            warn!("Failed to update paired device: {}", e);
        }
    }

    info!("SmartMic device connected: {}", device_name);

    // Send initial status
    let status_msg = ServerMessage::Status { recording: false };
    let _ = socket.send(Message::Text(status_msg.to_json().into())).await;

    // Send available LLM modes
    let llm_mode_names = crate::llm::helpers::load_llm_connect_settings(&app)
        .modes
        .iter()
        .map(|m| m.name.clone())
        .collect::<Vec<_>>();
    let modes_msg = ServerMessage::Modes { modes: llm_mode_names };
    let _ = socket.send(Message::Text(modes_msg.to_json().into())).await;

    let mut is_recording = false;
    let mut last_mic_level_time = std::time::Instant::now();

    // Main loop: handle incoming messages and outgoing messages
    loop {
        tokio::select! {
            // Outgoing messages from server to client
            msg = rx.recv() => {
                match msg {
                    Some(text) => {
                        if socket.send(Message::Text(text.into())).await.is_err() {
                            break;
                        }
                    }
                    None => break,
                }
            }
            // Incoming messages from client
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<ClientMessage>(&text) {
                            Ok(client_msg) => {
                                handle_client_message(
                                    &client_msg,
                                    &app,
                                    &state,
                                    &tx,
                                    &mut is_recording,
                                ).await;
                            }
                            Err(e) => {
                                warn!("Failed to parse SmartMic client message: {}", e);
                            }
                        }
                    }
                    Some(Ok(Message::Binary(data))) => {
                        if data.is_empty() {
                            continue;
                        }
                        // Check header byte 0x01 for audio data
                        if data[0] == 0x01 && is_recording {
                            let payload = &data[1..];
                            let mut buffer = state.recording_buffer.lock().unwrap();
                            audio_bridge::accumulate_pcm(&mut buffer, payload);

                            // Send mic level periodically (every 100ms max)
                            if last_mic_level_time.elapsed() >= std::time::Duration::from_millis(100) {
                                let level = audio_bridge::calculate_rms(&buffer[buffer.len().saturating_sub(1600)..]);
                                let level_msg = ServerMessage::MicLevel { level };
                                let _ = tx.send(level_msg.to_json());
                                last_mic_level_time = std::time::Instant::now();
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => {
                        break;
                    }
                    Some(Ok(_)) => {
                        // Ping/Pong handled automatically
                    }
                    Some(Err(e)) => {
                        warn!("SmartMic WebSocket error: {}", e);
                        break;
                    }
                }
            }
        }
    }

    // Cleanup on disconnect
    info!("SmartMic device disconnected: {}", device_name);

    // Cancel any ongoing recording
    if is_recording {
        let mut buffer = state.recording_buffer.lock().unwrap();
        buffer.clear();
    }

    // Remove connected device
    {
        let mut connected = state.connected_device.lock().unwrap();
        *connected = None;
    }
}

/// Handle a parsed client message
async fn handle_client_message(
    msg: &ClientMessage,
    app: &Arc<tauri::AppHandle>,
    state: &SmartMicState,
    tx: &mpsc::UnboundedSender<String>,
    is_recording: &mut bool,
) {
    match msg {
        ClientMessage::MouseMove { dx, dy } => {
            if let Err(e) = input_bridge::move_mouse(*dx, *dy) {
                warn!("SmartMic mouse move failed: {}", e);
            }
        }
        ClientMessage::Click { button } => {
            if let Err(e) = input_bridge::click(button) {
                warn!("SmartMic click failed: {}", e);
            }
        }
        ClientMessage::Scroll { dy } => {
            if let Err(e) = input_bridge::scroll(*dy) {
                warn!("SmartMic scroll failed: {}", e);
            }
        }
        ClientMessage::RecStart { mode } => {
            *is_recording = true;
            // Clear buffer
            {
                let mut buffer = state.recording_buffer.lock().unwrap();
                buffer.clear();
            }
            // Set mode
            {
                let mut rec_mode = state.recording_mode.lock().unwrap();
                *rec_mode = mode.clone();
            }

            let status_msg = ServerMessage::Status { recording: true };
            let _ = tx.send(status_msg.to_json());
            info!("SmartMic recording started (mode: {})", mode);
        }
        ClientMessage::RecStop => {
            *is_recording = false;

            let status_msg = ServerMessage::Status { recording: false };
            let _ = tx.send(status_msg.to_json());

            // Take buffer and process
            let buffer: Vec<i16> = {
                let mut buf = state.recording_buffer.lock().unwrap();
                std::mem::take(&mut *buf)
            };

            let mode_str = {
                let mode = state.recording_mode.lock().unwrap();
                mode.clone()
            };

            let sample_rate = {
                let sr = state.sample_rate.lock().unwrap();
                *sr
            };

            if buffer.is_empty() {
                info!("SmartMic recording stopped with empty buffer, skipping transcription");
                return;
            }

            info!(
                "SmartMic recording stopped, processing {} samples (mode: {})",
                buffer.len(),
                mode_str
            );

            let app_clone = app.clone();
            let tx_clone = tx.clone();

            // Spawn blocking transcription on a separate thread
            tokio::task::spawn_blocking(move || {
                let samples = audio_bridge::finalize_buffer(buffer, sample_rate);

                // Parse mode: "stt" -> Standard, "llm_N" -> switch to LLM mode N
                let recording_mode = if mode_str.starts_with("llm_") {
                    if let Ok(idx) = mode_str[4..].parse::<usize>() {
                        crate::llm::llm::switch_active_mode(&app_clone, idx);
                    }
                    crate::audio::types::RecordingMode::Llm
                } else {
                    crate::audio::types::RecordingMode::Standard
                };

                // Ensure engine is loaded
                if let Err(e) = crate::audio::preload_engine(&app_clone) {
                    error!("SmartMic: Failed to preload engine: {}", e);
                    let err_msg = ServerMessage::Error {
                        message: "Transcription failed: model not available".to_string(),
                    };
                    let _ = tx_clone.send(err_msg.to_json());
                    return;
                }

                match crate::audio::pipeline::process_recording_from_samples(
                    &app_clone,
                    samples,
                    recording_mode,
                ) {
                    Ok(text) => {
                        info!("SmartMic transcription result: {}", text);

                        // Inject text at cursor via enigo
                        if !text.is_empty() {
                            if let Err(e) = crate::clipboard::paste(&text, &app_clone) {
                                warn!("SmartMic: Failed to paste text: {}", e);
                            }
                        }

                        let trans_msg = ServerMessage::Transcription {
                            text: text.clone(),
                        };
                        let _ = tx_clone.send(trans_msg.to_json());
                    }
                    Err(e) => {
                        error!("SmartMic transcription failed: {}", e);
                        let err_msg = ServerMessage::Error {
                            message: format!("Transcription failed: {}", e),
                        };
                        let _ = tx_clone.send(err_msg.to_json());
                    }
                }
            });
        }
        ClientMessage::RecCancel => {
            *is_recording = false;
            {
                let mut buffer = state.recording_buffer.lock().unwrap();
                buffer.clear();
            }
            let status_msg = ServerMessage::Status { recording: false };
            let _ = tx.send(status_msg.to_json());
            info!("SmartMic recording cancelled");
        }
        ClientMessage::Pair { token } => {
            // Token validation is done at connection time, but we confirm pairing here
            let device = PairedDevice {
                token: token.clone(),
                name: format!("SmartMic-{}", &token[..8.min(token.len())]),
                last_connected: chrono::Utc::now().to_rfc3339(),
            };
            if let Err(e) = pairing::add_paired_device(state, app, device) {
                warn!("SmartMic: Failed to persist paired device: {}", e);
            }
        }
    }
}
