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

pub fn lower(percent: u8) -> Option<Vec<LoweredOutput>> {
    let streams = sink_inputs()?;
    let own_name = own_application_name();
    let mut lowered: Vec<LoweredOutput> = Vec::new();
    let mut skipped = 0usize;

    for stream in &streams {
        if own_name
            .as_deref()
            .is_some_and(|name| is_own_stream(stream, name))
        {
            skipped += 1;
            continue;
        }
        let Some(target) = stream_index(stream) else {
            continue;
        };
        let Some(original) = levels_from_object(stream) else {
            continue;
        };
        let applied = lowered_levels(&original, percent);
        if set_sink_input_volume(&target, &applied).is_none() {
            continue;
        }
        lowered.push(LoweredOutput {
            target,
            original,
            applied,
        });
    }

    if skipped > 0 {
        debug!("Skipped {} of our own output streams", skipped);
    }

    if lowered.is_empty() {
        None
    } else {
        Some(lowered)
    }
}

pub fn restore(state: &[LoweredOutput]) {
    let Some(streams) = sink_inputs() else {
        return;
    };

    for entry in state {
        let Some(stream) = streams
            .iter()
            .find(|stream| stream_index(stream).as_deref() == Some(entry.target.as_str()))
        else {
            continue;
        };
        let Some(current) = levels_from_object(stream) else {
            continue;
        };
        if !levels_match(&current, &entry.applied, TOLERANCE) {
            debug!(
                "Stream {} volume changed since it was lowered, keeping the user value",
                entry.target
            );
            continue;
        }
        if set_sink_input_volume(&entry.target, &entry.original).is_none() {
            debug!("Failed to restore volume of stream {}", entry.target);
        }
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

    levels_from_object(sink)
}

fn sink_inputs() -> Option<Vec<serde_json::Value>> {
    let stdout = pactl_stdout(&["-f", "json", "list", "sink-inputs"])?;
    serde_json::from_slice(&stdout).ok()
}

fn stream_index(stream: &serde_json::Value) -> Option<String> {
    Some(stream.get("index")?.as_u64()?.to_string())
}

fn own_application_name() -> Option<String> {
    let exe = std::env::current_exe().ok()?;
    Some(exe.file_stem()?.to_string_lossy().to_lowercase())
}

// Our playback goes through the ALSA plug-in, which reports no process id, so the
// application name carrying the binary name is the only way to spot our own stream.
fn is_own_stream(stream: &serde_json::Value, own_name: &str) -> bool {
    stream
        .get("properties")
        .and_then(|properties| properties.get("application.name"))
        .and_then(|name| name.as_str())
        .is_some_and(|name| name.to_lowercase().contains(own_name))
}

fn levels_from_object(object: &serde_json::Value) -> Option<Vec<u32>> {
    let volume = object.get("volume")?;
    let channels = channel_names(object)?;
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

fn channel_names(object: &serde_json::Value) -> Option<Vec<String>> {
    let names: Vec<String> = object
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

fn set_sink_input_volume(index: &str, levels: &[u32]) -> Option<()> {
    let output = std::process::Command::new("pactl")
        .args(["set-sink-input-volume", index])
        .args(levels.iter().map(|level| level.to_string()))
        .output()
        .ok()?;

    match output.status.success() {
        true => Some(()),
        false => {
            debug!(
                "pactl set-sink-input-volume failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn reads_levels_in_channel_map_order() {
        let object = json!({
            "channel_map": "front-left,front-right",
            "volume": {
                "front-right": { "value": 32768 },
                "front-left": { "value": 65536 }
            }
        });

        assert_eq!(levels_from_object(&object), Some(vec![65536, 32768]));
    }

    #[test]
    fn rejects_missing_or_empty_channel_map() {
        let missing = json!({ "volume": { "mono": { "value": 65536 } } });
        let empty = json!({
            "channel_map": "",
            "volume": { "mono": { "value": 65536 } }
        });

        assert_eq!(levels_from_object(&missing), None);
        assert_eq!(levels_from_object(&empty), None);
    }

    #[test]
    fn rejects_channel_absent_from_volume() {
        let object = json!({
            "channel_map": "front-left,front-right",
            "volume": { "front-left": { "value": 65536 } }
        });

        assert_eq!(levels_from_object(&object), None);
    }

    #[test]
    fn detects_our_stream_on_both_alsa_plugin_variants() {
        let pipewire = json!({
            "properties": { "application.name": "PipeWire ALSA [murmure]" }
        });
        let pulseaudio = json!({
            "properties": { "application.name": "ALSA plug-in [Murmure]" }
        });

        assert!(is_own_stream(&pipewire, "murmure"));
        assert!(is_own_stream(&pulseaudio, "murmure"));
    }

    #[test]
    fn keeps_streams_of_other_applications() {
        let other = json!({
            "properties": { "application.name": "Firefox" }
        });
        let without_name = json!({ "properties": {} });

        assert!(!is_own_stream(&other, "murmure"));
        assert!(!is_own_stream(&without_name, "murmure"));
    }
}
