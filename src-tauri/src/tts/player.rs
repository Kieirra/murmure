use log::error;
use rodio::buffer::SamplesBuffer;
use std::sync::mpsc::{channel, Sender};
use std::thread;

pub enum PlayerCommand {
    Play(Vec<f32>),
    Stop,
    IsPlaying(std::sync::mpsc::Sender<bool>),
}

pub struct TtsPlayer {
    tx: Sender<PlayerCommand>,
}

impl TtsPlayer {
    pub fn new() -> Self {
        let (tx, rx) = channel::<PlayerCommand>();

        thread::spawn(move || {
            let stream_handle = match rodio::OutputStreamBuilder::from_default_device() {
                Ok(builder) => match builder.open_stream_or_fallback() {
                    Ok(stream) => stream,
                    Err(e) => {
                        error!("Failed to open TTS audio stream: {}", e);
                        return;
                    }
                },
                Err(e) => {
                    error!("Failed to get default audio device for TTS: {}", e);
                    return;
                }
            };

            let mut sink = rodio::Sink::connect_new(stream_handle.mixer());

            while let Ok(cmd) = rx.recv() {
                match cmd {
                    PlayerCommand::Play(samples) => {
                        let source = SamplesBuffer::new(1, 24000, samples);
                        sink.append(source);
                    }
                    PlayerCommand::Stop => {
                        sink.stop();
                        sink = rodio::Sink::connect_new(stream_handle.mixer());
                    }
                    PlayerCommand::IsPlaying(reply_tx) => {
                        let is_playing = !sink.empty();
                        let _ = reply_tx.send(is_playing);
                    }
                }
            }
        });

        Self { tx }
    }

    pub fn play(&self, samples: Vec<f32>) {
        let _ = self.tx.send(PlayerCommand::Play(samples));
    }

    pub fn stop(&self) {
        let _ = self.tx.send(PlayerCommand::Stop);
    }

    pub fn is_playing(&self) -> bool {
        let (reply_tx, reply_rx) = std::sync::mpsc::channel();
        if self.tx.send(PlayerCommand::IsPlaying(reply_tx)).is_ok() {
            reply_rx.recv().unwrap_or(false)
        } else {
            false
        }
    }
}
