use super::helpers::{levels_match, lowered_levels};
use super::types::LoweredOutput;
use log::debug;

const TOLERANCE: u32 = 4;
const MAX_LEVEL: u32 = 100;

pub fn unsupported_reason() -> Option<String> {
    match current_level() {
        Some(_) => None,
        None => Some("no_volume_control".to_string()),
    }
}

pub fn lower(percent: u8) -> Option<LoweredOutput> {
    let original = vec![current_level()?];
    let applied = lowered_levels(&original, percent);
    set_level(*applied.first()?)?;
    Some(LoweredOutput {
        target: String::new(),
        original,
        applied,
    })
}

pub fn restore(state: &LoweredOutput) {
    let Some(current) = current_level() else {
        return;
    };
    if !levels_match(&[current], &state.applied, TOLERANCE) {
        debug!("Output volume changed since it was lowered, keeping the user value");
        return;
    }
    match state.original.first() {
        Some(level) => {
            if set_level(*level).is_none() {
                debug!("Failed to restore output volume");
            }
        }
        None => debug!("No output volume to restore"),
    }
}

fn current_level() -> Option<u32> {
    let output = std::process::Command::new("osascript")
        .args(["-e", "output volume of (get volume settings)"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    String::from_utf8_lossy(&output.stdout).trim().parse().ok()
}

fn set_level(level: u32) -> Option<()> {
    let script = format!("set volume output volume {}", level.min(MAX_LEVEL));
    let output = std::process::Command::new("osascript")
        .args(["-e", &script])
        .output()
        .ok()?;

    match output.status.success() {
        true => Some(()),
        false => {
            debug!(
                "osascript set volume failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            None
        }
    }
}
