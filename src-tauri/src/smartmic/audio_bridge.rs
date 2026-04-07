use crate::audio::helpers::resample_linear;

/// Maximum recording samples: 5 minutes at 16 kHz
const MAX_RECORDING_SAMPLES: usize = 4_800_000;

/// Accumulate raw PCM bytes (Int16 LE) into the buffer.
/// Returns `true` if samples were added, `false` if the buffer is full.
pub fn accumulate_pcm(buffer: &mut Vec<i16>, payload: &[u8]) -> bool {
    // Each sample is 2 bytes (Int16 LE)
    let sample_count = payload.len() / 2;

    if buffer.len() + sample_count > MAX_RECORDING_SAMPLES {
        return false;
    }

    buffer.reserve(sample_count);

    for chunk in payload.chunks_exact(2) {
        let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
        buffer.push(sample);
    }

    true
}

/// Convert the accumulated Int16 buffer to Vec<f32>, resampling if needed
pub fn finalize_buffer(buffer: Vec<i16>, source_sample_rate: u32) -> Vec<f32> {
    // Convert i16 to f32 (normalize to -1.0..1.0)
    let samples_f32: Vec<f32> = buffer
        .into_iter()
        .map(|s| s as f32 / i16::MAX as f32)
        .collect();

    // Resample to 16kHz if needed
    if source_sample_rate != 16000 {
        resample_linear(&samples_f32, source_sample_rate as usize, 16000)
    } else {
        samples_f32
    }
}

/// Calculate the RMS (Root Mean Square) level of the given samples, normalized to 0.0-1.0
pub fn calculate_rms(samples: &[i16]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }

    let sum_squares: f64 = samples
        .iter()
        .map(|&s| {
            let f = s as f64 / i16::MAX as f64;
            f * f
        })
        .sum();

    let rms = (sum_squares / samples.len() as f64).sqrt() as f32;

    // Clamp to 0.0-1.0
    rms.clamp(0.0, 1.0)
}
