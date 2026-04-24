use parking_lot::Mutex;
use std::{collections::HashMap, sync::Arc};

pub type EncodedDict = Vec<(String, String)>;

#[derive(Clone)]
pub struct Dictionary {
    /// Direct reads are fine (see `fix_transcription_with_dictionary`);
    /// **never** mutate directly — always go through `Dictionary::set()`
    /// so `encoded_cache` stays in sync with the words.
    pub words: Arc<Mutex<HashMap<String, Vec<String>>>>,
    /// Populated lazily in `fix_transcription_with_dictionary`. Cleared
    /// by `Dictionary::set()`. Do not mutate outside those two paths.
    pub encoded_cache: Arc<Mutex<Option<EncodedDict>>>,
}

impl Dictionary {
    pub fn new(dictionary: HashMap<String, Vec<String>>) -> Self {
        Self {
            words: Arc::new(Mutex::new(dictionary)),
            encoded_cache: Arc::new(Mutex::new(None)),
        }
    }
    pub fn set(&self, dictionary: HashMap<String, Vec<String>>) {
        *self.words.lock() = dictionary;
        *self.encoded_cache.lock() = None;
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DictionaryError {
    #[error("Invalid word format: {0}. Words must contain only letters (a-z, A-Z)")]
    InvalidWordFormat(String),
    #[error("Dictionary import must contain at least one valid word")]
    EmptyDictionary,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn words(pairs: &[(&str, &[&str])]) -> HashMap<String, Vec<String>> {
        pairs
            .iter()
            .map(|(k, langs)| {
                (
                    k.to_string(),
                    langs.iter().map(|s| s.to_string()).collect(),
                )
            })
            .collect()
    }

    #[test]
    fn new_starts_with_empty_cache() {
        let dict = Dictionary::new(HashMap::new());
        assert!(dict.encoded_cache.lock().is_none());
    }

    #[test]
    fn new_stores_words_unchanged() {
        let initial = words(&[("hello", &["english"]), ("bonjour", &["french"])]);
        let dict = Dictionary::new(initial.clone());
        assert_eq!(*dict.words.lock(), initial);
    }

    #[test]
    fn set_replaces_words() {
        let dict = Dictionary::new(words(&[("old", &["english"])]));
        let new_words = words(&[("new", &["french"])]);
        dict.set(new_words.clone());
        assert_eq!(*dict.words.lock(), new_words);
    }

    #[test]
    fn set_invalidates_populated_cache() {
        let dict = Dictionary::new(words(&[("hello", &["english"])]));
        *dict.encoded_cache.lock() =
            Some(vec![("hello".to_string(), "HL".to_string())]);
        assert!(dict.encoded_cache.lock().is_some());

        dict.set(words(&[("world", &["english"])]));
        assert!(dict.encoded_cache.lock().is_none());
    }

    #[test]
    fn set_to_same_words_still_invalidates_cache() {
        let initial = words(&[("hello", &["english"])]);
        let dict = Dictionary::new(initial.clone());
        *dict.encoded_cache.lock() =
            Some(vec![("hello".to_string(), "HL".to_string())]);

        dict.set(initial);
        assert!(dict.encoded_cache.lock().is_none());
    }

    #[test]
    fn clone_shares_words_storage() {
        let dict = Dictionary::new(HashMap::new());
        let dict2 = dict.clone();

        let new_words = words(&[("shared", &["english"])]);
        dict.set(new_words.clone());

        assert_eq!(*dict2.words.lock(), new_words);
    }

    #[test]
    fn clone_shares_cache_storage() {
        let dict = Dictionary::new(HashMap::new());
        let dict2 = dict.clone();

        let cached = vec![("a".to_string(), "A".to_string())];
        *dict.encoded_cache.lock() = Some(cached.clone());

        assert_eq!(*dict2.encoded_cache.lock(), Some(cached));
    }

    #[test]
    fn set_via_one_handle_invalidates_cache_on_clone() {
        let dict = Dictionary::new(words(&[("hello", &["english"])]));
        let dict2 = dict.clone();

        *dict.encoded_cache.lock() =
            Some(vec![("hello".to_string(), "HL".to_string())]);
        assert!(dict2.encoded_cache.lock().is_some());

        dict.set(words(&[("world", &["english"])]));
        assert!(dict2.encoded_cache.lock().is_none());
    }
}

#[cfg(test)]
mod perf_bench {
    use super::*;
    use std::hint::black_box;
    use std::time::Instant;

    fn make_dictionary(n_words: usize) -> HashMap<String, Vec<String>> {
        (0..n_words)
            .map(|i| {
                (
                    format!("customword{}", i),
                    vec!["english".to_string(), "french".to_string()],
                )
            })
            .collect()
    }

    // Sweeps across realistic dictionary sizes: 1 / 10 / 100 / 1 000 / 5 000
    // / 10 000 words.
    //
    // Baseline reflects production: `dictionary::store::load` iterates
    // `store.entries()` from `tauri-plugin-store` (which keeps parsed
    // `Value`s in memory) and calls `from_value<Vec<String>>` per entry.
    // The state path (`current()`) is an O(1) Arc refcount bump — the
    // HashMap is shared, not deep-cloned, regardless of word count.
    //
    // Hardening: `black_box` prevents DCE, 3 trials per size, steady-state
    // (warm allocator). Iterations scale inverse to word count so the
    // largest sizes still finish in seconds. Run with `--test-threads=1`
    // for noise-free output:
    //   cargo test --release --lib -- --ignored --nocapture --test-threads=1 perf_dictionary_disk_vs_state
    #[test]
    #[ignore]
    fn perf_dictionary_disk_vs_state() {
        const WARMUP: u32 = 2_000;
        const TRIALS: u32 = 3;

        println!("\n=== Dictionary load: store.entries+from_value  vs  state Arc-clone ===");
        println!(
            "{:>7} {:>10} {:>18} {:>18} {:>10} {:>10}",
            "words", "json_B", "store+parse ns", "Arc clone ns", "speedup", "saved_ns"
        );

        for n_words in [1, 10, 100, 1000, 5000, 10000] {
            let iters: u32 = match n_words {
                n if n >= 5000 => 500,
                n if n >= 1000 => 2_000,
                _ => 50_000,
            };

            let words_map = make_dictionary(n_words);
            let json = serde_json::to_string_pretty(&words_map).unwrap();
            let stored_entries: HashMap<String, serde_json::Value> = words_map
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::to_value(v).unwrap()))
                .collect();

            for _ in 0..WARMUP {
                let mut parsed = HashMap::new();
                for (k, v) in stored_entries.iter() {
                    let langs: Vec<String> =
                        serde_json::from_value(v.clone()).unwrap_or_default();
                    parsed.insert(k.clone(), langs);
                }
                black_box(parsed);
            }

            let dict = Dictionary::new(words_map);

            for trial in 1..=TRIALS {
                let start = Instant::now();
                for _ in 0..iters {
                    let mut parsed: HashMap<String, Vec<String>> = HashMap::new();
                    for (k, v) in black_box(&stored_entries).iter() {
                        let langs: Vec<String> =
                            serde_json::from_value(v.clone()).unwrap_or_default();
                        parsed.insert(k.clone(), langs);
                    }
                    // Wrap in Dictionary::new to match the full production
                    // path (fallback branch of `store::current()`).
                    let wrapped = Dictionary::new(parsed);
                    black_box(wrapped);
                }
                let store_path = start.elapsed();

                let start = Instant::now();
                for _ in 0..iters {
                    let cloned = dict.clone();
                    black_box(cloned);
                }
                let state_time = start.elapsed();

                let store_ns = store_path.as_nanos() / iters as u128;
                let state_ns = state_time.as_nanos() / iters as u128;
                let speedup = store_ns as f64 / state_ns.max(1) as f64;

                println!(
                    "{:>4}#{} {:>10} {:>18} {:>18} {:>9.1}× {:>10}",
                    n_words,
                    trial,
                    json.len(),
                    store_ns,
                    state_ns,
                    speedup,
                    store_ns.saturating_sub(state_ns)
                );
            }
        }
    }
}
