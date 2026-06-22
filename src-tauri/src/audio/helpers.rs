use anyhow::{Context, Result};

use hound::{WavSpec, WavWriter};
use log::warn;
use rubato::audioadapter_buffers::direct::InterleavedSlice;
use rubato::{
    calculate_cutoff, Async, FixedAsync, Resampler, SincInterpolationParameters,
    SincInterpolationType, WindowFunction,
};
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use tauri::Manager;

/// Capture buffer requested from cpal, in frames. The backend-default size
/// can silently drop a large share of capture periods, heard as crackling,
/// robotic audio.
const CAPTURE_BUFFER_FRAMES: u32 = 4096;

/// Build an input stream asking for the fixed capture buffer first, falling
/// back to the backend default when the device rejects it. `build` is called
/// with each candidate config.
pub fn build_input_with_buffer_fallback<S>(
    base: &cpal::StreamConfig,
    mut build: impl FnMut(&cpal::StreamConfig) -> std::result::Result<S, cpal::BuildStreamError>,
) -> std::result::Result<S, cpal::BuildStreamError> {
    let mut fixed = base.clone();
    fixed.buffer_size = cpal::BufferSize::Fixed(CAPTURE_BUFFER_FRAMES);
    build(&fixed).or_else(|e| {
        log::debug!(
            "Fixed capture buffer ({} frames) rejected: {}, falling back to default",
            CAPTURE_BUFFER_FRAMES,
            e
        );
        build(base)
    })
}

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
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("murmure-{}-{}.wav", ts, seq)
}

/// Recordings kept on disk when `keep_recordings` is enabled, rolling like
/// the transcription history (`MAX_HISTORY_ENTRIES`) so the audio matching a
/// history entry is still available without growing the temp dir forever.
const KEPT_RECORDINGS: usize = 5;

pub fn cleanup_recordings(app: &tauri::AppHandle) -> Result<()> {
    let recordings_dir = ensure_recordings_dir(app)?;

    if crate::settings::load_settings(app).keep_recordings {
        let mut recordings: Vec<(std::time::SystemTime, PathBuf)> =
            std::fs::read_dir(&recordings_dir)
                .context("Failed to read recordings directory")?
                .flatten()
                .filter(|entry| entry.path().is_file())
                .filter_map(|entry| {
                    let modified = entry.metadata().ok()?.modified().ok()?;
                    Some((modified, entry.path()))
                })
                .collect();
        recordings.sort_by(|a, b| b.0.cmp(&a.0));
        for (_, path) in recordings.into_iter().skip(KEPT_RECORDINGS) {
            if let Err(e) = std::fs::remove_file(&path) {
                warn!("Failed to delete old recording {}: {}", path.display(), e);
            }
        }
        log::info!(
            "Last {} recordings kept for debugging in {}",
            KEPT_RECORDINGS,
            recordings_dir.display()
        );
        return Ok(());
    }

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
    log::info!("Temporary audio files successfully cleaned up");

    Ok(())
}

pub fn read_wav_samples(wav_path: &Path) -> Result<Vec<f32>> {
    let (samples_f32, sample_rate) = read_wav_mono_native(wav_path)?;

    let out = if sample_rate != 16000 {
        resample(&samples_f32, sample_rate as usize, 16000)
    } else {
        samples_f32
    };

    Ok(out)
}

pub fn read_wav_mono_native(wav_path: &Path) -> Result<(Vec<f32>, u32)> {
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

    Ok((samples_f32, spec.sample_rate))
}

pub fn rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let sum_sq: f32 = samples.iter().map(|s| s * s).sum();
    (sum_sq / samples.len() as f32).sqrt()
}

pub fn resample(input: &[f32], src_hz: usize, dst_hz: usize) -> Vec<f32> {
    if input.is_empty() || src_hz == 0 || dst_hz == 0 {
        return Vec::new();
    }
    if src_hz == dst_hz {
        return input.to_vec();
    }

    resample_inner(input, src_hz, dst_hz).unwrap_or_else(|e| {
        warn!("Resampling failed ({src_hz}->{dst_hz}): {e}");
        Vec::new()
    })
}

fn resample_inner(input: &[f32], src_hz: usize, dst_hz: usize) -> Result<Vec<f32>> {
    let ratio = dst_hz as f64 / src_hz as f64;
    let sinc_len = 128;
    let window = WindowFunction::BlackmanHarris2;
    let params = SincInterpolationParameters {
        sinc_len,
        f_cutoff: calculate_cutoff(sinc_len, window),
        oversampling_factor: 256,
        interpolation: SincInterpolationType::Linear,
        window,
    };

    let mut resampler = Async::<f32>::new_sinc(ratio, 1.0, &params, 1024, 1, FixedAsync::Input)?;

    let mut output = vec![0.0f32; resampler.process_all_needed_output_len(input.len())];
    let in_adapter = InterleavedSlice::new(input, 1, input.len())?;
    let out_capacity = output.len();
    let mut out_adapter = InterleavedSlice::new_mut(&mut output, 1, out_capacity)?;

    let (_, nbr_out) =
        resampler.process_all_into_buffer(&in_adapter, &mut out_adapter, input.len(), None)?;
    output.truncate(nbr_out);
    Ok(output)
}

