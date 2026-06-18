use crate::audio::chunk_pipeline::ChunkJob;
use crate::audio::helpers::create_wav_writer;
use crate::audio::sound;
use crate::audio::types::RecordingTrigger;
use anyhow::{Context, Error, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Device;
use hound::WavWriter;
use log::{debug, error, info, trace, warn};
use parking_lot::Mutex;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread::JoinHandle;
use tauri::{AppHandle, Emitter, Manager};

/// Per-chunk safety guard only. Never reached during normal use because chunks
/// are force-cut at CHUNK_FORCE_CUT_SECS; it caps a single chunk if both the
/// silence and forced cuts somehow failed to fire.
const MAX_RECORDING_DURATION_SECS: u64 = 300; // 5 min
const SILENCE_AUTO_STOP_THRESHOLD: f32 = 0.03;
const SILENCE_AUTO_STOP_SPEECH_THRESHOLD: f32 = 0.03;

/// Once the current chunk reaches this length, a detected silence cuts it.
const CHUNK_SILENCE_ARM_SECS: u32 = 15;
/// Hard cut applied when no silence has been detected by this length.
const CHUNK_FORCE_CUT_SECS: u32 = 60;
/// Tail of the current chunk kept as the head of the next one on a forced cut,
/// so a word straddling the cut can be deduplicated at the seam.
const CHUNK_FORCED_OVERLAP_SECS: f32 = 1.0;

// Mirroring streaming.rs VAD, proven values: hysteresis on an EMA-smoothed RMS
// so a quiet short word still arms speech and a micro-peak during a pause does
// not reset the silence timer.
const LONG_DICTATION_SPEECH_THRESHOLD: f32 = 0.015;
const LONG_DICTATION_SILENCE_THRESHOLD: f32 = 0.01;
const LONG_DICTATION_EMA_ALPHA: f32 = 0.3;

/// Whether the smoothed signal is in silence once speech has started.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LongDictationSilence {
    NotStarted,
    Silent,
    Active,
}

/// EMA + hysteresis VAD. The silence-to-boundary timer lives in the writer thread, not here.
struct LongDictationVad {
    smoothed_rms: f32,
    has_speech_started: bool,
}

impl LongDictationVad {
    fn new() -> Self {
        Self {
            smoothed_rms: 0.0,
            has_speech_started: false,
        }
    }

    fn update(&mut self, rms: f32) -> LongDictationSilence {
        self.smoothed_rms =
            LONG_DICTATION_EMA_ALPHA * rms + (1.0 - LONG_DICTATION_EMA_ALPHA) * self.smoothed_rms;

        if self.smoothed_rms > LONG_DICTATION_SPEECH_THRESHOLD {
            self.has_speech_started = true;
        }

        if !self.has_speech_started {
            LongDictationSilence::NotStarted
        } else if self.smoothed_rms < LONG_DICTATION_SILENCE_THRESHOLD {
            LongDictationSilence::Silent
        } else {
            LongDictationSilence::Active
        }
    }
}

type WavWriterType = WavWriter<BufWriter<File>>;
type SharedWriter = Arc<Mutex<Option<WavWriterType>>>;

// Wrapper to safely store Stream. Stream on macOS doesn't implement Send.
pub struct SendStream(pub Option<cpal::Stream>);
unsafe impl Send for SendStream {}
unsafe impl Sync for SendStream {}

pub struct AudioRecorder {
    writer: SharedWriter,
    stream: SendStream,
    writer_thread: Option<JoinHandle<()>>,
    app_handle: AppHandle,
    start_time: Option<std::time::Instant>,
    previous_default_source: Option<String>,
    sample_rate: u32,
}

