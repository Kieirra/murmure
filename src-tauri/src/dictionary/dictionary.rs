use crate::engine::helpers::fold_accents;
use crate::engine::ParakeetEngine;
use std::collections::HashMap;

/// Resync the phrase-boosting words from the user dictionary onto the engine.
/// Must run before transcription so the boost tree reflects the current vocab.
pub fn sync_boost_words(engine: &mut ParakeetEngine, dictionary: &HashMap<String, Vec<String>>) {
    let words: Vec<String> = dictionary.keys().cloned().collect();
    engine.set_boost_words(&words);
}

/// Restore the dictionary's canonical spelling on whole-word matches.
/// Boosting may emit a word with different casing or accents than the user
/// typed; matching normalizes both sides (lowercase + accent fold) so any
/// such variant is rewritten to the exact spelling stored in the dictionary.
pub fn restore_dictionary_casing(text: &str, dictionary: &HashMap<String, Vec<String>>) -> String {
    if dictionary.is_empty() {
        return text.to_string();
    }

    let normalized: HashMap<String, &String> = dictionary
        .keys()
        .map(|key| (normalize_word(key), key))
        .collect();

    text.split_inclusive(|c: char| !c.is_alphanumeric())
        .map(|segment| {
            let boundary_len = segment
                .chars()
                .rev()
                .take_while(|c| !c.is_alphanumeric())
                .map(char::len_utf8)
                .sum::<usize>();
            let (word, trailing) = segment.split_at(segment.len() - boundary_len);
            match normalized.get(&normalize_word(word)) {
                Some(canonical) => format!("{}{}", canonical, trailing),
                None => segment.to_string(),
            }
        })
        .collect()
}

fn normalize_word(word: &str) -> String {
    fold_accents(word).to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::restore_dictionary_casing;
    use std::collections::HashMap;

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

    #[test]
    fn restore_casing_matches_ignoring_accents_and_case() {
        let dictionary = dict(&["célécoxib"]);
        assert_eq!(
            restore_dictionary_casing("dose de celecoxib", &dictionary),
            "dose de célécoxib"
        );
        assert_eq!(
            restore_dictionary_casing("Celecoxib prescrit", &dictionary),
            "célécoxib prescrit"
        );
        assert_eq!(
            restore_dictionary_casing("le célécoxib", &dictionary),
            "le célécoxib"
        );
    }

    #[test]
    fn restore_casing_leaves_unrelated_word_intact() {
        let dictionary = dict(&["célécoxib"]);
        let out = restore_dictionary_casing("aspirine maintenant", &dictionary);
        assert_eq!(out, "aspirine maintenant");
    }
}
