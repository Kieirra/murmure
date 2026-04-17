use super::types::{PairedDevice, ServerMessage, SmartMicState};
use anyhow::{Context, Result};
use chrono::Utc;
use log::{debug, info};
use tauri_plugin_store::StoreExt;

/// Generate a new UUID v4 token
pub fn generate_token() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Build a human-readable device name from a token.
pub fn device_name_from_token(token: &str) -> String {
    format!("SmartMic-{}", token.get(..8).unwrap_or(token))
}

/// Validate that a token exists in the paired devices list.
/// If a TTL is configured and the token has expired, the device is removed.
pub fn validate_token(state: &SmartMicState, app: &tauri::AppHandle, token: &str) -> bool {
    let settings = crate::settings::load_settings(app);
    let ttl_hours = settings.smartmic_token_ttl_hours.unwrap_or(0);

    let devices = state.paired_devices.lock();
    let device = match devices.iter().find(|d| d.token == token) {
        Some(d) => d.clone(),
        None => return false,
    };
    drop(devices);

    // Check TTL expiration
    if ttl_hours > 0 && !device.created_at.is_empty() {
        if let Ok(created) = chrono::DateTime::parse_from_rfc3339(&device.created_at) {
            let elapsed = Utc::now().signed_duration_since(created);
            if elapsed > chrono::TimeDelta::hours(ttl_hours as i64) {
                debug!("Token expired for device '{}' (TTL: {}h)", device.name, ttl_hours);
                let _ = remove_paired_device(state, app, token);
                return false;
            }
        }
    }

    true
}

/// Add a paired device and persist to disk
pub fn add_paired_device(
    state: &SmartMicState,
    app: &tauri::AppHandle,
    device: PairedDevice,
) -> Result<()> {
    let mut devices = state.paired_devices.lock();

    // Update existing device or add new one
    if let Some(existing) = devices.iter_mut().find(|d| d.token == device.token) {
        // Intentionally preserve existing created_at to keep TTL based on original pairing time
        existing.name = device.name;
        existing.last_connected = device.last_connected;
    } else {
        devices.push(device);
    }

    save_paired_devices(app, &devices)
}

/// Remove a paired device by token and persist to disk
pub fn remove_paired_device(
    state: &SmartMicState,
    app: &tauri::AppHandle,
    token: &str,
) -> Result<()> {
    // Lock paired_devices, mutate, save, then drop — BEFORE touching connected_device
    {
        let mut devices = state.paired_devices.lock();
        devices.retain(|d| d.token != token);
        save_paired_devices(app, &devices)?;
    }

    // Disconnect the device if it's currently connected
    {
        let mut connected = state.connected_device.lock();
        if let Some(ref dev) = *connected {
            if dev.token == token {
                let _ = dev.tx.try_send(
                    ServerMessage::Error {
                        message: "Device removed".to_string(),
                    }
                    .to_json(),
                );
                *connected = None;
            }
        }
    }

    Ok(())
}

/// Load paired devices from the Tauri store
pub fn load_paired_devices(app: &tauri::AppHandle) -> Result<Vec<PairedDevice>> {
    let store = app
        .store("smartmic_devices.json")
        .map_err(|e| anyhow::anyhow!("Failed to open smartmic store: {}", e))?;
    match store.get("paired_devices") {
        Some(value) => {
            let devices: Vec<PairedDevice> = serde_json::from_value(value)
                .context("Failed to parse paired devices from store")?;
            info!("Loaded {} paired SmartMic device(s)", devices.len());
            Ok(devices)
        }
        None => Ok(Vec::new()),
    }
}

/// Save paired devices to the Tauri store
pub fn save_paired_devices(app: &tauri::AppHandle, devices: &[PairedDevice]) -> Result<()> {
    let store = app
        .store("smartmic_devices.json")
        .map_err(|e| anyhow::anyhow!("Failed to open smartmic store: {}", e))?;
    let value = serde_json::to_value(devices).context("Failed to serialize paired devices")?;
    store.set("paired_devices", value);
    Ok(())
}

/// Reset all tokens: disconnect all devices, clear paired list, generate a fresh token.
pub fn reset_all_tokens(state: &SmartMicState, app: &tauri::AppHandle) -> Result<()> {
    // Disconnect any connected device
    {
        let mut connected = state.connected_device.lock();
        if let Some(ref dev) = *connected {
            let _ = dev.tx.try_send(
                ServerMessage::Error {
                    message: "Token reset".to_string(),
                }
                .to_json(),
            );
            *connected = None;
        }
    }

    // Clear all paired devices and generate a fresh token
    {
        let mut devices = state.paired_devices.lock();
        devices.clear();
        let token = generate_token();
        devices.push(PairedDevice {
            token,
            name: "Initial pairing token".to_string(),
            last_connected: String::new(),
            created_at: Utc::now().to_rfc3339(),
        });
        save_paired_devices(app, &devices)?;
    }

    info!("SmartMic tokens reset - all devices revoked");
    Ok(())
}

/// Load paired devices into state and ensure at least one token exists for pairing.
pub fn prepare_smartmic_state(state: &SmartMicState, app: &tauri::AppHandle) -> Result<(), String> {
    // Load paired devices into state
    match load_paired_devices(app) {
        Ok(devices) => {
            let mut paired = state.paired_devices.lock();
            *paired = devices;
        }
        Err(e) => {
            info!("No paired devices loaded: {}", e);
        }
    }

    // Ensure at least one token exists for pairing
    {
        let paired = state.paired_devices.lock();
        if paired.is_empty() {
            drop(paired);
            let token = generate_token();
            let device = PairedDevice {
                token,
                name: "Initial pairing token".to_string(),
                last_connected: String::new(),
                created_at: Utc::now().to_rfc3339(),
            };
            add_paired_device(state, app, device)
                .map_err(|e| format!("Failed to create initial pairing token: {}", e))?;
        }
    }

    Ok(())
}
