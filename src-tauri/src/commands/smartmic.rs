use crate::settings;
use crate::smartmic::pairing;
use crate::smartmic::qr;
use crate::smartmic::types::PairedDevice;
use crate::smartmic::{spawn_smartmic_thread, SmartMicState};
use log::info;
use tauri::{command, AppHandle, Manager};

#[command]
pub fn get_smartmic_enabled(app: AppHandle) -> Result<bool, String> {
    let s = settings::load_settings(&app);
    Ok(s.smartmic_enabled)
}

#[command]
pub fn set_smartmic_enabled(app: AppHandle, enabled: bool) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    s.smartmic_enabled = enabled;
    settings::save_settings(&app, &s)
}

#[command]
pub fn get_smartmic_port(app: AppHandle) -> Result<u16, String> {
    let s = settings::load_settings(&app);
    Ok(s.smartmic_port)
}

#[command]
pub fn set_smartmic_port(app: AppHandle, port: u16) -> Result<(), String> {
    if !(1024..=65535).contains(&port) {
        return Err("Port must be between 1024 and 65535".to_string());
    }
    let mut s = settings::load_settings(&app);
    s.smartmic_port = port;
    settings::save_settings(&app, &s)
}

#[command]
pub fn start_smartmic_server(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    let port = s.smartmic_port;
    let app_handle = app.clone();
    let state = app.state::<SmartMicState>().inner().clone();

    // Load paired devices into state
    match pairing::load_paired_devices(&app) {
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
            let token = pairing::generate_token();
            let device = PairedDevice {
                token,
                name: "Initial pairing token".to_string(),
                last_connected: String::new(),
            };
            pairing::add_paired_device(&state, &app, device)
                .map_err(|e| format!("Failed to create initial pairing token: {}", e))?;
        }
    }

    spawn_smartmic_thread(app_handle, port, state);

    Ok(format!(
        "SmartMic HTTPS server starting on port {}",
        s.smartmic_port
    ))
}

#[command]
pub fn stop_smartmic_server(app: AppHandle) -> Result<(), String> {
    let state = app.state::<SmartMicState>();
    state.stop();
    info!("SmartMic server stop signal sent");
    Ok(())
}

#[command]
pub fn get_smartmic_qr_code(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    let state = app.state::<SmartMicState>();

    let ip = qr::get_local_ip().map_err(|e| format!("Failed to get local IP: {}", e))?;

    // Get the first paired device token
    let token = {
        let devices = state.paired_devices.lock().unwrap();
        devices
            .first()
            .map(|d| d.token.clone())
            .ok_or_else(|| "No paired device token available".to_string())?
    };

    qr::generate_qr_data_uri(&ip, s.smartmic_port, &token)
        .map_err(|e| format!("Failed to generate QR code: {}", e))
}

#[command]
pub fn get_paired_devices(app: AppHandle) -> Result<Vec<PairedDevice>, String> {
    let state = app.state::<SmartMicState>();
    let devices = state.paired_devices.lock().unwrap();
    Ok(devices.clone())
}

#[command]
pub fn remove_paired_device(app: AppHandle, token: String) -> Result<(), String> {
    let state = app.state::<SmartMicState>();
    pairing::remove_paired_device(&state, &app, &token)
        .map_err(|e| format!("Failed to remove device: {}", e))
}
