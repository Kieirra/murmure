use cpal::traits::{DeviceTrait, HostTrait};
#[allow(unused_imports)]
use log::{debug, info, warn};
use std::collections::HashSet;
use tauri::Manager;

use super::types::MicInfo;

/// Lists available microphones.
/// On Linux, uses PulseAudio/PipeWire via `pactl` for clean device names.
/// On other platforms, uses CPAL device enumeration with filtering.
pub fn get_mic_list() -> Vec<MicInfo> {
    #[cfg(target_os = "linux")]
    {
        if let Some(mics) = list_sources_pactl() {
            return mics;
        }
        debug!("pactl unavailable, falling back to CPAL enumeration");
    }

    get_mic_list_cpal()
}

/// Resolves a mic_id to a CPAL Device for recording.
/// On Linux with manual selection, temporarily routes the default source
/// so CPAL records from the requested microphone during the active capture.
pub fn resolve_device_for_recording(
    mic_id: &str,
) -> Result<(cpal::Device, Option<String>), anyhow::Error> {
    #[allow(unused_variables)]
    let host = cpal::default_host();

    #[cfg(target_os = "linux")]
    {
        // Verify the source still exists before trying to use it
        if !is_pulse_source_available(mic_id) {
            return Err(anyhow::anyhow!("Selected microphone is unavailable"));
        }

        let previous_source = get_pulse_default_source();
        if previous_source.as_deref() != Some(mic_id) {
            set_pulse_default_source(mic_id);
        }
        // Small delay to let PipeWire apply the routing change
        std::thread::sleep(std::time::Duration::from_millis(50));

        // Verify PipeWire actually applied the change
        if let Some(current) = get_pulse_default_source() {
            if current != mic_id {
                warn!(
                    "PulseAudio source mismatch: expected {:?}, got {:?}",
                    mic_id, current
                );
                return Err(anyhow::anyhow!("Selected microphone is unavailable"));
            }
        }

        let device = host
            .default_input_device()
            .ok_or_else(|| anyhow::anyhow!("No default input device available"))?;

        Ok((device, previous_source.filter(|source| source != mic_id)))
    }

    #[cfg(not(target_os = "linux"))]
    {
        find_device_by_name(mic_id)
            .map(|device| (device, None))
            .ok_or_else(|| anyhow::anyhow!("Selected microphone is unavailable"))
    }
}

// ── PulseAudio/PipeWire enumeration (Linux) ──