impl AudioRecorder {
    pub fn new(app: AppHandle, file_path: &Path, limit_reached: Arc<AtomicBool>) -> Result<Self> {
        // Reset the limit flag at the start of each recording
        limit_reached.store(false, Ordering::SeqCst);

        let audio_state = app.state::<crate::audio::types::AudioState>();
        let recording_trigger = audio_state.get_recording_trigger();
        let long_dictation_active = audio_state.long_dictation_active.clone();
        let chunk_tx = audio_state
            .chunk_pipeline
            .lock()
            .as_ref()
            .map(|pipeline| pipeline.sender());

        let (device, previous_default_source) = Self::get_device(app.clone())?;
        let config = match device
            .default_input_config()
            .context("No input config available")
        {
            Ok(config) => config,
            Err(error) => {
                crate::audio::microphone::restore_default_source_after_recording(
                    previous_default_source,
                );
                return Err(error);
            }
        };

        let writer = match create_wav_writer(file_path, &config) {
            Ok(writer) => writer,
            Err(error) => {
                crate::audio::microphone::restore_default_source_after_recording(
                    previous_default_source,
                );
                return Err(error);
            }
        };
        let writer_arc = Arc::new(Mutex::new(Some(writer)));

        let streaming_buf = {
            let audio_state = app.state::<crate::audio::types::AudioState>();
            audio_state.streaming_buffer.clone()
        };

        let writer_ctx = WriterThreadCtx {
            app: app.clone(),
            limit_reached,
            recording_trigger,
            streaming_buffer: streaming_buf,
            chunk_tx,
            sample_rate: config.sample_rate(),
            long_dictation_active,
        };

        let (stream, writer_thread) =
            match build_stream(&device, &config, writer_arc.clone(), writer_ctx) {
                Ok(parts) => parts,
                Err(error) => {
                    crate::audio::microphone::restore_default_source_after_recording(
                        previous_default_source,
                    );
                    return Err(error);
                }
            };

        Ok(Self {
            writer: writer_arc,
            stream: SendStream(Some(stream)),
            writer_thread: Some(writer_thread),
            app_handle: app,
            start_time: None,
            previous_default_source,
            sample_rate: config.sample_rate(),
        })
    }

    fn get_device(app: AppHandle) -> Result<(Device, Option<String>), Error> {
        let settings = crate::settings::load_settings(&app);

        if let Some(ref mic_id) = settings.mic_id {
            debug!("Resolving manually selected microphone: {}", mic_id);
            return crate::audio::microphone::resolve_device_for_recording(mic_id);
        }

        // Automatic mode: use system default
        let host = cpal::default_host();
        let default_device = host
            .default_input_device()
            .context("No default input device available")?;
        if let Ok(desc) = default_device.description() {
            debug!("Selected microphone: default ({})", desc.name());
        }
        Ok((default_device, None))
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn start(&mut self, play_sound: bool) -> Result<()> {
        if let Some(stream) = &self.stream.0 {
            stream.play().context("Failed to start stream")?;
            self.start_time = Some(std::time::Instant::now());
            if play_sound {
                sound::play_sound(&self.app_handle, sound::Sound::StartRecording);
            }
        }
        Ok(())
    }

    pub fn stop(&mut self, play_sound: bool) -> Result<()> {
        // Drop stream first to stop recording. This also drops the sample
        // sender, which lets the writer thread drain pending samples and exit.
        self.stream.0 = None;
        self.start_time = None;

        if let Some(handle) = self.writer_thread.take() {
            let drain_start = std::time::Instant::now();
            let _ = handle.join();
            debug!("Writer thread drained in {:?}", drain_start.elapsed());
        }

        // Finalize writer
        let mut result = Ok(());
        let mut writer_guard = self.writer.lock();
        if let Some(writer) = writer_guard.take() {
            result = writer.finalize().context("Failed to finalize WAV file");
            if result.is_ok() && play_sound {
                sound::play_sound(&self.app_handle, sound::Sound::StopRecording);
            }
        }

        crate::audio::microphone::restore_default_source_after_recording(
            self.previous_default_source.take(),
        );

        result
    }
}

impl Drop for AudioRecorder {
    fn drop(&mut self) {
        crate::audio::microphone::restore_default_source_after_recording(
            self.previous_default_source.take(),
        );
    }
}

struct WriterThreadCtx {
    app: AppHandle,
    limit_reached: Arc<AtomicBool>,
    recording_trigger: RecordingTrigger,
    streaming_buffer: Arc<Mutex<Vec<f32>>>,
    /// Present when the session chunks its audio; the writer pushes chunks here.
    chunk_tx: Option<Sender<ChunkJob>>,
    sample_rate: u32,
    long_dictation_active: Arc<AtomicBool>,
}

fn build_stream(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
    writer: SharedWriter,
    ctx: WriterThreadCtx,
) -> Result<(cpal::Stream, JoinHandle<()>)> {
    let (tx, rx) = mpsc::channel::<Vec<f32>>();

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => build_stream_impl::<f32>(device, config, tx),
        cpal::SampleFormat::I16 => build_stream_impl::<i16>(device, config, tx),
        cpal::SampleFormat::I32 => build_stream_impl::<i32>(device, config, tx),
        f => Err(anyhow::anyhow!("Unsupported sample format: {:?}", f)),
    }?;

    let writer_thread = spawn_writer_thread(rx, writer, ctx);

    Ok((stream, writer_thread))
}

