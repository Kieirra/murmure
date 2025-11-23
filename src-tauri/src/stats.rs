use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsageStats {
    pub writing_speed_wpm: f64,
    pub words_current_month: u64,
    pub local_audio_mb: f64,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct AggregatedStats {
    words_this_month: u64,
    #[serde(default)]
    seconds_this_month: f64,
    last_reset_timestamp: i64,
    total_audio_bytes: u64,
}

const THIRTY_DAYS_SECS: i64 = 30 * 24 * 60 * 60;

fn maybe_reset_month(stats: &mut AggregatedStats, now_sec: i64) -> bool {
    let should_reset =
        stats.last_reset_timestamp == 0 || now_sec - stats.last_reset_timestamp >= THIRTY_DAYS_SECS;
    if should_reset {
        stats.words_this_month = 0;
        stats.seconds_this_month = 0.0;
        stats.last_reset_timestamp = now_sec;
        return true;
    }
    false
}

fn stats_file_path(app: &AppHandle) -> Result<PathBuf> {
    let dir = tauri::Manager::path(app).app_data_dir()?;
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(dir.join("stats_aggregated.json"))
}

fn read_stats(app: &AppHandle) -> Result<AggregatedStats> {
    let path = stats_file_path(app)?;
    if !path.exists() {
        return Ok(AggregatedStats::default());
    }
    let content = fs::read_to_string(path)?;
    // If the file exists but has old fields, serde will ignore them.
    // Missing new fields will use default values (0.0 for seconds_this_month).
    let data = serde_json::from_str::<AggregatedStats>(&content).unwrap_or_default();
    Ok(data)
}

fn write_stats(app: &AppHandle, data: &AggregatedStats) -> Result<()> {
    let path = stats_file_path(app)?;
    let content = serde_json::to_string_pretty(data)?;
    fs::write(path, content)?;
    Ok(())
}

pub fn add_transcription_session(
    app: &AppHandle,
    word_count: u64,
    duration_seconds: f64,
    wav_size_bytes: u64,
) -> Result<()> {
    let mut stats = read_stats(app)?;

    let now_sec = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;
    maybe_reset_month(&mut stats, now_sec);

    stats.words_this_month += word_count;
    stats.seconds_this_month += duration_seconds;
    stats.total_audio_bytes += wav_size_bytes;

    write_stats(app, &stats)?;

    app.emit("stats_updated", ())?;
    Ok(())
}

pub fn compute_stats(app: &AppHandle) -> Result<UsageStats> {
    let mut stats = read_stats(app)?;
    let now_sec = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;
    if maybe_reset_month(&mut stats, now_sec) {
        write_stats(app, &stats)?;
    }

    let writing_speed_wpm = if stats.seconds_this_month > 0.0 {
        (stats.words_this_month as f64) / (stats.seconds_this_month / 60.0)
    } else {
        0.0
    };

    let words_current_month = stats.words_this_month;

    let local_audio_mb = (stats.total_audio_bytes as f64) / 1_000_000.0;

    Ok(UsageStats {
        writing_speed_wpm,
        words_current_month,
        local_audio_mb,
    })
}
