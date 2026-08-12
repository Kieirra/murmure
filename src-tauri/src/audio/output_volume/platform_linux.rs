use super::helpers::{levels_match, lowered_levels};
use super::types::LoweredOutput;
use log::debug;

const TOLERANCE: u32 = 655;

pub fn unsupported_reason() -> Option<String> {
    let Some(sink) = default_sink() else {
        return Some("no_audio_server".to_string());
    };
    match sink_levels(&sink) {
        Some(_) => None,
        None => Some("no_volume_control".to_string()),
    }
}

pub fn lower(percent: u8) -> Option<LoweredOutput> {
    let target = default_sink()?;
    let original = sink_levels(&target)?;
    let applied = lowered_levels(&original, percent);
    set_sink_volume(&target, &applied)?;
    Some(LoweredOutput {
        target,
        original,
        applied,
    })
}

pub fn restore(state: &LoweredOutput) {
    let Some(current) = sink_levels(&state.target) else {
        return;
    };
    if !levels_match(&current, &state.applied, TOLERANCE) {
        debug!("Output volume changed since it was lowered, keeping the user value");
        return;
    }
    if set_sink_volume(&state.target, &state.original).is_none() {
        debug!("Failed to restore output volume of {}", state.target);
    }
}

fn pactl_stdout(args: &[&str]) -> Option<Vec<u8>> {
    let output = std::process::Command::new("pactl")
        .args(args)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    Some(output.stdout)
}

fn default_sink() -> Option<String> {
    let stdout = pactl_stdout(&["get-default-sink"])?;
    let name = String::from_utf8_lossy(&stdout).trim().to_string();

    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

fn sink_levels(sink_name: &str) -> Option<Vec<u32>> {
    let stdout = pactl_stdout(&["-f", "json", "list", "sinks"])?;
    let sinks: Vec<serde_json::Value> = serde_json::from_slice(&stdout).ok()?;
    let sink = sinks
        .iter()
        .find(|sink| sink.get("name").and_then(|name| name.as_str()) == Some(sink_name))?;

    let volume = sink.get("volume")?;
    let channels = channel_names(sink)?;
    let levels: Vec<u32> = channels
        .iter()
        .filter_map(|channel| {
            let value = volume.get(channel)?.get("value")?.as_u64()?;
            u32::try_from(value).ok()
        })
        .collect();

    if levels.len() == channels.len() {
        Some(levels)
    } else {
        None
    }
}

fn channel_names(sink: &serde_json::Value) -> Option<Vec<String>> {
    let names: Vec<String> = sink
        .get("channel_map")?
        .as_str()?
        .split(',')
        .map(|name| name.trim().to_string())
        .filter(|name| !name.is_empty())
        .collect();

    if names.is_empty() {
        None
    } else {
        Some(names)
    }
}

fn set_sink_volume(sink_name: &str, levels: &[u32]) -> Option<()> {
    let output = std::process::Command::new("pactl")
        .args(["set-sink-volume", sink_name])
        .args(levels.iter().map(|level| level.to_string()))
        .output()
        .ok()?;

    match output.status.success() {
        true => Some(()),
        false => {
            debug!(
                "pactl set-sink-volume failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            None
        }
    }
}
