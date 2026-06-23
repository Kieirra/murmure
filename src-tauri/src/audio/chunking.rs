use crate::audio::live_text::{LiveTextSilence, LiveTextVad};
use crate::audio::pipeline::{process_chunk, ChunkOutcome};
use crate::wake_word::wake_word::normalize_text;
use log::{debug, error};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use tauri::{AppHandle, Emitter};

const MERGE_WINDOW_WORDS: usize = 6;

/// Default arm length: once the current chunk reaches this, a detected silence cuts it.
pub const CHUNK_SILENCE_ARM_SECS: u32 = 15;
/// Arm length used for live long-dictation chunking.
pub const LONG_DICTATION_SILENCE_ARM_SECS: u32 = 3;
/// Hard cut applied when no silence has been detected by this length.
const CHUNK_FORCE_CUT_SECS: u32 = 60;
/// Tail kept as the next chunk's head so a word straddling a forced cut can be deduped.
const CHUNK_FORCED_OVERLAP_SECS: f32 = 1.0;

pub enum ChunkJob {
    Audio {
        seq: u64,
        samples: Vec<f32>,
        sample_rate: u32,
        overlap_prefix: usize,
    },
    Finalize,
}

pub struct ChunkPipeline {
    tx: Sender<ChunkJob>,
    accumulated: Arc<Mutex<String>>,
    worker: Option<JoinHandle<()>>,
    arm_secs: u32,
}

impl ChunkPipeline {
    pub fn start(
        app: &AppHandle,
        on_chunk: Option<Arc<dyn Fn(String) + Send + Sync>>,
        arm_secs: u32,
    ) -> Self {
        let (tx, rx) = mpsc::channel::<ChunkJob>();
        let accumulated = Arc::new(Mutex::new(String::new()));
        let worker = spawn_worker(app.clone(), rx, accumulated.clone(), on_chunk);
        Self {
            tx,
            accumulated,
            worker: Some(worker),
            arm_secs,
        }
    }

    pub fn sender(&self) -> Sender<ChunkJob> {
        self.tx.clone()
    }

    pub fn arm_secs(&self) -> u32 {
        self.arm_secs
    }

    pub fn submit(&self, job: ChunkJob) {
        if let Err(e) = self.tx.send(job) {
            error!("Chunk pipeline: failed to submit job: {}", e);
        }
    }

    pub fn finalize(mut self) -> String {
        self.submit(ChunkJob::Finalize);
        if let Some(handle) = self.worker.take() {
            if handle.join().is_err() {
                error!("Chunk pipeline: worker thread panicked, returning text accumulated so far");
            }
        }
        self.accumulated
            .lock()
            .map(|guard| guard.clone())
            .unwrap_or_default()
    }
}

fn spawn_worker(
    app: AppHandle,
    rx: Receiver<ChunkJob>,
    accumulated: Arc<Mutex<String>>,
    on_chunk: Option<Arc<dyn Fn(String) + Send + Sync>>,
) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let mut expected_seq: u64 = 0;
        while let Ok(job) = rx.recv() {
            match job {
                ChunkJob::Audio {
                    seq,
                    samples,
                    sample_rate,
                    overlap_prefix,
                } => {
                    debug_assert_eq!(seq, expected_seq, "chunk seq must be monotonic");
                    expected_seq = expected_seq.saturating_add(1);
                    let chunk_secs = samples.len() as f32 / sample_rate.max(1) as f32;

                    let chunk_text = match process_chunk(&app, samples, sample_rate) {
                        ChunkOutcome::Text(text) => text,
                        ChunkOutcome::Empty => String::new(),
                        ChunkOutcome::Failed => {
                            let _ = app.emit("transcription-chunk-error", ());
                            String::new()
                        }
                    };

                    debug!(
                        "Standard chunking: chunk {} transcribed ({:.1}s audio, {} chars)",
                        seq,
                        chunk_secs,
                        chunk_text.len()
                    );

                    let delta = merge_chunk(&accumulated, &chunk_text, overlap_prefix);
                    if !delta.trim().is_empty() {
                        if let Some(ref cb) = on_chunk {
                            cb(delta);
                        }
                    }
                }
                ChunkJob::Finalize => break,
            }
        }
    })
}

