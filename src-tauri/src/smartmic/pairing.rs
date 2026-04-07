use super::types::{PairedDevice, SmartMicState};
use anyhow::{Context, Result};
use log::info;
use std::path::PathBuf;

/// Generate a new UUID v4 token
pub fn generate_token() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Build a human-readable device name from a token.
pub fn device_name_from_token(token: &str) -> String {
    format!("SmartMic-{}", token.get(..8).unwrap_or(token))
}

/// Validate that a token exists in the paired devices list
pub fn validate_token(state: &SmartMicState, token: &str) -> bool {
    let devices = state.paired_devices.lock().unwrap();
    devices.iter().any(|d| d.token == token)
}

/// Add a paired device and persist to disk
pub fn add_paired_device(
    state: &SmartMicState,
    app: &tauri::AppHandle,
    device: PairedDevice,
) -> Result<()> {
    let mut devices = state.paired_devices.lock().unwrap();

    // Update existing device or add new one
    if let Some(existing) = devices.iter_mut().find(|d| d.token == device.token) {
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
    let mut devices = state.paired_devices.lock().unwrap();
    devices.retain(|d| d.token != token);

    // If the connected device has this token, disconnect it
    let mut connected = state.connected_device.lock().unwrap();
    if let Some(ref dev) = *connected {
        if dev.token == token {
            *connected = None;
        }
    }

    save_paired_devices(app, &devices)
}

/// Load paired devices from disk
pub fn load_paired_devices(app: &tauri::AppHandle) -> Result<Vec<PairedDevice>> {
    let path = paired_devices_path(app)?;

    if !path.exists() {
        return Ok(Vec::new());
    }

    let content = std::fs::read_to_string(&path).context("Failed to read paired_devices.json")?;
    let devices: Vec<PairedDevice> =
        serde_json::from_str(&content).context("Failed to parse paired_devices.json")?;

    info!("Loaded {} paired SmartMic device(s)", devices.len());
    Ok(devices)
}

/// Save paired devices to disk
pub fn save_paired_devices(app: &tauri::AppHandle, devices: &[PairedDevice]) -> Result<()> {
    let path = paired_devices_path(app)?;
    let content =
        serde_json::to_string_pretty(devices).context("Failed to serialize paired devices")?;
    std::fs::write(&path, content).context("Failed to write paired_devices.json")?;
    Ok(())
}

/// Load paired devices into state and ensure at least one token exists for pairing.
pub fn prepare_smartmic_state(
    state: &SmartMicState,
    app: &tauri::AppHandle,
) -> Result<(), String> {
    // Load paired devices into state
    match load_paired_devices(app) {
        Ok(devices) => {
            let mut paired = state.paired_devices.lock().unwrap();
            *paired = devices;
        }
        Err(e) => {
            info!("No paired devices loaded: {}", e);
        }
    }

    // Ensure at least one token exists for pairing
    {
        let paired = state.paired_devices.lock().unwrap();
        if paired.is_empty() {
            drop(paired);
            let token = generate_token();
            let device = PairedDevice {
                token,
                name: "Initial pairing token".to_string(),
                last_connected: String::new(),
            };
            add_paired_device(state, app, device)
                .map_err(|e| format!("Failed to create initial pairing token: {}", e))?;
        }
    }

    Ok(())
}

/// Get the path to paired_devices.json
fn paired_devices_path(app: &tauri::AppHandle) -> Result<PathBuf> {
    let dir = super::smartmic_data_dir(app)?;
    Ok(dir.join("paired_devices.json"))
}