pub fn create_wav_writer(
    path: &Path,
    config: &cpal::SupportedStreamConfig,
) -> Result<WavWriter<BufWriter<File>>> {
    let file = File::create(path).context("Failed to create WAV file")?;
    let writer = BufWriter::new(file);
    let spec = WavSpec {
        channels: 1,
        sample_rate: config.sample_rate(),
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    WavWriter::new(writer, spec).context("Failed to create WAV writer")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    fn sine(freq_hz: f32, sample_rate: usize, duration_s: f32) -> Vec<f32> {
        let n = (sample_rate as f32 * duration_s) as usize;
        (0..n)
            .map(|i| (2.0 * PI * freq_hz * i as f32 / sample_rate as f32).sin())
            .collect()
    }

    fn linear_reference(input: &[f32], src_hz: usize, dst_hz: usize) -> Vec<f32> {
        let ratio = dst_hz as f64 / src_hz as f64;
        let out_len = ((input.len() as f64) * ratio).ceil() as usize;
        let last_idx = input.len().saturating_sub(1);
        (0..out_len)
            .map(|i| {
                let t = i as f64 / ratio;
                let idx = t.floor() as usize;
                let frac = (t - idx as f64) as f32;
                let a = input[idx.min(last_idx)];
                let b = input[(idx + 1).min(last_idx)];
                a + (b - a) * frac
            })
            .collect()
    }

    #[test]
    fn should_return_empty_vec_when_input_is_empty() {
        assert_eq!(resample(&[], 48000, 16000), Vec::<f32>::new());
    }

    #[test]
    fn should_return_empty_vec_when_a_sample_rate_is_zero() {
        let signal = sine(1000.0, 48000, 0.1);
        assert_eq!(resample(&signal, 0, 16000), Vec::<f32>::new());
        assert_eq!(resample(&signal, 48000, 0), Vec::<f32>::new());
    }

    #[test]
    fn should_return_input_unchanged_when_src_equals_dst() {
        let signal = sine(1000.0, 16000, 0.1);
        assert_eq!(resample(&signal, 16000, 16000), signal);
    }

    #[test]
    fn should_produce_expected_length_without_nan_when_downsampling_48k_to_16k() {
        let signal = sine(1000.0, 48000, 1.0);
        let out = resample(&signal, 48000, 16000);

        let expected = signal.len() * 16000 / 48000;
        let block = 1024;
        assert!((out.len() as i64 - expected as i64).unsigned_abs() as usize <= block);
        assert!(out.iter().all(|s| s.is_finite()));
    }

    #[test]
    fn should_attenuate_above_nyquist_component_when_downsampling() {
        // 12 kHz at 48 kHz is above the 16 kHz Nyquist (8 kHz). Linear interpolation
        // folds it back into the audible band; rubato's sinc filter removes it.
        let signal = sine(12000.0, 48000, 1.0);

        let rubato_out = resample(&signal, 48000, 16000);
        let linear_out = linear_reference(&signal, 48000, 16000);

        assert!(rubato_out.iter().all(|s| s.is_finite()));
        assert!(rms(&rubato_out) < rms(&linear_out) * 0.2);
    }

    #[test]
    fn should_not_panic_and_double_length_when_upsampling_8k_to_16k() {
        let signal = sine(1000.0, 8000, 0.5);
        let out = resample(&signal, 8000, 16000);

        let expected = signal.len() * 2;
        let block = 2048;
        assert!((out.len() as i64 - expected as i64).unsigned_abs() as usize <= block);
        assert!(out.iter().all(|s| s.is_finite()));
    }

    #[test]
    fn should_produce_coherent_length_when_ratio_is_fractional_44100_to_16000() {
        let signal = sine(1000.0, 44100, 1.0);
        let out = resample(&signal, 44100, 16000);

        let expected = (signal.len() as f64 * 16000.0 / 44100.0) as usize;
        let block = 2048;
        assert!((out.len() as i64 - expected as i64).unsigned_abs() as usize <= block);
        assert!(out.iter().all(|s| s.is_finite()));
    }

    #[test]
    fn generate_unique_wav_name_differs_on_rapid_calls() {
        let names: Vec<String> = (0..100).map(|_| generate_unique_wav_name()).collect();
        let unique: std::collections::HashSet<&String> = names.iter().collect();
        assert_eq!(unique.len(), names.len());
        assert!(names
            .iter()
            .all(|n| n.starts_with("murmure-") && n.ends_with(".wav")));
    }
}