fn merge_chunk(
    accumulated: &Arc<Mutex<String>>,
    chunk_text: &str,
    overlap_prefix: usize,
) -> String {
    let Ok(mut acc) = accumulated.lock() else {
        error!("Chunk pipeline: accumulator mutex poisoned, dropping chunk");
        return String::new();
    };

    let deduped = if overlap_prefix > 0 {
        remove_overlap_duplication(&acc, chunk_text)
    } else {
        chunk_text.trim().to_string()
    };

    if deduped.is_empty() {
        return String::new();
    }

    let delta = if acc.is_empty() {
        acc.push_str(&deduped);
        deduped
    } else {
        let with_space = format!(" {}", deduped);
        acc.push_str(&with_space);
        with_space
    };
    delta
}

fn remove_overlap_duplication(cumulated: &str, new_chunk: &str) -> String {
    let new_words: Vec<&str> = new_chunk.split_whitespace().collect();
    if new_words.is_empty() {
        return String::new();
    }

    let acc_words: Vec<&str> = cumulated.split_whitespace().collect();
    let max_overlap = MERGE_WINDOW_WORDS.min(new_words.len()).min(acc_words.len());

    let mut matched = 0;
    for len in (1..=max_overlap).rev() {
        let acc_tail = &acc_words[acc_words.len() - len..];
        let new_head = &new_words[..len];
        let tails_match = acc_tail
            .iter()
            .zip(new_head.iter())
            .all(|(a, b)| normalize_text(a) == normalize_text(b));
        if tails_match {
            matched = len;
            break;
        }
    }

    new_words[matched..].join(" ")
}

/// Accumulates the current chunk's native-rate mono samples and cuts it into the
/// FIFO: a detected silence past CHUNK_SILENCE_ARM_SECS, or a forced cut at
/// CHUNK_FORCE_CUT_SECS. A forced cut keeps ~1s of overlap as the next chunk's
/// head so a word straddling the cut can be deduplicated downstream.
pub(super) struct Chunker {
    tx: Sender<ChunkJob>,
    sample_rate: u32,
    arm_samples: usize,
    force_samples: usize,
    overlap_samples: usize,
    silence_cut_samples: usize,
    silence_run: usize,
    last_tick_len: usize,
    seq: u64,
    samples: Vec<f32>,
    overlap_prefix: usize,
    vad: LiveTextVad,
}

impl Chunker {
    pub(super) fn new(
        tx: Sender<ChunkJob>,
        sample_rate: u32,
        silence_ms: u64,
        arm_secs: u32,
    ) -> Self {
        let sr = sample_rate as usize;
        Self {
            tx,
            sample_rate,
            arm_samples: arm_secs as usize * sr,
            force_samples: CHUNK_FORCE_CUT_SECS as usize * sr,
            overlap_samples: (CHUNK_FORCED_OVERLAP_SECS * sample_rate as f32) as usize,
            silence_cut_samples: (silence_ms as usize * sr / 1000).max(1),
            silence_run: 0,
            last_tick_len: 0,
            seq: 0,
            samples: Vec::new(),
            overlap_prefix: 0,
            vad: LiveTextVad::new(),
        }
    }

    pub(super) fn push_samples(&mut self, mono: &[f32]) {
        self.samples.extend_from_slice(mono);
    }

