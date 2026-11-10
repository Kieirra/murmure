use anyhow::{Context, Result};
use chrono::{DateTime, Datelike, Local, NaiveDate, TimeZone};

use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};

use super::types::{AggregatedStats, UsageStats};

const TYPING_WPM: f64 = 40.0;

fn should_reset_month<Tz: TimeZone>(last_reset_ts: i64, now: &DateTime<Tz>) -> bool {
    match DateTime::from_timestamp(last_reset_ts, 0) {
        Some(last_reset) if last_reset_ts != 0 => {
            let last_reset = last_reset.with_timezone(&now.timezone());
            (last_reset.year(), last_reset.month()) != (now.year(), now.month())
        }
        _ => true,
    }
}

fn maybe_reset_month<Tz: TimeZone>(stats: &mut AggregatedStats, now: &DateTime<Tz>) -> bool {
    if should_reset_month(stats.last_reset_timestamp, now) {
        stats.words_this_month = 0;
        stats.last_reset_timestamp = now.timestamp();
        return true;
    }
    false
}

fn next_month_start_ts<Tz: TimeZone>(now: &DateTime<Tz>) -> Result<i64> {
    let (year, month) = match now.month() {
        12 => (now.year() + 1, 1),
        month => (now.year(), month + 1),
    };
    let date = NaiveDate::from_ymd_opt(year, month, 1).context("Invalid next month start date")?;
    let midnight = date.and_hms_opt(0, 0, 0).context("Invalid midnight")?;
    let timezone = now.timezone();
    let next_reset = timezone
        .from_local_datetime(&midnight)
        .earliest()
        .or_else(|| {
            date.and_hms_opt(1, 0, 0)
                .and_then(|fallback| timezone.from_local_datetime(&fallback).earliest())
        })
        .context("Next month start is unavailable in the local timezone")?;
    Ok(next_reset.timestamp())
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

    let now = Local::now();
    maybe_reset_month(&mut stats, &now);
    stats.words_this_month += word_count;

    stats.total_audio_bytes += wav_size_bytes;

    write_stats(app, &stats)?;

    app.emit("stats_updated", ())?;
    Ok(())
}

pub fn compute_stats(app: &AppHandle) -> Result<UsageStats> {
    let mut stats = read_stats(app)?;
    let now = Local::now();
    if maybe_reset_month(&mut stats, &now) {
        write_stats(app, &stats)?;
    }

    let writing_speed_wpm = if stats.wpm_count > 0 {
        stats.wpm_sum / (stats.wpm_count as f64)
    } else {
        0.0
    };

    let words_current_month = stats.words_this_month;

    let local_audio_mb = (stats.total_audio_bytes as f64) / 1_000_000.0;

    let time_saved_seconds = if stats.wpm_count > 0 && writing_speed_wpm > TYPING_WPM {
        words_current_month as f64 * (1.0 / TYPING_WPM - 1.0 / writing_speed_wpm) * 60.0
    } else {
        0.0
    };

    Ok(UsageStats {
        writing_speed_wpm,
        words_current_month,
        local_audio_mb,
        time_saved_seconds,
        next_reset_timestamp: next_month_start_ts(&now)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::FixedOffset;

    fn at(offset_hours: i32, y: i32, m: u32, d: u32, h: u32, min: u32) -> DateTime<FixedOffset> {
        FixedOffset::east_opt(offset_hours * 3600)
            .unwrap()
            .with_ymd_and_hms(y, m, d, h, min, 0)
            .unwrap()
    }

    #[test]
    fn zero_timestamp_resets() {
        assert!(should_reset_month(0, &at(1, 2026, 11, 10, 12, 0)));
    }

    #[test]
    fn out_of_range_timestamp_resets() {
        assert!(should_reset_month(i64::MAX, &at(1, 2026, 11, 10, 12, 0)));
    }

    #[test]
    fn same_month_does_not_reset() {
        let last = at(1, 2026, 10, 1, 0, 30).timestamp();
        assert!(!should_reset_month(last, &at(1, 2026, 10, 31, 23, 59)));
    }

    #[test]
    fn thirty_days_inside_same_month_does_not_reset() {
        let last = at(1, 2026, 10, 1, 8, 0).timestamp();
        assert!(!should_reset_month(last, &at(1, 2026, 10, 31, 9, 0)));
    }

    #[test]
    fn crossing_month_resets() {
        let last = at(1, 2026, 10, 31, 23, 59).timestamp();
        assert!(should_reset_month(last, &at(1, 2026, 11, 1, 0, 0)));
    }

    #[test]
    fn uses_local_month_not_utc() {
        let last = at(2, 2026, 10, 15, 12, 0).timestamp();
        assert!(should_reset_month(last, &at(2, 2026, 11, 1, 1, 0)));
    }

    #[test]
    fn clock_moved_back_resets() {
        let last = at(1, 2026, 12, 5, 12, 0).timestamp();
        assert!(should_reset_month(last, &at(1, 2026, 11, 20, 12, 0)));
    }

    #[test]
    fn maybe_reset_clears_words_and_sets_timestamp() {
        let now = at(1, 2026, 11, 1, 9, 0);
        let mut stats = AggregatedStats {
            words_this_month: 4200,
            last_reset_timestamp: at(1, 2026, 10, 22, 9, 0).timestamp(),
            ..Default::default()
        };
        assert!(maybe_reset_month(&mut stats, &now));
        assert_eq!(stats.words_this_month, 0);
        assert_eq!(stats.last_reset_timestamp, now.timestamp());
    }

    #[test]
    fn next_month_start_is_first_day_midnight_local() {
        assert_eq!(
            next_month_start_ts(&at(1, 2026, 11, 10, 12, 0)).unwrap(),
            at(1, 2026, 12, 1, 0, 0).timestamp()
        );
    }

    #[test]
    fn december_rolls_to_january() {
        assert_eq!(
            next_month_start_ts(&at(0, 2026, 12, 15, 0, 0)).unwrap(),
            at(0, 2027, 1, 1, 0, 0).timestamp()
        );
    }
}
