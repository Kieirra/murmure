use crate::audio::helpers::create_wav_writer;
use crate::audio::sound;
use crate::audio::types::RecordingTrigger;
use anyhow::{Context, Error, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Device;
use hound::WavWriter;
use log::{debug, error, info, trace};
use parking_lot::Mutex;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};

const MAX_RECORDING_DURATION_SECS: u64 = 300; // 5 min
const SILENCE_AUTO_STOP_THRESHOLD: f32 = 0.03;
const SILENCE_AUTO_STOP_SPEECH_THRESHOLD: f32 = 0.03;

type WavWriterType = WavWriter<BufWriter<File>>;
type SharedWriter = Arc<Mutex<Option<WavWriterType>>>;

// Wrapper to safely store Stream. Stream on macOS doesn't implement Send.
pub struct SendStream(pub Option<cpal::Stream>);
unsafe impl Send for SendStream {}
unsafe impl Sync for SendStream {}

/// Internal abstraction over an audio capture source.
///
/// The default impl `CpalAudioSource` wraps the real cpal microphone capture.
/// When the `audio-injection` Cargo feature is enabled, `WavFileAudioSource`
/// provides an offline replacement that replays a WAV file at real-time pace
/// so e2e tests can drive the pipeline deterministically.
pub(crate) trait AudioSource: Send + Sync {
    fn start(&mut self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
    fn sample_rate(&self) -> u32;
}

/// Cpal-backed live audio source (production default). Encapsulates the
/// previous `AudioRecorder` body verbatim; behaviour is unchanged.
pub(crate) struct CpalAudioSource {
    writer: SharedWriter,
    stream: SendStream,
    app_handle: AppHandle,
    start_time: Option<std::time::Instant>,
    previous_default_source: Option<String>,
    sample_rate: u32,
}

impl CpalAudioSource {
    pub fn new(app: AppHandle, file_path: &Path, limit_reached: Arc<AtomicBool>) -> Result<Self> {
        // Reset the limit flag at the start of each recording
        limit_reached.store(false, Ordering::SeqCst);

        let audio_state = app.state::<crate::audio::types::AudioState>();
        let recording_trigger = audio_state.get_recording_trigger();

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

        let stream = match build_stream(
            &device,
            &config,
            writer_arc.clone(),
            app.clone(),
            limit_reached,
            recording_trigger,
            streaming_buf,
        ) {
            Ok(stream) => stream,
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
}

impl AudioSource for CpalAudioSource {
    fn start(&mut self) -> Result<()> {
        if let Some(stream) = &self.stream.0 {
            stream.play().context("Failed to start stream")?;
            self.start_time = Some(std::time::Instant::now());
            let settings = crate::settings::load_settings(&self.app_handle);
            if settings.sound_enabled {
                sound::play_sound(&self.app_handle, sound::Sound::StartRecording);
            }
        }
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        // Drop stream first to stop recording
        self.stream.0 = None;
        self.start_time = None;

        // Finalize writer
        let mut result = Ok(());
        let mut writer_guard = self.writer.lock();
        if let Some(writer) = writer_guard.take() {
            result = writer.finalize().context("Failed to finalize WAV file");
            if result.is_ok() {
                let settings = crate::settings::load_settings(&self.app_handle);
                if settings.sound_enabled {
                    sound::play_sound(&self.app_handle, sound::Sound::StopRecording);
                }
            }
        }

        crate::audio::microphone::restore_default_source_after_recording(
            self.previous_default_source.take(),
        );

        result
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
}

impl Drop for CpalAudioSource {
    fn drop(&mut self) {
        crate::audio::microphone::restore_default_source_after_recording(
            self.previous_default_source.take(),
        );
    }
}

/// Public façade that owns a boxed `AudioSource`. The rest of the crate keeps
/// using `AudioRecorder::new(app, path, limit_reached)` exactly as before; the
/// trait is an implementation detail of this module.
pub struct AudioRecorder {
    source: Box<dyn AudioSource>,
}

impl AudioRecorder {
    pub fn new(app: AppHandle, file_path: &Path, limit_reached: Arc<AtomicBool>) -> Result<Self> {
        let source = CpalAudioSource::new(app, file_path, limit_reached)?;
        Ok(Self {
            source: Box::new(source),
        })
    }

    /// Test-only constructor that injects a pre-recorded WAV instead of the live
    /// microphone. Gated behind the `audio-injection` feature; the symbol does
    /// not exist in release binaries.
    #[cfg(feature = "audio-injection")]
    pub fn new_with_wav(
        app: AppHandle,
        file_path: &Path,
        limit_reached: Arc<AtomicBool>,
        wav_path: std::path::PathBuf,
    ) -> Result<Self> {
        let source = WavFileAudioSource::new(app, file_path, limit_reached, wav_path)?;
        Ok(Self {
            source: Box::new(source),
        })
    }

    pub fn sample_rate(&self) -> u32 {
        self.source.sample_rate()
    }

    pub fn start(&mut self) -> Result<()> {
        self.source.start()
    }

    pub fn stop(&mut self) -> Result<()> {
        self.source.stop()
    }
}

fn build_stream(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
    writer: SharedWriter,
    app: AppHandle,
    limit_reached: Arc<AtomicBool>,
    recording_trigger: RecordingTrigger,
    streaming_buffer: Arc<Mutex<Vec<f32>>>,
) -> Result<cpal::Stream> {
    match config.sample_format() {
        cpal::SampleFormat::F32 => build_stream_impl::<f32>(
            device,
            config,
            writer,
            app,
            limit_reached.clone(),
            recording_trigger,
            streaming_buffer,
        ),
        cpal::SampleFormat::I16 => build_stream_impl::<i16>(
            device,
            config,
            writer,
            app,
            limit_reached.clone(),
            recording_trigger,
            streaming_buffer,
        ),
        cpal::SampleFormat::I32 => build_stream_impl::<i32>(
            device,
            config,
            writer,
            app,
            limit_reached.clone(),
            recording_trigger,
            streaming_buffer,
        ),
        f => Err(anyhow::anyhow!("Unsupported sample format: {:?}", f)),
    }
}

fn build_stream_impl<T>(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
    writer: SharedWriter,
    app: AppHandle,
    limit_reached_flag: Arc<AtomicBool>,
    recording_trigger: RecordingTrigger,
    streaming_buffer: Arc<Mutex<Vec<f32>>>,
) -> Result<cpal::Stream>
where
    T: cpal::Sample + cpal::SizedSample + Send + 'static,
    f32: cpal::FromSample<T>,
{
    let channels = config.channels() as usize;

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

    let app_handle = app.clone();
    let writer_clone = writer.clone();
    let mut mono_cache: Vec<f32> = Vec::new();

    let stream = device.build_input_stream(
        &config.clone().into(),
        move |data: &[T], _: &cpal::InputCallbackInfo| {
            if !local_limit_triggered
                && start_time.elapsed()
                    >= std::time::Duration::from_secs(MAX_RECORDING_DURATION_SECS)
            {
                local_limit_triggered = true;
                limit_reached_flag.store(true, Ordering::SeqCst);
                let _ = app_handle.emit("recording-limit-reached", ());
                return;
            }

            if local_limit_triggered {
                return;
            }

            mono_cache.clear();

            let mut recorder = writer_clone.lock();
            if let Some(writer) = recorder.as_mut() {
                for frame in data.chunks_exact(channels) {
                    let sample = if channels == 1 {
                        frame[0].to_sample::<f32>()
                    } else {
                        frame.iter().map(|&s| s.to_sample::<f32>()).sum::<f32>() / channels as f32
                    };

                    // write to WAV
                    let sample_i16 = (sample * i16::MAX as f32) as i16;
                    if let Err(e) = writer.write_sample(sample_i16) {
                        error!("Error writing sample: {}", e);
                    }

                    // accumulate for RMS
                    acc_sum_squares += sample * sample;
                    acc_count += 1;

                    mono_cache.push(sample);
                }
            }

            if !mono_cache.is_empty() {
                streaming_buffer.lock().extend_from_slice(&mono_cache);
            }

            // Throttle to ~30 FPS
            if last_emit.elapsed() >= std::time::Duration::from_millis(33) {
                if acc_count > 0 {
                    let rms = (acc_sum_squares / acc_count as f32).sqrt();
                    // Normalize a bit and clamp
                    let mut level = (rms * 1.5).min(1.0);
                    // simple noise gate
                    if level < 0.02 {
                        level = 0.0;
                    }
                    // EMA smoothing
                    ema_level = alpha * level + (1.0 - alpha) * ema_level;
                    let _ = app_handle.emit("mic-level", ema_level);
                    if let Some(overlay_window) = app_handle.get_webview_window("recording_overlay")
                    {
                        let _ = overlay_window.emit("mic-level", ema_level);
                    }

                    if is_wake_word && !silence_auto_stop_triggered && silence_auto_stop_ms > 0 {
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
                                        let app = app_handle.clone();
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

                    acc_sum_squares = 0.0;
                    acc_count = 0;
                } else {
                    let _ = app_handle.emit("mic-level", 0.0f32);
                    if let Some(overlay_window) = app_handle.get_webview_window("recording_overlay")
                    {
                        let _ = overlay_window.emit("mic-level", 0.0f32);
                    }
                }
                last_emit = std::time::Instant::now();
            }
        },
        |err| error!("Stream error: {}", err),
        None,
    )?;

    Ok(stream)
}

// ---------------------------------------------------------------------------
// Test-only WAV file injection source. Entire section gated behind the
// `audio-injection` feature so the production binary contains no trace of it.
// ---------------------------------------------------------------------------

#[cfg(feature = "audio-injection")]
pub(crate) struct WavFileAudioSource {
    wav_path: std::path::PathBuf,
    writer: SharedWriter,
    app_handle: AppHandle,
    streaming_buffer: Arc<Mutex<Vec<f32>>>,
    playback_thread: Option<std::thread::JoinHandle<()>>,
    stop_flag: Arc<AtomicBool>,
    sample_rate: u32,
}

#[cfg(feature = "audio-injection")]
impl WavFileAudioSource {
    pub fn new(
        app: AppHandle,
        file_path: &Path,
        _limit_reached: Arc<AtomicBool>,
        wav_path: std::path::PathBuf,
    ) -> Result<Self> {
        if !wav_path.exists() {
            return Err(anyhow::anyhow!(
                "WAV fixture not found: {}",
                wav_path.display()
            ));
        }

        // Peek the WAV header to restore the original sample rate downstream.
        let reader = hound::WavReader::open(&wav_path)
            .with_context(|| format!("Failed to open WAV fixture: {}", wav_path.display()))?;
        let spec = reader.spec();
        let sample_rate = spec.sample_rate;
        drop(reader);

        // Mirror the on-disk WAV the production path produces. The pipeline
        // post-processing reads the recording from this file path so it must
        // exist and be valid.
        let writer = create_injection_wav_writer(file_path, sample_rate)
            .context("Failed to create WAV writer for injection source")?;
        let writer_arc = Arc::new(Mutex::new(Some(writer)));

        let streaming_buffer = {
            let state = app.state::<crate::audio::types::AudioState>();
            state.streaming_buffer.clone()
        };

        Ok(Self {
            wav_path,
            writer: writer_arc,
            app_handle: app,
            streaming_buffer,
            playback_thread: None,
            stop_flag: Arc::new(AtomicBool::new(false)),
            sample_rate,
        })
    }
}

#[cfg(feature = "audio-injection")]
impl AudioSource for WavFileAudioSource {
    fn start(&mut self) -> Result<()> {
        let settings = crate::settings::load_settings(&self.app_handle);
        if settings.sound_enabled {
            sound::play_sound(&self.app_handle, sound::Sound::StartRecording);
        }

        let wav_path = self.wav_path.clone();
        let writer = self.writer.clone();
        let streaming_buffer = self.streaming_buffer.clone();
        let stop_flag = self.stop_flag.clone();
        let sample_rate = self.sample_rate;

        stop_flag.store(false, Ordering::SeqCst);

        let handle = std::thread::Builder::new()
            .name("wav-injection-playback".into())
            .spawn(move || {
                if let Err(e) =
                    playback_loop(wav_path, writer, streaming_buffer, stop_flag, sample_rate)
                {
                    error!("WAV injection playback failed: {}", e);
                }
            })
            .context("Failed to spawn WAV injection playback thread")?;

        self.playback_thread = Some(handle);
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        self.stop_flag.store(true, Ordering::SeqCst);
        if let Some(handle) = self.playback_thread.take() {
            let _ = handle.join();
        }

        let mut result = Ok(());
        let mut writer_guard = self.writer.lock();
        if let Some(writer) = writer_guard.take() {
            result = writer
                .finalize()
                .context("Failed to finalize injected WAV file");
            if result.is_ok() {
                let settings = crate::settings::load_settings(&self.app_handle);
                if settings.sound_enabled {
                    sound::play_sound(&self.app_handle, sound::Sound::StopRecording);
                }
            }
        }

        result
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
}

#[cfg(feature = "audio-injection")]
fn create_injection_wav_writer(
    path: &Path,
    sample_rate: u32,
) -> Result<WavWriter<BufWriter<File>>> {
    let file = File::create(path).context("Failed to create injected WAV file")?;
    let writer = BufWriter::new(file);
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    WavWriter::new(writer, spec).context("Failed to create injected WAV writer")
}

#[cfg(feature = "audio-injection")]
fn playback_loop(
    wav_path: std::path::PathBuf,
    writer: SharedWriter,
    streaming_buffer: Arc<Mutex<Vec<f32>>>,
    stop_flag: Arc<AtomicBool>,
    sample_rate: u32,
) -> Result<()> {
    use std::time::{Duration, Instant};

    let mut reader = hound::WavReader::open(&wav_path)
        .with_context(|| format!("Failed to open WAV fixture: {}", wav_path.display()))?;
    let spec = reader.spec();
    let channels = spec.channels.max(1) as usize;

    // Real-time pacing: chunks of ~33ms (matches the cpal callback emission
    // cadence the downstream VAD / streaming code was tuned against).
    let chunk_frames = (sample_rate as usize / 30).max(1);
    let chunk_period =
        Duration::from_micros(((chunk_frames as u64) * 1_000_000) / sample_rate as u64);

    let samples: Vec<i16> = reader
        .samples::<i16>()
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to decode WAV samples")?;

    let mono: Vec<i16> = match channels {
        1 => samples,
        ch => {
            let mut out = Vec::with_capacity(samples.len() / ch);
            for frame in samples.chunks_exact(ch) {
                let sum: i32 = frame.iter().map(|&s| s as i32).sum();
                let avg = (sum / ch as i32).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
                out.push(avg);
            }
            out
        }
    };

    let mut next_tick = Instant::now();
    let mut idx = 0;
    let total = mono.len();

    while idx < total && !stop_flag.load(Ordering::SeqCst) {
        let end = (idx + chunk_frames).min(total);
        let chunk = &mono[idx..end];
        idx = end;

        let f32_chunk: Vec<f32> = chunk.iter().map(|&s| s as f32 / i16::MAX as f32).collect();

        {
            let mut guard = writer.lock();
            if let Some(w) = guard.as_mut() {
                for &s in chunk {
                    if let Err(e) = w.write_sample(s) {
                        error!("Error writing injected sample: {}", e);
                    }
                }
            }
        }

        if !f32_chunk.is_empty() {
            streaming_buffer.lock().extend_from_slice(&f32_chunk);
        }

        next_tick += chunk_period;
        let now = Instant::now();
        if next_tick > now {
            std::thread::sleep(next_tick - now);
        } else {
            // Reset pacing reference if we fell behind, otherwise we would
            // race-flood the buffer on slow hosts.
            next_tick = now;
        }
    }

    debug!("WAV injection playback finished (frames={})", idx);
    Ok(())
}