    pub(super) fn on_throttle_tick(&mut self, rms: f32) {
        if self.samples.len() >= self.force_samples {
            self.cut_forced();
            return;
        }

        if self.samples.len() < self.arm_samples {
            self.last_tick_len = self.samples.len();
            return;
        }

        let delta = self.samples.len().saturating_sub(self.last_tick_len);
        self.last_tick_len = self.samples.len();

        match self.vad.update(rms) {
            LiveTextSilence::Silent => {
                self.silence_run += delta;
                if self.silence_run >= self.silence_cut_samples {
                    self.cut_on_silence();
                }
            }
            LiveTextSilence::Active => self.silence_run = 0,
            LiveTextSilence::NotStarted => {}
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

    pub(super) fn flush_remaining(mut self) {
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
        self.vad = LiveTextVad::new();
        self.silence_run = 0;
        self.last_tick_len = self.samples.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::helpers::rms;

    const SR: u32 = 16000;

    fn dedup(acc: &str, chunk: &str) -> String {
        remove_overlap_duplication(acc, chunk)
    }

    fn drive_chunker(samples: &[f32], silence_ms: u64) -> usize {
        let (tx, rx) = mpsc::channel::<ChunkJob>();
        let mut chunker = Chunker::new(tx, SR, silence_ms, CHUNK_SILENCE_ARM_SECS);
        let window = (SR as usize * 33 / 1000).max(1);
        for win in samples.chunks(window) {
            chunker.push_samples(win);
            chunker.on_throttle_tick(rms(win));
        }
        chunker.flush_remaining();
        rx.try_iter()
            .filter(|job| matches!(job, ChunkJob::Audio { .. }))
            .count()
    }

    fn speech(secs: f32) -> Vec<f32> {
        vec![0.1; (SR as f32 * secs) as usize]
    }

    fn silence(secs: f32) -> Vec<f32> {
        vec![0.0; (SR as f32 * secs) as usize]
    }

    #[test]
    fn chunker_silence_cuts_once_armed() {
        let mut samples = speech(16.0);
        samples.extend(silence(2.0));
        samples.extend(speech(5.0));
        assert_eq!(drive_chunker(&samples, 500), 2);
    }

    #[test]
    fn chunker_no_silence_cut_below_arm() {
        let mut samples = speech(5.0);
        samples.extend(silence(2.0));
        assert_eq!(drive_chunker(&samples, 500), 1);
    }

    #[test]
    fn removes_single_word_seam() {
        assert_eq!(dedup("bonjour le monde", "monde entier ici"), "entier ici");
    }

    #[test]
    fn removes_longest_multi_word_seam() {
        assert_eq!(
            dedup("je vais au marché", "au marché acheter du pain"),
            "acheter du pain"
        );
    }

    #[test]
    fn seam_is_case_and_accent_insensitive() {
        assert_eq!(dedup("je vais au Marché", "marche acheter"), "acheter");
    }

    #[test]
    fn no_overlap_keeps_chunk_intact() {
        assert_eq!(dedup("bonjour le monde", "autre chose"), "autre chose");
    }

    #[test]
    fn dedups_exactly_at_window_size() {
        let acc = "zero un deux trois quatre cinq six";
        let chunk = "un deux trois quatre cinq six sept";
        assert_eq!(dedup(acc, chunk), "sept");
    }

    #[test]
    fn overlap_beyond_window_is_left_intact() {
        let acc = "un deux trois quatre cinq six sept";
        let chunk = "un deux trois quatre cinq six sept huit";
        assert_eq!(dedup(acc, chunk), chunk);
    }

    #[test]
    fn empty_chunk_returns_empty() {
        assert_eq!(dedup("bonjour le monde", ""), "");
    }

    #[test]
    fn empty_accumulator_keeps_chunk_intact() {
        assert_eq!(dedup("", "bonjour le monde"), "bonjour le monde");
    }

    #[test]
    fn full_chunk_is_overlap_returns_empty() {
        assert_eq!(dedup("je vais au marché", "au marché"), "");
    }

    #[test]
    fn merge_silence_cut_joins_with_space() {
        let acc = Arc::new(Mutex::new(String::from("bonjour")));
        let delta = merge_chunk(&acc, "le monde", 0);
        assert_eq!(delta, " le monde");
        assert_eq!(*acc.lock().unwrap(), "bonjour le monde");
    }

    #[test]
    fn merge_first_chunk_has_no_leading_space() {
        let acc = Arc::new(Mutex::new(String::new()));
        let delta = merge_chunk(&acc, "bonjour", 0);
        assert_eq!(delta, "bonjour");
        assert_eq!(*acc.lock().unwrap(), "bonjour");
    }

    #[test]
    fn merge_forced_cut_dedups_seam() {
        let acc = Arc::new(Mutex::new(String::from("je vais au marché")));
        let delta = merge_chunk(&acc, "au marché acheter du pain", 16000);
        assert_eq!(delta, " acheter du pain");
        assert_eq!(*acc.lock().unwrap(), "je vais au marché acheter du pain");
    }

    #[test]
    fn merge_empty_chunk_leaves_accumulator_unchanged() {
        let acc = Arc::new(Mutex::new(String::from("bonjour")));
        let delta = merge_chunk(&acc, "", 0);
        assert_eq!(delta, "");
        assert_eq!(*acc.lock().unwrap(), "bonjour");
    }
}
