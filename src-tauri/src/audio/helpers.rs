use anyhow::{Context, Result};

use hound::{WavSpec, WavWriter};
use log::warn;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use tauri::Manager;

pub fn ensure_recordings_dir(app: &tauri::AppHandle) -> Result<PathBuf> {
    let recordings = app
        .path()
        .temp_dir()
        .context("Failed to resolve temp dir")?
        .join("murmure_recordings");

    if !recordings.exists() {
        std::fs::create_dir_all(&recordings).context("Failed to create recordings dir")?;
    }

    Ok(recordings)
}

pub fn generate_unique_wav_name() -> String {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("murmure-{}.wav", ts)
}

pub fn cleanup_recordings(app: &tauri::AppHandle) -> Result<()> {
    let recordings_dir = ensure_recordings_dir(app)?;

    if !recordings_dir.exists() {
        return Ok(());
    }

    let entries =
        std::fs::read_dir(&recordings_dir).context("Failed to read recordings directory")?;

    for entry in entries.flatten() {
        if entry.path().is_file() {
            if let Err(e) = std::fs::remove_file(entry.path()) {
                warn!("Failed to delete {}: {}", entry.path().display(), e);
            }
        }
    }

    Ok(())
}

pub fn read_wav_samples(wav_path: &Path) -> Result<Vec<f32>> {
    let mut reader = hound::WavReader::open(wav_path)?;
    let spec = reader.spec();

    if spec.bits_per_sample != 16 {
        return Err(anyhow::anyhow!(
            "Expected 16 bits per sample, found {}",
            spec.bits_per_sample
        ));
    }

    if spec.sample_format != hound::SampleFormat::Int {
        return Err(anyhow::anyhow!(
            "Expected Int sample format, found {:?}",
            spec.sample_format
        ));
    }

    let raw_i16: Result<Vec<i16>, _> = reader.samples::<i16>().collect();
    let mut raw_i16 = raw_i16?;

    if spec.channels > 1 {
        let ch = spec.channels as usize;
        let mut mono: Vec<i16> = Vec::with_capacity(raw_i16.len() / ch);
        for frame in raw_i16.chunks_exact(ch) {
            let sum: i32 = frame.iter().map(|&s| s as i32).sum();
            let avg = (sum / ch as i32).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
            mono.push(avg);
        }
        raw_i16 = mono;
    }

    let samples_f32: Vec<f32> = raw_i16
        .into_iter()
        .map(|s| s as f32 / i16::MAX as f32)
        .collect();

    let out = if spec.sample_rate != 16000 {
        resample_linear(&samples_f32, spec.sample_rate as usize, 16000)
    } else {
        samples_f32
    };

    Ok(out)
}

pub fn resample_linear(input: &[f32], src_hz: usize, dst_hz: usize) -> Vec<f32> {
    if input.is_empty() || src_hz == 0 || dst_hz == 0 {
        return Vec::new();
    }
    if src_hz == dst_hz {
        return input.to_vec();
    }
    let ratio = dst_hz as f64 / src_hz as f64;
    let out_len = ((input.len() as f64) * ratio).ceil() as usize;
    if out_len == 0 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(out_len);
    let last_idx = input.len().saturating_sub(1);
    for i in 0..out_len {
        let t = (i as f64) / ratio;
        let idx = t.floor() as usize;
        let frac = (t - idx as f64) as f32;
        let a = input[idx];
        let b = input[std::cmp::min(idx + 1, last_idx)];
        out.push(a + (b - a) * frac);
    }
    out
}

pub fn create_wav_writer(
    path: &Path,
    config: &cpal::SupportedStreamConfig,
) -> Result<WavWriter<BufWriter<File>>> {
    let file = File::create(path).context("Failed to create WAV file")?;
    let writer = BufWriter::new(file);
    let spec = WavSpec {
        channels: 1,
        sample_rate: config.sample_rate().0,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    WavWriter::new(writer, spec).context("Failed to create WAV writer")
}
