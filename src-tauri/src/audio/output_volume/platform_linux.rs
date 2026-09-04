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
            application: application_name(stream),
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
    let mut restored: Vec<String> = Vec::new();

    for entry in state {
        let stream = streams.iter().find(|stream| {
            stream_index(stream).as_deref() == Some(entry.target.as_str())
                && same_application(stream, entry)
        });
        let Some(stream) = stream else {
            continue;
        };
        if !is_at_lowered_level(stream, entry) {
            debug!(
                "Stream {} volume changed since it was lowered, keeping the user value",
                entry.target
            );
            continue;
        }
        if set_sink_input_volume(&entry.target, &entry.original).is_none() {
            debug!("Failed to restore volume of stream {}", entry.target);
            continue;
        }
        restored.push(entry.target.clone());
    }

    // The sweep always runs because a stream born during the dictation starts at the level
    // the audio server remembers for the application, even when the recorded stream survived.
    // It comes last so exact matches claim their own index before the sweep can take it.
    for entry in state {
        restore_by_application(&streams, entry, &mut restored);
    }
}

// Browsers drop and recreate their stream on tab churn, and the audio server remembers
// the level we set per application, so the next stream would start ducked. Restoring
// every live stream of the application rewrites that remembered level.
fn restore_by_application(
    streams: &[serde_json::Value],
    entry: &LoweredOutput,
    restored: &mut Vec<String>,
) {
    let Some(application) = entry.application.as_deref() else {
        return;
    };

    for index in streams_to_restore(streams, entry, restored) {
        if set_sink_input_volume(&index, &entry.original).is_none() {
            debug!("Failed to restore volume of stream {}", index);
            continue;
        }
        debug!(
            "Restored stream {} of {} by application name",
            index, application
        );
        restored.push(index);
    }
}

fn streams_to_restore(
    streams: &[serde_json::Value],
    entry: &LoweredOutput,
    restored: &[String],
) -> Vec<String> {
    let Some(application) = entry.application.as_deref() else {
        return Vec::new();
    };
    let mut targets: Vec<String> = Vec::new();

    for stream in streams {
        let Some(index) = stream_index(stream) else {
            continue;
        };
        if restored.contains(&index) || application_name(stream).as_deref() != Some(application) {
            continue;
        }
        if !is_at_lowered_level(stream, entry) {
            continue;
        }
        targets.push(index);
    }

    targets
}

fn is_at_lowered_level(stream: &serde_json::Value, entry: &LoweredOutput) -> bool {
    levels_from_object(stream)
        .is_some_and(|current| levels_match(&current, &entry.applied, TOLERANCE))
}

// Indexes are recycled, so a stream carrying the recorded index can belong to another
// application by the time we restore.
fn same_application(stream: &serde_json::Value, entry: &LoweredOutput) -> bool {
    match entry.application.as_deref() {
        Some(application) => application_name(stream).as_deref() == Some(application),
        None => true,
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

fn application_name(stream: &serde_json::Value) -> Option<String> {
    stream
        .get("properties")
        .and_then(|properties| properties.get("application.name"))
        .and_then(|name| name.as_str())
        .map(|name| name.to_string())
}

// Our playback goes through the ALSA plug-in, which reports no process id, so the
// application name carrying the binary name is the only way to spot our own stream.
fn is_own_stream(stream: &serde_json::Value, own_name: &str) -> bool {
    application_name(stream).is_some_and(|name| name.to_lowercase().contains(own_name))
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

    fn sink_input(index: u64, application: &str, level: u32) -> serde_json::Value {
        json!({
            "index": index,
            "channel_map": "mono",
            "volume": { "mono": { "value": level } },
            "properties": { "application.name": application }
        })
    }

    fn lowered_brave() -> LoweredOutput {
        LoweredOutput {
            target: "42".to_string(),
            original: vec![65536],
            applied: vec![13107],
            application: Some("Brave".to_string()),
        }
    }

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
    fn matches_the_recorded_application_on_a_recycled_index() {
        let entry = LoweredOutput {
            target: "42".to_string(),
            original: vec![65536],
            applied: vec![13107],
            application: Some("Brave".to_string()),
        };
        let brave = json!({ "properties": { "application.name": "Brave" } });
        let firefox = json!({ "properties": { "application.name": "Firefox" } });

        assert!(same_application(&brave, &entry));
        assert!(!same_application(&firefox, &entry));
    }

    #[test]
    fn accepts_any_application_when_none_was_recorded() {
        let entry = LoweredOutput {
            target: "42".to_string(),
            original: vec![65536],
            applied: vec![13107],
            application: None,
        };

        assert!(same_application(
            &json!({ "properties": { "application.name": "Firefox" } }),
            &entry
        ));
    }

    #[test]
    fn sweeps_a_recreated_stream_of_the_same_application() {
        let streams = vec![sink_input(77, "Brave", 13107)];

        assert_eq!(
            streams_to_restore(&streams, &lowered_brave(), &[]),
            vec!["77".to_string()]
        );
    }

    #[test]
    fn skips_streams_of_another_application_at_the_lowered_level() {
        let streams = vec![sink_input(77, "Firefox", 13107)];

        assert!(streams_to_restore(&streams, &lowered_brave(), &[]).is_empty());
    }

    #[test]
    fn skips_an_index_an_exact_match_already_restored() {
        let streams = vec![sink_input(42, "Brave", 13107)];

        assert!(streams_to_restore(&streams, &lowered_brave(), &["42".to_string()]).is_empty());
    }

    #[test]
    fn skips_streams_the_user_raised_since_they_were_lowered() {
        let streams = vec![sink_input(77, "Brave", 45000)];

        assert!(streams_to_restore(&streams, &lowered_brave(), &[]).is_empty());
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
