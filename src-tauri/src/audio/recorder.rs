use crate::audio::helpers::create_wav_writer;
use crate::audio::sound;
use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::WavWriter;
use parking_lot::Mutex;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};

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
}

impl AudioRecorder {
    pub fn new(app: AppHandle, file_path: &Path) -> Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .context("No input device available")?;
        let config = device
            .default_input_config()
            .context("No input config available")?;

        let writer = create_wav_writer(file_path, &config)?;
        let writer_arc = Arc::new(Mutex::new(Some(writer)));

        let stream = build_stream(&device, &config, writer_arc.clone(), app.clone())?;

        Ok(Self {
            writer: writer_arc,
            stream: SendStream(Some(stream)),
            app_handle: app,
        })
    }

    pub fn start(&self) -> Result<()> {
        if let Some(stream) = &self.stream.0 {
            stream.play().context("Failed to start stream")?;
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

        // Finalize writer
        let mut writer_guard = self.writer.lock();
        if let Some(writer) = writer_guard.take() {
            writer.finalize().context("Failed to finalize WAV file")?;
            let settings = crate::settings::load_settings(&self.app_handle);
            if settings.sound_enabled {
                sound::play_sound(&self.app_handle, sound::Sound::StopRecording);
            }
        }
        Ok(())
    }
}

fn build_stream(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
    writer: SharedWriter,
    app: AppHandle,
) -> Result<cpal::Stream> {
    match config.sample_format() {
        cpal::SampleFormat::F32 => build_stream_impl::<f32>(device, config, writer, app),
        cpal::SampleFormat::I16 => build_stream_impl::<i16>(device, config, writer, app),
        cpal::SampleFormat::I32 => build_stream_impl::<i32>(device, config, writer, app),
        f => Err(anyhow::anyhow!("Unsupported sample format: {:?}", f)),
    }
}

fn build_stream_impl<T>(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
    writer: SharedWriter,
    app: AppHandle,
) -> Result<cpal::Stream>
where
    T: cpal::Sample + cpal::SizedSample + Send + 'static,
    f32: cpal::FromSample<T>,
{
    let channels = config.channels() as usize;
    let _sample_rate = config.sample_rate().0 as f32;

    // State for simple RMS + EMA smoothing and throttled emission
    let mut acc_sum_squares: f32 = 0.0;
    let mut acc_count: usize = 0;
    let mut ema_level: f32 = 0.0;
    let alpha: f32 = 0.35; // smoothing factor
    let mut last_emit = std::time::Instant::now();

    let app_handle = app.clone();
    let writer_clone = writer.clone();

    let stream = device.build_input_stream(
        &config.clone().into(),
        move |data: &[T], _: &cpal::InputCallbackInfo| {
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
                        eprintln!("Error writing sample: {}", e);
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
                    // also forward to overlay window if present
                    if let Some(overlay_window) = app_handle.get_webview_window("recording_overlay")
                    {
                        let _ = overlay_window.emit("mic-level", ema_level);
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
        |err| eprintln!("Stream error: {}", err),
        None,
    )?;

    Ok(stream)
}