#[cfg(target_os = "linux")]
fn list_sources_pactl() -> Option<Vec<MicInfo>> {
    let output = std::process::Command::new("pactl")
        .args(["-f", "json", "list", "sources"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let json_str = String::from_utf8(output.stdout).ok()?;
    let sources: Vec<serde_json::Value> = serde_json::from_str(&json_str).ok()?;

    let mut mics = Vec::new();
    let mut seen_labels = HashSet::new();

    for source in &sources {
        let props = match source.get("properties").and_then(|p| p.as_object()) {
            Some(p) => p,
            None => continue,
        };
        let device_class = props
            .get("device.class")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if device_class != "sound" {
            continue;
        }

        let name = source.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let description = props
            .get("device.description")
            .and_then(|v| v.as_str())
            .unwrap_or(name);

        if name.is_empty() {
            continue;
        }

        let label = description.to_string();
        if seen_labels.insert(label.clone()) {
            debug!("Mic accepted (pactl): {} (source: {})", label, name);
            mics.push(MicInfo {
                id: name.to_string(),
                label,
            });
        }
    }

    // Fall back to CPAL if no valid sources found
    if mics.is_empty() {
        return None;
    }

    Some(mics)
}

#[cfg(target_os = "linux")]
fn is_pulse_source_available(source_name: &str) -> bool {
    let output = match std::process::Command::new("pactl")
        .args(["-f", "json", "list", "sources", "short"])
        .output()
    {
        Ok(o) if o.status.success() => o,
        _ => return true, // If pactl fails, don't block recording
    };

    let json_str = match String::from_utf8(output.stdout) {
        Ok(s) => s,
        Err(_) => return true,
    };

    let sources: Vec<serde_json::Value> = match serde_json::from_str(&json_str) {
        Ok(s) => s,
        Err(_) => return true,
    };

    sources
        .iter()
        .any(|s| s.get("name").and_then(|v| v.as_str()) == Some(source_name))
}

#[cfg(target_os = "linux")]
fn get_pulse_default_source() -> Option<String> {
    let output = std::process::Command::new("pactl")
        .args(["get-default-source"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[cfg(target_os = "linux")]
fn set_pulse_default_source(source_name: &str) {
    match std::process::Command::new("pactl")
        .args(["set-default-source", source_name])
        .output()
    {
        Ok(output) if output.status.success() => {
            debug!("Set PulseAudio default source: {}", source_name);
        }
        Ok(output) => {
            warn!(
                "Failed to set PulseAudio default source: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Err(e) => {
            warn!("pactl not available: {}", e);
        }
    }
}

pub fn restore_default_source_after_recording(
    #[allow(unused_variables)] previous_source: Option<String>,
) {
    #[cfg(target_os = "linux")]
    if let Some(source_name) = previous_source {
        set_pulse_default_source(&source_name);
        info!("Restored PulseAudio default source: {}", source_name);
    }
}

// ── CPAL-based enumeration (macOS/Windows fallback) ──

fn get_mic_list_cpal() -> Vec<MicInfo> {
    let host = cpal::default_host();
    let default_name = host
        .default_input_device()
        .and_then(|d| get_device_name(&d));

    match host.input_devices() {
        Ok(devices) => {
            let mut mics = Vec::new();
            let mut seen_names = HashSet::new();

            for device in devices {
                let name = get_device_name(&device);
                let driver = device
                    .description()
                    .ok()
                    .and_then(|d| d.driver().map(|s| s.to_string()));

                if !is_valid_input_device(&device) {
                    debug!(
                        "Mic filtered (invalid input): {:?} (driver: {:?})",
                        name, driver
                    );
                    continue;
                }
                if !is_relevant_device(&device) {
                    debug!(
                        "Mic filtered (not relevant): {:?} (driver: {:?})",
                        name, driver
                    );
                    continue;
                }

                if let Some(name) = name {
                    if seen_names.insert(name.clone()) {
                        debug!("Mic accepted: {} (driver: {:?})", name, driver);
                        mics.push(MicInfo {
                            id: name.clone(),
                            label: name,
                        });
                    }
                }
            }

            // Move default device to first position
            if let Some(ref default) = default_name {
                if let Some(pos) = mics.iter().position(|m| m.label == *default) {
                    let mic = mics.remove(pos);
                    mics.insert(0, mic);
                }
            }

            mics
        }
        Err(_) => Vec::new(),
    }
}

fn is_valid_input_device(device: &cpal::Device) -> bool {
    let configs = match device.supported_input_configs() {
        Ok(c) => c,
        Err(_) => return false,
    };

    for config in configs {
        let channels = config.channels();
        let format = config.sample_format();

        let valid_channels = channels == 1 || channels == 2;
        let valid_format = matches!(
            format,
            cpal::SampleFormat::I16 | cpal::SampleFormat::I32 | cpal::SampleFormat::F32
        );

        if valid_channels && valid_format {
            return true;
        }
    }

    false
}

fn is_relevant_device(device: &cpal::Device) -> bool {
    use cpal::{DeviceType, InterfaceType};

    let desc = match device.description() {
        Ok(d) => d,
        Err(_) => return false,
    };

    let device_type = desc.device_type();
    let interface_type = desc.interface_type();

    // On platforms with metadata (macOS/Windows), filter by type
    if device_type != DeviceType::Unknown || interface_type != InterfaceType::Unknown {
        if matches!(device_type, DeviceType::Virtual | DeviceType::Tuner) {
            return false;
        }
        if matches!(
            interface_type,
            InterfaceType::Network
                | InterfaceType::Aggregate
                | InterfaceType::Hdmi
                | InterfaceType::DisplayPort
                | InterfaceType::Spdif
        ) {
            return false;
        }
        return true;
    }

    // ALSA fallback: filter by PCM ID
    if let Some(driver) = desc.driver() {
        return driver.starts_with("sysdefault:");
    }

    true
}

fn get_device_name(device: &cpal::Device) -> Option<String> {
    device
        .description()
        .ok()
        .map(|desc| desc.name().to_string())
}

#[cfg(not(target_os = "linux"))]
fn find_device_by_name(name: &str) -> Option<cpal::Device> {
    let host = cpal::default_host();
    host.input_devices()
        .ok()?
        .find(|d| get_device_name(d).is_some_and(|n| n == name))
}

pub fn update_mic_cache(app: &tauri::AppHandle, mic_id: Option<String>) {
    let audio_state = app.state::<crate::audio::types::AudioState>();
    match mic_id {
        Some(ref id) => {
            #[cfg(not(target_os = "linux"))]
            {
                audio_state.set_cached_device(find_device_by_name(id));
            }

            #[cfg(target_os = "linux")]
            {
                audio_state.set_cached_device(None);
            }

            info!("Microphone selection updated: {}", id);
        }
        None => {
            audio_state.set_cached_device(None);
        }
    }
}

pub fn init_mic_cache_if_needed(app: &tauri::AppHandle, mic_id: Option<String>) {
    if let Some(id) = mic_id {
        #[cfg(not(target_os = "linux"))]
        {
            let app_handle = app.clone();
            std::thread::spawn(move || {
                if let Some(device) = find_device_by_name(&id) {
                    let audio_state = app_handle.state::<crate::audio::types::AudioState>();
                    audio_state.set_cached_device(Some(device));
                    info!("Microphone cache initialized: {}", id);
                }
            });
        }

        #[cfg(target_os = "linux")]
        {
            let _ = app;
            info!("Microphone configured: {}", id);
        }
    }
}
