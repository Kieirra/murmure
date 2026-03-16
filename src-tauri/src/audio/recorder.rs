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
const SILENCE_AUTO_STOP_MS: u64 = 1500;
const SILENCE_AUTO_STOP_THRESHOLD: f32 = 0.03;
const SILENCE_AUTO_STOP_SPEECH_THRESHOLD: f32 = 0.03;

type WavWriterType = WavWriter<BufWriter<File>>;
type SharedWriter = Arc<Mutex<Option<WavWriterType>>>;

// Wrapper to safely store Stream. Stream on macOS doesn't implement Send.
pub struct SendStream(pub Option<cpal::Stream>);
unsafe impl Send for SendStream {}
unsafe impl Sync for SendStream {}

pub struct AudioRecorder {
    writer: SharedWriter,
    stream: SendStream,
    app_handle: AppHandle,
    start_time: Option<std::time::Instant>,
    previous_default_source: Option<String>,
}

impl AudioRecorder {
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

        let stream = match build_stream(
            &device,
            &config,
            writer_arc.clone(),
            app.clone(),
            limit_reached,
            recording_trigger,
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

    pub fn start(&mut self) -> Result<()> {
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

    pub fn stop(&mut self) -> Result<()> {
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
}

impl Drop for AudioRecorder {
    fn drop(&mut self) {
        crate::audio::microphone::restore_default_source_after_recording(
            self.previous_default_source.take(),
        );
    }
}

fn build_stream(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
    writer: SharedWriter,
    app: AppHandle,
    limit_reached: Arc<AtomicBool>,
    recording_trigger: RecordingTrigger,
) -> Result<cpal::Stream> {
    match config.sample_format() {
        cpal::SampleFormat::F32 => build_stream_impl::<f32>(
            device,
            config,
            writer,
            app,
            limit_reached.clone(),
            recording_trigger,
        ),
        cpal::SampleFormat::I16 => build_stream_impl::<i16>(
            device,
            config,
            writer,
            app,
            limit_reached.clone(),
            recording_trigger,
        ),
        cpal::SampleFormat::I32 => build_stream_impl::<i32>(
            device,
            config,
            writer,
            app,
            limit_reached.clone(),
            recording_trigger,
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
    let mut silence_start: Option<std::time::Instant> = None;
    let mut silence_auto_stop_triggered = false;
    let mut has_speech_started = false;

    let app_handle = app.clone();
    let writer_clone = writer.clone();

    let stream = device.build_input_stream(
        &config.clone().into(),
        move |data: &[T], _: &cpal::InputCallbackInfo| {
            // Check for duration limit
            if !local_limit_triggered
                && start_time.elapsed()
                    >= std::time::Duration::from_secs(MAX_RECORDING_DURATION_SECS)
            {
                local_limit_triggered = true;
                limit_reached_flag.store(true, Ordering::SeqCst);
                let _ = app_handle.emit("recording-limit-reached", ());
            }

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
                }
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

                    if is_wake_word && !silence_auto_stop_triggered {
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
                                        >= std::time::Duration::from_millis(SILENCE_AUTO_STOP_MS)
                                    {
                                        silence_auto_stop_triggered = true;
                                        info!(
                                            "Wake word auto-stop: stopping after {}ms silence",
                                            SILENCE_AUTO_STOP_MS
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
