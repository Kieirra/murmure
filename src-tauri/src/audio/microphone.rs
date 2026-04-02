use cpal::traits::{DeviceTrait, HostTrait};
use log::{debug, info, warn};
use std::collections::{HashMap, HashSet};
use tauri::Manager;

use super::types::MicInfo;

const GENERIC_MIC_NAMES: &[&str] =
    &["microphone", "microfone", "microphone array", "line in", "default"];

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
        return find_device_by_identifier(mic_id)
            .map(|device| (device, None))
            .ok_or_else(|| anyhow::anyhow!("Selected microphone is unavailable"));
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
    let mut seen_ids = HashSet::new();

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

        if seen_ids.insert(name.to_string()) {
            let label = description.to_string();
            debug!("Mic accepted (pactl): {} (source: {})", label, name);
            mics.push(MicInfo {
                id: name.to_string(),
                label,
            });
        }
    }

    disambiguate_labels(&mut mics);

    let default_source = get_pulse_default_source();
    if let Some(ref default) = default_source {
        if let Some(pos) = mics.iter().position(|m| m.id == *default) {
            let mic = mics.remove(pos);
            mics.insert(0, mic);
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

pub fn restore_default_source_after_recording(previous_source: Option<String>) {
    #[cfg(target_os = "linux")]
    if let Some(source_name) = previous_source {
        set_pulse_default_source(&source_name);
        info!("Restored PulseAudio default source: {}", source_name);
    }
}

// ── CPAL-based enumeration (macOS/Windows fallback) ──

fn get_mic_list_cpal() -> Vec<MicInfo> {
    let host = cpal::default_host();
    let default_id = host
        .default_input_device()
        .and_then(|d| get_device_id(&d));

    match host.input_devices() {
        Ok(devices) => {
            let mut mics = Vec::new();
            let mut seen_ids = HashSet::new();

            for device in devices {
                let name = get_device_name(&device);
                let device_id = get_device_id(&device);
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

                if let Some(id) = device_id {
                    if seen_ids.insert(id.clone()) {
                        let label = get_device_label(&device, &id);
                        debug!("Mic accepted: {} (id: {}, driver: {:?})", label, id, driver);
                        mics.push(MicInfo { id, label });
                    }
                }
            }

            disambiguate_labels(&mut mics);

            // Move default device to first position
            if let Some(ref default) = default_id {
                if let Some(pos) = mics.iter().position(|m| m.id == *default) {
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
    let is_valid_format = |format: cpal::SampleFormat| {
        matches!(
            format,
            cpal::SampleFormat::I16
                | cpal::SampleFormat::I32
                | cpal::SampleFormat::F32
                | cpal::SampleFormat::U8
                | cpal::SampleFormat::U16
        )
    };

    if let Ok(configs) = device.supported_input_configs() {
        for config in configs {
            if config.channels() >= 1 && is_valid_format(config.sample_format()) {
                return true;
            }
        }
    }

    // Fallback: virtual devices (e.g. NVIDIA Broadcast) may fail on
    // supported_input_configs() but work via default_input_config().
    if let Ok(config) = device.default_input_config() {
        return is_valid_format(config.sample_format());
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
        if matches!(device_type, DeviceType::Tuner) {
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

/// Builds a human-readable label for a device using description metadata with fallbacks.
/// Priority: extended metadata > manufacturer/driver/address > raw name > fallback ID.
fn get_device_label(device: &cpal::Device, fallback_id: &str) -> String {
    let desc = match device.description() {
        Ok(d) => d,
        Err(_) => return fallback_id.to_string(),
    };

    let raw_name = desc.name().trim().to_string();
    let name = if raw_name.is_empty() {
        fallback_id.to_string()
    } else {
        raw_name
    };
    let name_lower = name.trim().to_lowercase();

    // Try extended metadata first — pick the most human-readable entry
    if let Some(best) = pick_best_extended(&desc, &name_lower) {
        let best_lower = best.trim().to_lowercase();
        if is_generic_mic_name(&name)
            || best_lower.contains(&name_lower)
            || name_lower.contains(&best_lower)
        {
            return best;
        }
        return format!("{} ({})", name, best);
    }

    // Fallback: combine manufacturer, driver, address
    let parts = collect_metadata_parts(&desc, &name_lower);
    if !parts.is_empty() {
        return format!("{} ({})", name, parts.join(" | "));
    }

    name
}

/// Scores extended metadata entries and returns the most human-readable one.
fn pick_best_extended(desc: &cpal::DeviceDescription, name_lower: &str) -> Option<String> {
    let mut best: Option<(String, i32)> = None;

    for extra in desc.extended() {
        let extra = extra.trim();
        if extra.is_empty() || extra.trim().to_lowercase() == *name_lower {
            continue;
        }

        let extra_lower = extra.trim().to_lowercase();
        let mut score: i32 = 0;
        if !looks_technical(extra) {
            score += 3;
        }
        if extra_lower.contains(name_lower) || name_lower.contains(&extra_lower) {
            score += 2;
        }
        if (4..=96).contains(&extra.len()) {
            score += 1;
        }

        if best.as_ref().map_or(true, |(_, s)| score > *s) {
            best = Some((extra.to_string(), score));
        }
    }

    best.map(|(s, _)| s)
}

/// Collects non-redundant metadata parts (manufacturer, driver, address).
fn collect_metadata_parts(desc: &cpal::DeviceDescription, name_lower: &str) -> Vec<String> {
    let mut parts: Vec<String> = Vec::new();

    for value in [desc.manufacturer(), desc.driver(), desc.address()] {
        if let Some(v) = value {
            let v = v.trim();
            if !v.is_empty() && v.trim().to_lowercase() != *name_lower {
                parts.push(v.to_string());
            }
        }
    }

    parts.sort();
    parts.dedup();
    parts
}

fn is_generic_mic_name(value: &str) -> bool {
    GENERIC_MIC_NAMES.contains(&value.trim().to_lowercase().as_str())
}

/// Detects GUIDs and hex-heavy strings that shouldn't appear in labels.
fn looks_technical(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.contains('{') && trimmed.contains('}') {
        return true;
    }
    let alnum_count = trimmed.chars().filter(|c| c.is_ascii_alphanumeric()).count();
    let hexish_count = trimmed
        .chars()
        .filter(|c| c.is_ascii_hexdigit() || *c == '-')
        .count();
    alnum_count > 0 && hexish_count * 2 >= alnum_count
}

/// Appends a short ID suffix to labels that appear more than once.
fn disambiguate_labels(mics: &mut Vec<MicInfo>) {
    let mut label_counts: HashMap<String, usize> = HashMap::new();
    for mic in mics.iter() {
        *label_counts.entry(mic.label.clone()).or_insert(0) += 1;
    }
    for mic in mics.iter_mut() {
        if label_counts.get(&mic.label).copied().unwrap_or(0) > 1 {
            let suffix = short_id_suffix(&mic.id);
            mic.label = format!("{} [{}]", mic.label, suffix);
        }
    }
}

fn short_id_suffix(id: &str) -> String {
    let mut chars: Vec<char> = id.chars().rev().take(8).collect();
    chars.reverse();
    chars.into_iter().collect()
}

fn get_device_name(device: &cpal::Device) -> Option<String> {
    device
        .description()
        .ok()
        .map(|desc| desc.name().to_string())
}

fn get_device_id(device: &cpal::Device) -> Option<String> {
    device.id().ok().map(|id| id.to_string())
}

#[cfg(not(target_os = "linux"))]
fn find_device_by_identifier(identifier: &str) -> Option<cpal::Device> {
    let host = cpal::default_host();
    host.input_devices()
        .ok()?
        .find(|d| {
            get_device_id(d).as_deref() == Some(identifier)
                || get_device_name(d).as_deref() == Some(identifier)
        })
}

pub fn update_mic_cache(app: &tauri::AppHandle, mic_id: Option<String>) {
    let audio_state = app.state::<crate::audio::types::AudioState>();
    match mic_id {
        Some(ref id) => {
            #[cfg(not(target_os = "linux"))]
            {
                audio_state.set_cached_device(find_device_by_identifier(id));
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
                if let Some(device) = find_device_by_identifier(&id) {
                    if let Some(device_id) = get_device_id(&device) {
                        if device_id != id {
                            let mut s = crate::settings::load_settings(&app_handle);
                            s.mic_id = Some(device_id.clone());
                            if let Err(e) = crate::settings::save_settings(&app_handle, &s) {
                                warn!("Failed to migrate mic_id: {}", e);
                            } else {
                                info!("Migrated mic_id from display name to device ID: {}", device_id);
                            }
                        }
                    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_generic_mic_name_matches_known_names() {
        assert!(is_generic_mic_name("Microphone"));
        assert!(is_generic_mic_name("microphone"));
        assert!(is_generic_mic_name("Microphone Array"));
        assert!(is_generic_mic_name("Line In"));
        assert!(!is_generic_mic_name("Blue Yeti"));
    }

    #[test]
    fn looks_technical_detects_guids() {
        assert!(looks_technical("{0.0.1.00000000}.{abcdef12-3456-7890}"));
        assert!(!looks_technical("Realtek High Definition Audio"));
    }

    #[test]
    fn looks_technical_detects_hex_heavy_strings() {
        assert!(looks_technical("ab12cd34ef56"));
        assert!(!looks_technical("Blue Yeti Microphone"));
    }

    #[test]
    fn short_id_suffix_returns_last_8_chars() {
        assert_eq!(short_id_suffix("abcdefghijklmnop"), "ijklmnop");
    }

    #[test]
    fn short_id_suffix_handles_short_ids() {
        assert_eq!(short_id_suffix("abc"), "abc");
    }
}
