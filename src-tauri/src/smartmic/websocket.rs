use super::audio_bridge;
use super::input_bridge;
use super::pairing;
use super::types::{
    ClientMessage, ConnectedDevice, PairedDevice, ServerMessage, SmartMicMode, SmartMicState,
};
use axum::extract::ws::{Message, WebSocket};
use log::{debug, error, info, warn};
use std::sync::Arc;
use std::time::{Duration, Instant};
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
    let (tx, mut rx) = mpsc::channel::<String>(256);
    let device_name = pairing::device_name_from_token(&token);

    // Check if another device is already connected
    let existing_device_name = {
        let connected = state.connected_device.lock();
        connected.as_ref().map(|d| d.name.clone())
    };

    if let Some(existing_name) = existing_device_name {
        // Inform the new device that another device is connected
        let msg = ServerMessage::DeviceAlreadyConnected {
            device_name: existing_name,
        };
        let _ = socket.send(Message::Text(msg.to_json().into())).await;

        // Wait for ForceConnect or connection close
        let force = loop {
            match socket.recv().await {
                Some(Ok(Message::Text(text))) => {
                    if let Ok(ClientMessage::ForceConnect) = serde_json::from_str(&text) {
                        break true;
                    }
                }
                Some(Ok(Message::Close(_))) | None => {
                    break false;
                }
                Some(Err(_)) => {
                    break false;
                }
                _ => continue,
            }
        };

        if !force {
            return;
        }

        // Disconnect the old device
        {
            let mut connected = state.connected_device.lock();
            if let Some(old_device) = connected.take() {
                let _ = old_device.tx.try_send(ServerMessage::ForceDisconnect.to_json());
                info!(
                    "SmartMic force-disconnect: {} replaced by {}",
                    old_device.name, device_name
                );
            }
        }
    }

    // Register the new device
    {
        let mut connected = state.connected_device.lock();
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
            created_at: String::new(),
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

    let mut ping_interval = tokio::time::interval(Duration::from_secs(30));
    let mut last_activity = Instant::now();

    // Rate limiting: max 100 text messages per second per connection
    const RATE_LIMIT_MAX: u32 = 100;
    let mut rate_limit_count: u32 = 0;
    let mut rate_limit_window = Instant::now();

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
                        last_activity = Instant::now();

                        // Rate limiting: reset counter each second, drop excess messages
                        if rate_limit_window.elapsed() >= Duration::from_secs(1) {
                            rate_limit_count = 0;
                            rate_limit_window = Instant::now();
                        }
                        rate_limit_count += 1;
                        if rate_limit_count > RATE_LIMIT_MAX {
                            if rate_limit_count == RATE_LIMIT_MAX + 1 {
                                warn!("SmartMic rate limit hit for {}: dropping messages", device_name);
                            }
                            continue;
                        }

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
                        last_activity = Instant::now();
                        if data.is_empty() {
                            continue;
                        }
                        // Check header byte 0x01 for audio data
                        if data[0] == 0x01 && is_recording {
                            let payload = &data[1..];
                            let mut buffer = state.recording_buffer.lock();
                            let accepted = audio_bridge::accumulate_pcm(&mut buffer, payload);

                            if !accepted {
                                // Buffer full - stop recording and notify client
                                drop(buffer);
                                is_recording = false;
                                let err_msg = ServerMessage::Error {
                                    message: "Recording buffer full (max 5 minutes)".to_string(),
                                };
                                let _ = tx.try_send(err_msg.to_json());
                                let status_msg = ServerMessage::Status { recording: false };
                                let _ = tx.try_send(status_msg.to_json());
                            } else {
                                // Send mic level periodically (every 100ms max)
                                if last_mic_level_time.elapsed() >= std::time::Duration::from_millis(100) {
                                    let level = audio_bridge::calculate_rms(&buffer[buffer.len().saturating_sub(1600)..]);
                                    let level_msg = ServerMessage::MicLevel { level };
                                    let _ = tx.try_send(level_msg.to_json());
                                    last_mic_level_time = std::time::Instant::now();
                                }
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => {
                        break;
                    }
                    Some(Ok(_)) => {
                        last_activity = Instant::now();
                    }
                    Some(Err(e)) => {
                        warn!("SmartMic WebSocket error ({}): {}", device_name, e);
                        break;
                    }
                }
            }
            // Ping keepalive: detect dead connections
            _ = ping_interval.tick() => {
                if last_activity.elapsed() > Duration::from_secs(60) {
                    warn!("SmartMic WebSocket timeout: {} inactive for 60s, closing connection", device_name);
                    break;
                }
                if socket.send(Message::Ping(vec![].into())).await.is_err() {
                    break;
                }
            }
        }
    }

    // Cleanup on disconnect
    info!("SmartMic device disconnected: {}", device_name);

    // Cancel any ongoing recording
    if is_recording {
        let mut buffer = state.recording_buffer.lock();
        buffer.clear();
    }

    // Remove connected device only if it's still us
    {
        let mut connected = state.connected_device.lock();
        if let Some(ref device) = *connected {
            if device.token == token {
                *connected = None;
            }
        }
    }
}

