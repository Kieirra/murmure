use crate::audio::pipeline::{process_chunk, ChunkOutcome};
use crate::wake_word::wake_word::normalize_text;
use log::{debug, error};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use tauri::{AppHandle, Emitter};

const MERGE_WINDOW_WORDS: usize = 6;

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
}

impl ChunkPipeline {
    pub fn start(app: &AppHandle) -> Self {
        let (tx, rx) = mpsc::channel::<ChunkJob>();
        let accumulated = Arc::new(Mutex::new(String::new()));
        let worker = spawn_worker(app.clone(), rx, accumulated.clone());
        Self {
            tx,
            accumulated,
            worker: Some(worker),
        }
    }

    pub fn sender(&self) -> Sender<ChunkJob> {
        self.tx.clone()
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

                    merge_chunk(&accumulated, &chunk_text, overlap_prefix);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn dedup(acc: &str, chunk: &str) -> String {
        remove_overlap_duplication(acc, chunk)
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
