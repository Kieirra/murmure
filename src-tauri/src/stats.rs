use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsageStats {
    pub writing_speed_wpm: f64,
    pub words_current_month: u64,
    pub local_audio_mb: f64,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct AggregatedStats {
    wpm_sum: f64,
    wpm_count: u64,
    words_this_month: u64,
    last_reset_timestamp: i64,
    total_audio_bytes: u64,
}

const THIRTY_DAYS_SECS: i64 = 30 * 24 * 60 * 60;

fn maybe_reset_month(stats: &mut AggregatedStats, now_sec: i64) -> bool {
    let should_reset = stats.last_reset_timestamp == 0
        || now_sec - stats.last_reset_timestamp >= THIRTY_DAYS_SECS;
    if should_reset {
        stats.words_this_month = 0;
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
    let data = serde_json::from_str::<AggregatedStats>(&content)?;
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

    if word_count > 0 && duration_seconds > 0.0 {
        let minutes = duration_seconds / 60.0;
        if minutes > 0.0 {
            let wpm = (word_count as f64) / minutes;
            stats.wpm_sum += wpm;
            stats.wpm_count += 1;
        }
    }

    let now_sec = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;
    maybe_reset_month(&mut stats, now_sec);
    stats.words_this_month += word_count;

    stats.total_audio_bytes += wav_size_bytes;

    write_stats(app, &stats)
}

pub fn compute_stats(app: &AppHandle) -> Result<UsageStats> {
    let mut stats = read_stats(app)?;
    let now_sec = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;
    if maybe_reset_month(&mut stats, now_sec) {
        write_stats(app, &stats)?;
    }

    let writing_speed_wpm = if stats.wpm_count > 0 {
        stats.wpm_sum / (stats.wpm_count as f64)
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


