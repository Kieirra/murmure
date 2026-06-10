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
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread::JoinHandle;
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

        let (stream, writer_thread) = match build_stream(
            &device,
            &config,
            writer_arc.clone(),
            app.clone(),
            limit_reached,
            recording_trigger,
            streaming_buf,
        ) {
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
    streaming_buffer: Arc<Mutex<Vec<f32>>>,
) -> Result<(cpal::Stream, JoinHandle<()>)> {
    let (tx, rx) = mpsc::channel::<Vec<f32>>();

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => build_stream_impl::<f32>(device, config, tx),
        cpal::SampleFormat::I16 => build_stream_impl::<i16>(device, config, tx),
        cpal::SampleFormat::I32 => build_stream_impl::<i32>(device, config, tx),
        f => Err(anyhow::anyhow!("Unsupported sample format: {:?}", f)),
    }?;

    let writer_thread = spawn_writer_thread(
        rx,
        writer,
        app,
        limit_reached,
        recording_trigger,
        streaming_buffer,
    );

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
    app: AppHandle,
    limit_reached_flag: Arc<AtomicBool>,
    recording_trigger: RecordingTrigger,
    streaming_buffer: Arc<Mutex<Vec<f32>>>,
) -> JoinHandle<()> {
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

        while let Ok(mono) = rx.recv() {
            if !local_limit_triggered
                && start_time.elapsed()
                    >= std::time::Duration::from_secs(MAX_RECORDING_DURATION_SECS)
            {
                local_limit_triggered = true;
                limit_reached_flag.store(true, Ordering::SeqCst);
                let _ = app.emit("recording-limit-reached", ());
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

            if !mono.is_empty() {
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
    })
}
