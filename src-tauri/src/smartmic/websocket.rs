use super::audio_bridge;
use super::input_bridge;
use super::pairing;
use super::types::{
    ClientMessage, ConnectedDevice, PairedDevice, ServerMessage, SmartMicMode, SmartMicState,
};
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
    // Token already validated in server.rs before WebSocket upgrade

    // Create channel and device name BEFORE locking to avoid holding lock during await
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();
    let device_name = pairing::device_name_from_token(&token);

    // Atomic check-and-set in a single lock scope (no TOCTOU race)
    let registered = {
        let mut connected = state.connected_device.lock().unwrap();
        if connected.is_some() {
            false
        } else {
            *connected = Some(ConnectedDevice {
                token: token.clone(),
                name: device_name.clone(),
                tx: tx.clone(),
            });
            true
        }
    };

    if !registered {
        let msg = ServerMessage::Error {
            message: "Another device is already connected".to_string(),
        };
        let _ = socket.send(Message::Text(msg.to_json().into())).await;
        return;
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
    let _ = socket
        .send(Message::Text(status_msg.to_json().into()))
        .await;

    // Send available LLM modes
    let llm_mode_names = crate::llm::helpers::load_llm_connect_settings(&app)
        .modes
        .iter()
        .map(|m| m.name.clone())
        .collect::<Vec<_>>();
    let modes_msg = ServerMessage::Modes {
        modes: llm_mode_names,
    };
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
                                if matches!(client_msg, ClientMessage::RecStart { .. }) {
                                    last_mic_level_time = std::time::Instant::now();
                                }
                                handle_client_message(
                                    &client_msg,
                                    &app,
                                    &state,
                                    &tx,
                                    &mut is_recording,
                                    &token,
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
                            let accepted = audio_bridge::accumulate_pcm(&mut buffer, payload);

                            if !accepted {
                                // Buffer full - stop recording and notify client
                                drop(buffer);
                                is_recording = false;
                                let err_msg = ServerMessage::Error {
                                    message: "Recording buffer full (max 5 minutes)".to_string(),
                                };
                                let _ = tx.send(err_msg.to_json());
                                let status_msg = ServerMessage::Status { recording: false };
                                let _ = tx.send(status_msg.to_json());
                            } else {
                                // Send mic level periodically (every 100ms max)
                                if last_mic_level_time.elapsed() >= std::time::Duration::from_millis(100) {
                                    let level = audio_bridge::calculate_rms(&buffer[buffer.len().saturating_sub(1600)..]);
                                    let level_msg = ServerMessage::MicLevel { level };
                                    let _ = tx.send(level_msg.to_json());
                                    last_mic_level_time = std::time::Instant::now();
                                }
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
    connection_token: &str,
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
                *rec_mode = SmartMicMode::from_client(mode);
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

            let smartmic_mode = {
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
                "SmartMic recording stopped, processing {} samples (mode: {:?})",
                buffer.len(),
                smartmic_mode
            );

            let app_clone = app.clone();
            let tx_clone = tx.clone();

            tokio::task::spawn_blocking(move || {
                process_recording(app_clone, tx_clone, buffer, smartmic_mode, sample_rate);
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
        ClientMessage::Pair { token: _ } => {
            // Ignore token from client message — use the authenticated connection token
            let device = PairedDevice {
                token: connection_token.to_string(),
                name: pairing::device_name_from_token(connection_token),
                last_connected: chrono::Utc::now().to_rfc3339(),
            };
            if let Err(e) = pairing::add_paired_device(state, app, device) {
                warn!("SmartMic: Failed to persist paired device: {}", e);
            }
        }
    }
}

/// Process a completed SmartMic recording: resample, transcribe, paste, and notify.
fn process_recording(
    app: Arc<tauri::AppHandle>,
    tx: mpsc::UnboundedSender<String>,
    buffer: Vec<i16>,
    mode: SmartMicMode,
    sample_rate: u32,
) {
    let samples = audio_bridge::finalize_buffer(buffer, sample_rate);
    let recording_mode = mode.to_recording_mode();

    // Switch LLM mode if needed
    if let SmartMicMode::Llm(idx) = &mode {
        crate::llm::llm::switch_active_mode(&app, *idx);
    }

    // Ensure engine is loaded
    if let Err(e) = crate::audio::preload_engine(&app) {
        error!("SmartMic: Failed to preload engine: {}", e);
        let err_msg = ServerMessage::Error {
            message: "Transcription failed: model not available".to_string(),
        };
        let _ = tx.send(err_msg.to_json());
        return;
    }

    match crate::audio::pipeline::process_recording_from_samples(&app, samples, recording_mode) {
        Ok(text) => {
            info!("SmartMic transcription result: {}", text);

            if !text.is_empty() {
                if let Err(e) = crate::clipboard::paste(&text, &app) {
                    warn!("SmartMic: Failed to paste text: {}", e);
                }
            }

            let trans_msg = ServerMessage::Transcription { text: text.clone() };
            let _ = tx.send(trans_msg.to_json());
        }
        Err(e) => {
            error!("SmartMic transcription failed: {}", e);
            let err_msg = ServerMessage::Error {
                message: "Transcription failed".to_string(),
            };
            let _ = tx.send(err_msg.to_json());
        }
    }
}
