use log::debug;
use once_cell::sync::OnceCell;
use rphonetic::{BeiderMorseBuilder, ConfigFiles, LanguageSet};
use std::collections::HashMap;
use std::path::PathBuf;
use tauri::AppHandle;

/// Cached resolved path to the bundled `cc-rules/` directory.
/// Populated on the first successful `get_cc_rules_path` call; subsequent
/// calls return the cached value without re-walking the Tauri resource paths.
/// Failures do not poison the cache — a transient resource-resolution error
/// leaves the cell empty and the next call retries.
static CC_RULES_PATH: OnceCell<PathBuf> = OnceCell::new();

/**
 * Use phonetic algorithm to fix the transcription
 */
pub fn fix_transcription_with_dictionary(
    transcription: String,
    dictionary: &HashMap<String, Vec<String>>,
    cc_rules_path: &PathBuf,
) -> String {
    if dictionary.is_empty() {
        return transcription;
    }

    let config_files = ConfigFiles::new(cc_rules_path).unwrap();
    let builder = BeiderMorseBuilder::new(&config_files);
    let beider_morse = builder.build();

    // TODO: Make user able to choose the languages for each word
    let langs = LanguageSet::from(vec!["french", "english"]);

    // Prepare dictionary words to be encoded phonetically
    let mut encoded_dict = Vec::new();
    for word in dictionary.keys() {
        let code = beider_morse.encode_with_languages(word, &langs);
        encoded_dict.push((word, code));
    }

    // Split transcription into words
    let mut corrected_transcription = transcription.clone();
    let words: Vec<&str> = transcription.split_whitespace().collect();

    for word in words {
        let candidate = beider_morse.encode_with_languages(word, &langs);
        let candidate_codes: Vec<&str> = candidate.split('|').collect();
        for (dict_word, dict_code) in &encoded_dict {
            let dict_codes: Vec<&str> = dict_code.split('|').collect();
            // println!(
            //     "Dict word: {:?}, Dict code: {:?}, Candidate: {:?}",
            //     dict_word, dict_code, candidate
            // );
            if dict_codes.iter().any(|dc| candidate_codes.contains(dc)) {
                corrected_transcription = corrected_transcription.replace(word, dict_word);
            }
        }
    }

    corrected_transcription
}

/// Restore the dictionary's canonical casing on whole-word matches.
/// Phrase boosting and the vocab are lowercase, so a boosted word lands
/// lowercased; this rewrites it to the exact spelling stored in the dictionary.
pub fn restore_dictionary_casing(text: &str, dictionary: &HashMap<String, Vec<String>>) -> String {
    if dictionary.is_empty() {
        return text.to_string();
    }

    text.split_inclusive(|c: char| !c.is_alphanumeric())
        .map(|segment| {
            let boundary_len = segment
                .chars()
                .rev()
                .take_while(|c| !c.is_alphanumeric())
                .map(char::len_utf8)
                .sum::<usize>();
            let (word, trailing) = segment.split_at(segment.len() - boundary_len);
            match dictionary.keys().find(|key| key.eq_ignore_ascii_case(word)) {
                Some(canonical) => format!("{}{}", canonical, trailing),
                None => segment.to_string(),
            }
        })
        .collect()
}

// Downloaded from https://github.com/apache/commons-codec/tree/rel/commons-codec-1.15/src/main/resources/org/apache/commons/codec/language/bm
pub fn get_cc_rules_path(app_handle: &AppHandle) -> anyhow::Result<PathBuf> {
    CC_RULES_PATH
        .get_or_try_init(|| -> anyhow::Result<PathBuf> {
            let path = crate::utils::resources::resolve_resource_path(app_handle, "cc-rules/")
                .ok_or_else(|| {
                    anyhow::anyhow!("Bundled cc_rules not found in any known location")
                })?;
            debug!("CC rules found at: {}", path.display());
            Ok(path)
        })
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::restore_dictionary_casing;
    use once_cell::sync::OnceCell;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn dict(words: &[&str]) -> HashMap<String, Vec<String>> {
        words
            .iter()
            .map(|w| (w.to_string(), vec!["french".to_string()]))
            .collect()
    }

    #[test]
    fn restore_casing_rewrites_whole_word_match() {
        let dictionary = dict(&["Syntocinon"]);
        let out = restore_dictionary_casing("on injecte syntocinon maintenant", &dictionary);
        assert_eq!(out, "on injecte Syntocinon maintenant");
    }

    #[test]
    fn restore_casing_preserves_punctuation() {
        let dictionary = dict(&["Syntocinon"]);
        let out = restore_dictionary_casing("dose: syntocinon.", &dictionary);
        assert_eq!(out, "dose: Syntocinon.");
    }

    #[test]
    fn restore_casing_ignores_substrings() {
        let dictionary = dict(&["cin"]);
        let out = restore_dictionary_casing("syntocinon", &dictionary);
        assert_eq!(out, "syntocinon");
    }

    #[test]
    fn restore_casing_noop_on_empty_dictionary() {
        let dictionary: HashMap<String, Vec<String>> = HashMap::new();
        let out = restore_dictionary_casing("syntocinon", &dictionary);
        assert_eq!(out, "syntocinon");
    }

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