fn build_stream_impl<T>(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
    tx: Sender<Vec<f32>>,
) -> Result<cpal::Stream>
where
    T: cpal::Sample + cpal::SizedSample + Send + 'static,
    f32: cpal::FromSample<T>,
{
    let channels = config.channels() as usize;

    let make_callback = || {
        let tx = tx.clone();
        move |data: &[T], _: &cpal::InputCallbackInfo| {
            // Real-time audio callback: blocking here (disk IO, locks, IPC)
            // makes the OS drop microphone buffers, which is heard as
            // crackling. Only downmix and hand off to the writer thread.
            let mut mono: Vec<f32> = Vec::with_capacity(data.len() / channels);
            for frame in data.chunks_exact(channels) {
                let sample = if channels == 1 {
                    frame[0].to_sample::<f32>()
                } else {
                    frame.iter().map(|&s| s.to_sample::<f32>()).sum::<f32>() / channels as f32
                };
                mono.push(sample);
            }
            let _ = tx.send(mono);
        }
    };

    let stream = crate::audio::helpers::build_input_with_buffer_fallback(
        &config.clone().into(),
        |stream_config| {
            device.build_input_stream(
                stream_config,
                make_callback(),
                |err| error!("Stream error: {}", err),
                None,
            )
        },
    )?;

    Ok(stream)
}