/// Handle a parsed client message
async fn handle_client_message(
    msg: &ClientMessage,
    app: &Arc<tauri::AppHandle>,
    state: &SmartMicState,
    tx: &mpsc::Sender<String>,
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
        ClientMessage::KeyPress { key } => {
            if let Err(e) = input_bridge::key_press(key) {
                warn!("SmartMic key press failed: {}", e);
            }
        }
        ClientMessage::RecStart { mode, paste, source_lang, target_lang } => {
            let paste = *paste;
            *is_recording = true;
            // Clear buffer
            {
                let mut buffer = state.recording_buffer.lock();
                buffer.clear();
            }
            // Set mode and paste flag
            {
                let mut rec_mode = state.recording_mode.lock();
                *rec_mode = SmartMicMode::from_client(mode, source_lang.clone(), target_lang.clone());
            }
            state.paste_enabled.store(paste, std::sync::atomic::Ordering::SeqCst);

            let status_msg = ServerMessage::Status { recording: true };
            let _ = tx.try_send(status_msg.to_json());
            info!("SmartMic recording started (mode: {}, paste: {})", mode, paste);
        }
        ClientMessage::RecStop => {
            *is_recording = false;

            let status_msg = ServerMessage::Status { recording: false };
            let _ = tx.try_send(status_msg.to_json());

            // Take buffer and process
            let buffer: Vec<i16> = {
                let mut buf = state.recording_buffer.lock();
                std::mem::take(&mut *buf)
            };

            let smartmic_mode = {
                let mode = state.recording_mode.lock();
                mode.clone()
            };

            let sample_rate = {
                let sr = state.sample_rate.lock();
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
            let should_paste = state.paste_enabled.load(std::sync::atomic::Ordering::SeqCst);

            tokio::task::spawn_blocking(move || {
                process_recording(app_clone, tx_clone, buffer, smartmic_mode, sample_rate, should_paste);
            });
        }
        ClientMessage::RecCancel => {
            *is_recording = false;
            {
                let mut buffer = state.recording_buffer.lock();
                buffer.clear();
            }
            let status_msg = ServerMessage::Status { recording: false };
            let _ = tx.try_send(status_msg.to_json());
            info!("SmartMic recording cancelled");
        }
        ClientMessage::Pair { token: _ } => {
            // Ignore token from client message — use the authenticated connection token
            let device = PairedDevice {
                token: connection_token.to_string(),
                name: pairing::device_name_from_token(connection_token),
                last_connected: chrono::Utc::now().to_rfc3339(),
                created_at: chrono::Utc::now().to_rfc3339(),
            };
            if let Err(e) = pairing::add_paired_device(state, app, device) {
                warn!("SmartMic: Failed to persist paired device: {}", e);
            }
        }
        ClientMessage::ForceConnect => {
            // ForceConnect is handled during connection phase, ignore if received during normal operation
        }
    }
}

