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
    let state = app.state::<SmartMicState>().inner().clone();

    if state
        .is_running
        .compare_exchange(
            false,
            true,
            std::sync::atomic::Ordering::SeqCst,
            std::sync::atomic::Ordering::SeqCst,
        )
        .is_err()
    {
        return Err("SmartMic server is already running".to_string());
    }

    let s = settings::load_settings(&app);
    let port = s.smartmic_port;
    let app_handle = app.clone();

    pairing::prepare_smartmic_state(&state, &app)?;
    spawn_smartmic_thread(app_handle, port, state, None);

    Ok(format!(
        "SmartMic HTTPS server starting on port {}",
        s.smartmic_port
    ))
}

#[command]
pub async fn stop_smartmic_server(app: AppHandle) -> Result<(), String> {
    let state = app.state::<SmartMicState>();
    state.stop();
    info!("SmartMic server stop signal sent");

    // Wait for server to actually stop (poll is_running with timeout)
    for _ in 0..20 {
        if !state.is_running.load(std::sync::atomic::Ordering::SeqCst) {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    Ok(()) // Timeout after 2s, proceed anyway
}

/// Resolve the PWA language from the current Murmure settings.
/// Returns `Some("en" | "fr")` if the user has chosen an explicit language,
/// `None` if settings are set to "default" so the QR URL omits `&lang=`.
fn resolve_pwa_lang(settings: &crate::settings::AppSettings) -> Option<&'static str> {
    match settings.language.as_str() {
        "en" => Some("en"),
        "fr" => Some("fr"),
        _ => None,
    }
}

#[command]
pub fn get_smartmic_qr_code(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    let state = app.state::<SmartMicState>();

    // Get the first paired device token
    let token = {
        let devices = state.paired_devices.lock();
        devices
            .first()
            .map(|d| d.token.clone())
            .ok_or_else(|| "No paired device token available".to_string())?
    };

    let lang = resolve_pwa_lang(&s);

    // Build base URL: relay mode or direct mode
    let relay_url = s.smartmic_relay_url.as_deref().unwrap_or("").trim();
    if !relay_url.is_empty() {
        let relay_url = relay_url.trim_end_matches('/');
        let base_url = if s.smartmic_machine_id_enabled {
            let machine_id = s
                .smartmic_machine_id
                .as_deref()
                .filter(|id| !id.trim().is_empty())
                .map(|id| id.to_string())
                .unwrap_or_else(|| {
                    hostname::get()
                        .ok()
                        .and_then(|h| h.into_string().ok())
                        .unwrap_or_else(|| "unknown".to_string())
                });
            format!("{}/{}/", relay_url, machine_id)
        } else {
            format!("{}/", relay_url)
        };
        qr::generate_qr_data_uri_from_base(&base_url, &token, lang)
            .map_err(|e| format!("Failed to generate QR code: {}", e))
    } else {
        let ip = qr::get_local_ip().map_err(|e| format!("Failed to get local IP: {}", e))?;
        qr::generate_qr_data_uri(&ip, s.smartmic_port, &token, lang)
            .map_err(|e| format!("Failed to generate QR code: {}", e))
    }
}

#[command]
pub fn get_smartmic_relay_url(app: AppHandle) -> Result<Option<String>, String> {
    let s = settings::load_settings(&app);
    Ok(s.smartmic_relay_url)
}

#[command]
pub fn set_smartmic_relay_url(app: AppHandle, url: Option<String>) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    s.smartmic_relay_url = url;
    settings::save_settings(&app, &s)
}

#[command]
pub fn get_smartmic_machine_id(app: AppHandle) -> Result<Option<String>, String> {
    let s = settings::load_settings(&app);
    Ok(s.smartmic_machine_id)
}

#[command]
pub fn set_smartmic_machine_id(app: AppHandle, id: Option<String>) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    s.smartmic_machine_id = id;
    settings::save_settings(&app, &s)
}

#[command]
pub fn get_smartmic_machine_id_enabled(app: AppHandle) -> Result<bool, String> {
    let s = settings::load_settings(&app);
    Ok(s.smartmic_machine_id_enabled)
}

#[command]
pub fn set_smartmic_machine_id_enabled(app: AppHandle, enabled: bool) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    s.smartmic_machine_id_enabled = enabled;
    settings::save_settings(&app, &s)
}

#[command]
pub fn get_smartmic_token_ttl_hours(app: AppHandle) -> Result<Option<u64>, String> {
    let s = settings::load_settings(&app);
    Ok(s.smartmic_token_ttl_hours)
}

#[command]
pub fn set_smartmic_token_ttl_hours(app: AppHandle, hours: Option<u64>) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    s.smartmic_token_ttl_hours = hours;
    settings::save_settings(&app, &s)
}

#[command]
pub fn get_machine_hostname() -> Result<String, String> {
    hostname::get()
        .map_err(|e| format!("Failed to get hostname: {}", e))
        .and_then(|h| {
            h.into_string()
                .map_err(|_| "Hostname contains invalid UTF-8".to_string())
        })
}

#[command]
pub fn get_paired_devices(app: AppHandle) -> Result<Vec<PairedDevice>, String> {
    let state = app.state::<SmartMicState>();
    let devices = state.paired_devices.lock();
    Ok(devices.clone())
}

#[command]
pub fn remove_paired_device(app: AppHandle, token: String) -> Result<(), String> {
    let state = app.state::<SmartMicState>();
    pairing::remove_paired_device(&state, &app, &token)
        .map_err(|e| format!("Failed to remove device: {}", e))
}

#[command]
pub fn reset_smartmic_tokens(app: AppHandle) -> Result<(), String> {
    let state = app.state::<SmartMicState>();
    pairing::reset_all_tokens(&state, &app)
        .map_err(|e| format!("Failed to reset tokens: {}", e))
}