fn spawn_writer_thread(
    rx: Receiver<Vec<f32>>,
    writer: SharedWriter,
    ctx: WriterThreadCtx,
) -> JoinHandle<()> {
    let WriterThreadCtx {
        app,
        limit_reached: limit_reached_flag,
        recording_trigger,
        streaming_buffer,
        chunk_tx,
        sample_rate,
        long_dictation_active,
    } = ctx;
    std::thread::spawn(move || {
        // State for simple RMS + EMA smoothing and throttled emission
        let mut acc_sum_squares: f32 = 0.0;
        let mut acc_count: usize = 0;
        let mut ema_level: f32 = 0.0;
        let alpha: f32 = 0.35; // smoothing factor
        let mut last_emit = std::time::Instant::now();
        let start_time = std::time::Instant::now();
        let mut local_limit_triggered = false;

        let is_wake_word = recording_trigger == RecordingTrigger::WakeWord;
        let settings = crate::settings::load_settings(&app);
        let silence_auto_stop_ms = if settings.silence_timeout_ms == 0 {
            0
        } else {
            settings.silence_timeout_ms.clamp(500, 5000)
        };
        let mut silence_start: Option<std::time::Instant> = None;
        let mut silence_auto_stop_triggered = false;
        let mut has_speech_started = false;

        let chunk_silence_ms = settings.long_dictation_silence_ms.clamp(250, 3000);
        let mut chunker = chunk_tx.map(|tx| Chunker::new(tx, sample_rate, chunk_silence_ms));

        let is_long_dictation = long_dictation_active.load(Ordering::SeqCst);
        let long_dictation_silence_ms = settings.long_dictation_silence_ms.clamp(250, 3000);
        let mut long_segment_emitted = false;
        let mut long_vad = LongDictationVad::new();
        let mut long_silence_start: Option<std::time::Instant> = None;

        while let Ok(mono) = rx.recv() {
            if !local_limit_triggered
                && start_time.elapsed()
                    >= std::time::Duration::from_secs(MAX_RECORDING_DURATION_SECS)
            {
                local_limit_triggered = true;
                limit_reached_flag.store(true, Ordering::SeqCst);
                // The duration cap no longer aborts the session: chunks are
                // force-cut at CHUNK_FORCE_CUT_SECS so this is never reached, but
                // it still caps a single chunk if both cuts failed to fire.
                warn!("Recording duration cap reached on a single chunk");
                continue;
            }

            if local_limit_triggered {
                continue;
            }

            {
                let mut recorder = writer.lock();
                if let Some(writer) = recorder.as_mut() {
                    for &sample in &mono {
                        // write to WAV
                        let sample_i16 = (sample * i16::MAX as f32) as i16;
                        if let Err(e) = writer.write_sample(sample_i16) {
                            error!("Error writing sample: {}", e);
                        }

                        // accumulate for RMS
                        acc_sum_squares += sample * sample;
                        acc_count += 1;
                    }
                }
            }

            // Standard chunking feeds the chunker and the preview buffer in
            // parallel. Long dictation never consumes the streaming buffer (no
            // preview), so it is skipped to avoid unbounded growth.
            if let Some(chunker) = chunker.as_mut() {
                chunker.push_samples(&mono);
            }
            if !is_long_dictation && !mono.is_empty() {
                streaming_buffer.lock().extend_from_slice(&mono);
            }

            // Throttle to ~30 FPS
            if last_emit.elapsed() >= std::time::Duration::from_millis(33) {
                if acc_count > 0 {
                    let rms = (acc_sum_squares / acc_count as f32).sqrt();
                    // Linear gain only, the frontend applies the non-linear stretch.
                    let mut level = (rms * 3.0).min(1.0);
                    if level < 0.005 {
                        level = 0.0;
                    }
                    // EMA smoothing
                    ema_level = alpha * level + (1.0 - alpha) * ema_level;
                    let _ = app.emit("mic-level", ema_level);
                    if let Some(overlay_window) = app.get_webview_window("recording_overlay") {
                        let _ = overlay_window.emit("mic-level", ema_level);
                    }

                    // Long dictation ends on a stop wake word, never on silence,
                    // so the silence auto-stop is disabled while it is active.
                    if is_wake_word
                        && !is_long_dictation
                        && !silence_auto_stop_triggered
                        && silence_auto_stop_ms > 0
                    {
                        if rms >= SILENCE_AUTO_STOP_SPEECH_THRESHOLD {
                            if !has_speech_started {
                                info!("Wake word auto-stop: speech detected (rms={:.4})", rms);
                            }
                            has_speech_started = true;
                        }

                        if has_speech_started {
                            if rms < SILENCE_AUTO_STOP_THRESHOLD {
                                if silence_start.is_none() {
                                    silence_start = Some(std::time::Instant::now());
                                    trace!("Wake word auto-stop: silence started (rms={:.4})", rms);
                                }
                                if let Some(start) = silence_start {
                                    if start.elapsed()
                                        >= std::time::Duration::from_millis(silence_auto_stop_ms)
                                    {
                                        silence_auto_stop_triggered = true;
                                        info!(
                                            "Wake word auto-stop: stopping after {}ms silence",
                                            silence_auto_stop_ms
                                        );
                                        let app = app.clone();
                                        std::thread::spawn(move || {
                                            crate::shortcuts::force_stop_recording(&app);
                                        });
                                    }
                                }
                            } else {
                                silence_start = None;
                            }
                        }
                    }

                    if is_long_dictation && !long_segment_emitted {
                        match long_vad.update(rms) {
                            LongDictationSilence::Silent => {
                                let start =
                                    long_silence_start.get_or_insert_with(std::time::Instant::now);
                                if start.elapsed()
                                    >= std::time::Duration::from_millis(long_dictation_silence_ms)
                                {
                                    long_segment_emitted = true;
                                    info!(
                                        "Long dictation: segment boundary after {}ms silence",
                                        long_dictation_silence_ms
                                    );
                                    let _ = app.emit("long-dictation-segment", ());
                                }
                            }
                            LongDictationSilence::Active => {
                                long_silence_start = None;
                            }
                            LongDictationSilence::NotStarted => {}
                        }
                    }

                    if let Some(chunker) = chunker.as_mut() {
                        chunker.on_throttle_tick(rms);
                    }

                    acc_sum_squares = 0.0;
                    acc_count = 0;
                } else {
                    let _ = app.emit("mic-level", 0.0f32);
                    if let Some(overlay_window) = app.get_webview_window("recording_overlay") {
                        let _ = overlay_window.emit("mic-level", 0.0f32);
                    }
                }
                last_emit = std::time::Instant::now();
            }
        }

        if let Some(chunker) = chunker {
            chunker.flush_remaining();
        }
    })
}

/// Accumulates the current chunk's native-rate mono samples and cuts it into the
/// FIFO: a detected silence past CHUNK_SILENCE_ARM_SECS, or a forced cut at
/// CHUNK_FORCE_CUT_SECS. A forced cut keeps ~1s of overlap as the next chunk's
/// head so a word straddling the cut can be deduplicated downstream.
struct Chunker {
    tx: Sender<ChunkJob>,
    sample_rate: u32,
    arm_samples: usize,
    force_samples: usize,
    overlap_samples: usize,
    silence_ms: u64,
    seq: u64,
    samples: Vec<f32>,
    overlap_prefix: usize,
    vad: LongDictationVad,
    silence_start: Option<std::time::Instant>,
}

impl Chunker {
    fn new(tx: Sender<ChunkJob>, sample_rate: u32, silence_ms: u64) -> Self {
        let sr = sample_rate as usize;
        Self {
            tx,
            sample_rate,
            arm_samples: CHUNK_SILENCE_ARM_SECS as usize * sr,
            force_samples: CHUNK_FORCE_CUT_SECS as usize * sr,
            overlap_samples: (CHUNK_FORCED_OVERLAP_SECS * sample_rate as f32) as usize,
            silence_ms,
            seq: 0,
            samples: Vec::new(),
            overlap_prefix: 0,
            vad: LongDictationVad::new(),
            silence_start: None,
        }
    }