/// Map a language code to its English name for translation prompts.
fn lang_code_to_name(code: &str) -> &'static str {
    match code {
        "bg" => "Bulgarian", "hr" => "Croatian", "cs" => "Czech",
        "da" => "Danish", "nl" => "Dutch", "en" => "English",
        "et" => "Estonian", "fi" => "Finnish", "fr" => "French",
        "de" => "German", "el" => "Greek", "hu" => "Hungarian",
        "it" => "Italian", "lv" => "Latvian", "lt" => "Lithuanian",
        "mt" => "Maltese", "pl" => "Polish", "pt" => "Portuguese",
        "ro" => "Romanian", "ru" => "Russian", "sk" => "Slovak",
        "sl" => "Slovenian", "es" => "Spanish", "sv" => "Swedish",
        "uk" => "Ukrainian", _ => "Unknown",
    }
}

/// Process a completed SmartMic recording: resample, transcribe, optionally paste, and notify.
fn process_recording(
    app: Arc<tauri::AppHandle>,
    tx: mpsc::Sender<String>,
    buffer: Vec<i16>,
    mode: SmartMicMode,
    sample_rate: u32,
    should_paste: bool,
) {
    let samples = audio_bridge::finalize_buffer(buffer, sample_rate);

    // For Translation mode: always transcribe in Standard mode first
    let recording_mode = match &mode {
        SmartMicMode::Translation { .. } => crate::audio::types::RecordingMode::Standard,
        _ => mode.to_recording_mode(),
    };

    // Switch LLM mode if needed (not for Translation)
    if let SmartMicMode::Llm(idx) = &mode {
        crate::llm::llm::switch_active_mode(&app, *idx);
    }

    // Ensure engine is loaded
    if let Err(e) = crate::audio::preload_engine(&app) {
        error!("SmartMic: Failed to preload engine: {}", e);
        let err_msg = ServerMessage::Error {
            message: "Transcription failed: model not available".to_string(),
        };
        let _ = tx.try_send(err_msg.to_json());
        return;
    }

    match crate::audio::pipeline::process_recording_from_samples(&app, samples, recording_mode) {
        Ok(text) => {
            debug!("SmartMic transcription result: {}", text);

            // For Translation mode: translate via LLM
            let final_text = if let SmartMicMode::Translation { source_lang, target_lang } = &mode {
                if text.trim().is_empty() {
                    text
                } else {
                    let system_prompt = format!(
                        "You are a translator. Translate the following {} text to {}. Output ONLY the translation, nothing else. No explanations, no quotes, no formatting.",
                        lang_code_to_name(source_lang), lang_code_to_name(target_lang)
                    );
                    let rt = tokio::runtime::Runtime::new();
                    match rt {
                        Ok(rt) => {
                            match rt.block_on(crate::llm::process_command_with_llm(&app, system_prompt, text.clone())) {
                                Ok(translated) => {
                                    debug!("SmartMic translation result: {}", translated);
                                    translated
                                }
                                Err(e) => {
                                    warn!("SmartMic translation failed: {}. Returning original.", e);
                                    text
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Failed to create runtime for translation: {}", e);
                            text
                        }
                    }
                }
            } else {
                text
            };

            if !final_text.is_empty() && should_paste {
                if let Err(e) = crate::clipboard::paste(&final_text, &app) {
                    warn!("SmartMic: Failed to paste text: {}", e);
                }
            }

            let trans_msg = ServerMessage::Transcription { text: final_text };
            let _ = tx.try_send(trans_msg.to_json());
        }
        Err(e) => {
            error!("SmartMic transcription failed: {}", e);
            let err_msg = ServerMessage::Error {
                message: "Transcription failed".to_string(),
            };
            let _ = tx.try_send(err_msg.to_json());
        }
    }
}
