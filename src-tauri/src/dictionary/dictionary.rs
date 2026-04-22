use crate::dictionary::types::Dictionary;
use log::debug;
use once_cell::sync::OnceCell;
use rphonetic::{BeiderMorseBuilder, ConfigFiles, LanguageSet};
use std::path::PathBuf;
use tauri::AppHandle;

/// Cached resolved path to the bundled `cc-rules/` directory.
/// Populated on the first successful `get_cc_rules_path` call; subsequent
/// calls return the cached value without re-walking the Tauri resource paths.
/// Failures do not poison the cache — a transient resource-resolution error
/// leaves the cell empty and the next call retries.
static CC_RULES_PATH: OnceCell<PathBuf> = OnceCell::new();

static CONFIG_FILES: OnceCell<ConfigFiles> = OnceCell::new();

fn get_or_init_config_files(cc_rules_path: &PathBuf) -> &'static ConfigFiles {
    CONFIG_FILES.get_or_init(|| {
        ConfigFiles::new(cc_rules_path).expect("Failed to load BeiderMorse config files")
    })
}

/**
 * Use phonetic algorithm to fix the transcription
 */
pub fn fix_transcription_with_dictionary(
    transcription: String,
    dictionary: &Dictionary,
    cc_rules_path: &PathBuf,
) -> String {
    let words_snapshot = dictionary.words.lock().unwrap().clone();
    if words_snapshot.is_empty() {
        return transcription;
    }

    let config_files = get_or_init_config_files(cc_rules_path);
    let builder = BeiderMorseBuilder::new(config_files);
    let beider_morse = builder.build();

    // TODO: Make user able to choose the languages for each word
    let langs = LanguageSet::from(vec!["french", "english"]);

    let encoded_dict = {
        let mut cache = dictionary.encoded_cache.lock().unwrap();
        match cache.as_ref() {
            Some(cached) => cached.clone(),
            None => {
                let mut encoded = Vec::with_capacity(words_snapshot.len());
                for word in words_snapshot.keys() {
                    let code = beider_morse.encode_with_languages(word, &langs);
                    encoded.push((word.clone(), code));
                }
                *cache = Some(encoded.clone());
                encoded
            }
        }
    };

    let mut corrected_transcription = transcription.clone();
    let words: Vec<&str> = transcription.split_whitespace().collect();

    for word in words {
        let candidate = beider_morse.encode_with_languages(word, &langs);
        let candidate_codes: Vec<&str> = candidate.split('|').collect();
        for (dict_word, dict_code) in &encoded_dict {
            let dict_codes: Vec<&str> = dict_code.split('|').collect();
            if dict_codes.iter().any(|dc| candidate_codes.contains(dc)) {
                corrected_transcription = corrected_transcription.replace(word, dict_word);
            }
        }
    }

    corrected_transcription
}

// Downloaded from https://github.com/apache/commons-codec/tree/rel/commons-codec-1.15/src/main/resources/org/apache/commons/codec/language/bm
pub fn get_cc_rules_path(app_handle: &AppHandle) -> anyhow::Result<PathBuf> {
    CC_RULES_PATH
        .get_or_try_init(|| -> anyhow::Result<PathBuf> {
            let path = crate::utils::resources::resolve_resource_path(app_handle, "cc-rules/")
                .ok_or_else(|| anyhow::anyhow!("Bundled cc_rules not found in any known location"))?;
            debug!("CC rules found at: {}", path.display());
            Ok(path)
        })
        .cloned()
}

#[cfg(test)]
mod tests {
    use once_cell::sync::OnceCell;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Documents the contract `get_cc_rules_path` relies on: the init closure
    /// runs at most once per process, subsequent calls return the cached path
    /// without re-running the Tauri resource walk. A failing init leaves the
    /// cell empty so the next call retries.
    #[test]
    fn once_cell_init_runs_exactly_once_on_success() {
        let cache: OnceCell<PathBuf> = OnceCell::new();
        let call_count = AtomicUsize::new(0);

        let first = cache
            .get_or_try_init(|| -> Result<_, ()> {
                call_count.fetch_add(1, Ordering::SeqCst);
                Ok(PathBuf::from("/tmp/test-cc-rules"))
            })
            .unwrap();

        let second = cache
            .get_or_try_init(|| -> Result<_, ()> {
                call_count.fetch_add(1, Ordering::SeqCst);
                panic!("second call must not re-run the init closure");
            })
            .unwrap();

        assert_eq!(first, second);
        assert_eq!(call_count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn once_cell_init_is_retried_after_failure() {
        let cache: OnceCell<PathBuf> = OnceCell::new();
        let call_count = AtomicUsize::new(0);

        let first = cache.get_or_try_init(|| -> Result<_, &'static str> {
            call_count.fetch_add(1, Ordering::SeqCst);
            Err("transient failure")
        });
        assert!(first.is_err());

        let second = cache
            .get_or_try_init(|| -> Result<_, &'static str> {
                call_count.fetch_add(1, Ordering::SeqCst);
                Ok(PathBuf::from("/tmp/test-cc-rules"))
            })
            .unwrap();

        assert_eq!(second, &PathBuf::from("/tmp/test-cc-rules"));
        assert_eq!(call_count.load(Ordering::SeqCst), 2);
    }
}