    fn push_samples(&mut self, mono: &[f32]) {
        self.samples.extend_from_slice(mono);
    }

    fn on_throttle_tick(&mut self, rms: f32) {
        if self.samples.len() >= self.force_samples {
            self.cut_forced();
            return;
        }

        if self.samples.len() < self.arm_samples {
            return;
        }

        match self.vad.update(rms) {
            LongDictationSilence::Silent => {
                let start = self
                    .silence_start
                    .get_or_insert_with(std::time::Instant::now);
                if start.elapsed() >= std::time::Duration::from_millis(self.silence_ms) {
                    self.cut_on_silence();
                }
            }
            LongDictationSilence::Active => self.silence_start = None,
            LongDictationSilence::NotStarted => {}
        }
    }

    fn cut_on_silence(&mut self) {
        let samples = std::mem::take(&mut self.samples);
        debug!(
            "Standard chunking: silence cut at {:.1}s ({} samples, seq {})",
            samples.len() as f32 / self.sample_rate.max(1) as f32,
            samples.len(),
            self.seq
        );
        self.emit(samples, self.overlap_prefix);
        self.overlap_prefix = 0;
        self.reset_silence_state();
    }

    fn cut_forced(&mut self) {
        let overlap_start = self.samples.len().saturating_sub(self.overlap_samples);
        let tail = self.samples[overlap_start..].to_vec();
        let samples = std::mem::take(&mut self.samples);
        debug!(
            "Standard chunking: forced cut at {:.1}s ({} samples, seq {})",
            samples.len() as f32 / self.sample_rate.max(1) as f32,
            samples.len(),
            self.seq
        );
        let prefix = self.overlap_prefix;
        self.emit(samples, prefix);
        // The retained tail becomes both the next chunk's head and its overlap.
        self.overlap_prefix = tail.len();
        self.samples = tail;
        self.reset_silence_state();
    }

    fn flush_remaining(mut self) {
        if !self.samples.is_empty() {
            let samples = std::mem::take(&mut self.samples);
            let prefix = self.overlap_prefix;
            self.emit(samples, prefix);
        }
    }

    fn emit(&mut self, samples: Vec<f32>, overlap_prefix: usize) {
        let job = ChunkJob::Audio {
            seq: self.seq,
            samples,
            sample_rate: self.sample_rate,
            overlap_prefix,
        };
        if let Err(e) = self.tx.send(job) {
            error!("Chunking: failed to push chunk (seq {}): {}", self.seq, e);
        }
        self.seq = self.seq.saturating_add(1);
    }

    fn reset_silence_state(&mut self) {
        self.vad = LongDictationVad::new();
        self.silence_start = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vad_stays_not_started_below_speech_threshold() {
        let mut vad = LongDictationVad::new();
        // A faint signal under the speech threshold never arms speech.
        for _ in 0..50 {
            assert_eq!(vad.update(0.005), LongDictationSilence::NotStarted);
        }
    }

    #[test]
    fn vad_arms_speech_on_quiet_word_via_ema() {
        let mut vad = LongDictationVad::new();
        // A quiet word just above the speech threshold arms speech once the EMA
        // converges, even though a single raw frame is borderline.
        let mut armed = false;
        for _ in 0..20 {
            if vad.update(0.02) != LongDictationSilence::NotStarted {
                armed = true;
                break;
            }
        }
        assert!(armed, "EMA should arm speech on a sustained quiet word");
    }

    #[test]
    fn vad_micro_peak_during_silence_does_not_return_active() {
        let mut vad = LongDictationVad::new();
        // Arm speech with clear speech-level input.
        for _ in 0..20 {
            vad.update(0.05);
        }
        // Settle into silence.
        for _ in 0..20 {
            vad.update(0.0);
        }
        assert_eq!(vad.update(0.0), LongDictationSilence::Silent);
        // A single micro-peak frame is absorbed by the EMA and must not flip
        // the state back to Active (which would reset the silence timer).
        assert_eq!(vad.update(0.03), LongDictationSilence::Silent);
    }

    #[test]
    fn vad_sustained_speech_returns_active_and_resets_silence() {
        let mut vad = LongDictationVad::new();
        for _ in 0..20 {
            vad.update(0.05);
        }
        assert_eq!(vad.update(0.05), LongDictationSilence::Active);
    }
}
