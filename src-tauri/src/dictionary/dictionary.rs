use crate::engine::helpers::fold_accents;
use crate::engine::ParakeetEngine;
use std::collections::HashMap;

/// Resync the phrase-boosting words from the user dictionary onto the engine.
/// Must run before transcription so the boost tree reflects the current vocab.
pub fn sync_boost_words(engine: &mut ParakeetEngine, dictionary: &HashMap<String, Vec<String>>) {
    let words: Vec<String> = dictionary.keys().cloned().collect();
    engine.set_boost_words(&words);
}

/// Words shorter than this (normalized, in chars) are never fuzzy-corrected,
/// so short common words are not pulled toward short dictionary keys.
const POSTCORR_MIN_LEN: usize = 5;
/// Words of this length and above (normalized, in chars) may absorb 2 edits;
/// shorter ones only 1. Two edits on a 5-7 char word reach too many common
/// words, so short keys would capture unrelated vocabulary.
const POSTCORR_LONG_LEN: usize = 8;

fn max_distance_for(len: usize) -> usize {
    if len >= POSTCORR_LONG_LEN {
        2
    } else {
        1
    }
}

/// Words whose model confidence (min softmax prob over the word's tokens) is
/// at or above this are never fuzzy-corrected: the model was sure of what it
/// heard, so a near-miss dictionary key must not capture it. Calibrated on the
/// eval/ corpus with the shipped 1.0.0 encoder: real words clearly heard
/// score >= 0.853 (maçon, commis, repos), hallucinated spellings of dictionary
/// terms <= 0.570 (Cintocinon, Wayand, Oliama, Excalidro). Set above 1.0 to
/// disable the gate.
pub const POSTCORR_CONF_THRESHOLD: f32 = 0.75;

/// Above this many dictionary words the fuzzy step is disabled entirely
/// (exact-match casing restore always stays on, it is a hash lookup). Every
/// key is a potential false-positive attractor and the Levenshtein scan is
/// O(text words × keys), so large imported vocabularies keep working without
/// degrading common words or streaming latency. Mirrors `degressive_alpha`
/// on the boosting side.
pub const POSTCORR_MAX_DICT_WORDS: usize = 100;

/// Build the lookup the gate uses: normalized word -> confidence. Same word
/// appearing several times keeps the max (protecting a confident occurrence
/// beats correcting a mumbled duplicate).
pub fn confidence_map(word_confidences: &[(String, f32)]) -> HashMap<String, f32> {
    let mut map: HashMap<String, f32> = HashMap::new();
    for (word, conf) in word_confidences {
        let key: String = normalize_word(word)
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();
        if key.is_empty() {
            continue;
        }
        map.entry(key)
            .and_modify(|c| *c = c.max(*conf))
            .or_insert(*conf);
    }
    map
}

/// Restore the dictionary's canonical spelling on whole-word matches, with a
/// strict fuzzy fallback. Exact normalized matches (distance 0) just rewrite
/// casing/accents to the stored spelling; near matches under the strict
/// thresholds are corrected to the closest dictionary key (mur acoustique /
/// segmentation cases the greedy boost cannot fix). Ambiguous near matches
/// (a tie on the minimal distance between two keys) are left untouched.
/// Ungated variant, kept for the eval harness and unit tests; production
/// always goes through `restore_dictionary_casing_gated`.
#[cfg(test)]
pub fn restore_dictionary_casing(text: &str, dictionary: &HashMap<String, Vec<String>>) -> String {
    restore_dictionary_casing_gated(text, dictionary, None)
}

/// Same as `restore_dictionary_casing`, with the fuzzy step gated by the model
/// confidence of each word (see `POSTCORR_CONF_THRESHOLD`). `None`, or a word
/// missing from the map, behaves like the ungated version.
pub fn restore_dictionary_casing_gated(
    text: &str,
    dictionary: &HashMap<String, Vec<String>>,
    confidences: Option<&HashMap<String, f32>>,
) -> String {
    if dictionary.is_empty() {
        return text.to_string();
    }

    let fuzzy_enabled = dictionary.len() <= POSTCORR_MAX_DICT_WORDS;
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
            match best_dictionary_match(word, &normalized, confidences, fuzzy_enabled) {
                Some(canonical) => format!("{}{}", canonical, trailing),
                None => segment.to_string(),
            }
        })
        .collect()
}

/// Pick the dictionary key closest to `word`. Returns an exact normalized
/// match outright, else the single strictly-closest key under the thresholds,
/// or `None` when nothing qualifies or the best distance is a tie.
fn best_dictionary_match<'a>(
    word: &str,
    normalized: &'a HashMap<String, &'a String>,
    confidences: Option<&HashMap<String, f32>>,
    fuzzy_enabled: bool,
) -> Option<&'a String> {
    let target = normalize_word(word);
    if target.is_empty() {
        return None;
    }

    if let Some(canonical) = normalized.get(&target) {
        return Some(canonical);
    }

    if !fuzzy_enabled {
        return None;
    }

    let target_len = target.chars().count();
    if target_len < POSTCORR_MIN_LEN {
        return None;
    }

    // Confidence gate: the model was sure of this word, leave it alone.
    if let Some(map) = confidences {
        if let Some(&conf) = map.get(&target) {
            if conf >= POSTCORR_CONF_THRESHOLD {
                return None;
            }
        }
    }

    let mut best: Option<(usize, &String)> = None;
    let mut tied = false;
    for (key, canonical) in normalized {
        let dist = levenshtein(&target, key);
        match best {
            Some((best_dist, _)) if dist > best_dist => {}
            Some((best_dist, _)) if dist == best_dist => tied = true,
            _ => {
                best = Some((dist, canonical));
                tied = false;
            }
        }
    }

    match best {
        Some((dist, canonical)) if !tied && dist <= max_distance_for(target_len) => {
            log::debug!(
                "dictionary post-correction: {} -> {} (d={})",
                word,
                canonical,
                dist
            );
            Some(canonical)
        }
        _ => None,
    }
}

/// Diagnostic for the eval harness: the fuzzy corrections the thresholds
/// would allow on `text`, gate ignored, with each word's model confidence,
/// as "word→key d=N p=0.973" entries.
#[cfg(test)]
pub fn fuzzy_correction_candidates(
    text: &str,
    dictionary: &HashMap<String, Vec<String>>,
    confidences: &HashMap<String, f32>,
) -> Vec<String> {
    let normalized: HashMap<String, &String> = dictionary
        .keys()
        .map(|key| (normalize_word(key), key))
        .collect();
    let mut out = Vec::new();
    for word in text.split(|c: char| !c.is_alphanumeric()) {
        let target = normalize_word(word);
        if target.is_empty() || normalized.contains_key(&target) {
            continue;
        }
        if let Some(canonical) = best_dictionary_match(word, &normalized, None, true) {
            let dist = levenshtein(&target, &normalize_word(canonical));
            let conf = confidences
                .get(&target)
                .map(|c| format!("{:.3}", c))
                .unwrap_or_else(|| "?".into());
            out.push(format!("{}→{} d={} p={}", word, canonical, dist, conf));
        }
    }
    out
}

/// Classic two-row Levenshtein edit distance over Unicode scalar values.
fn levenshtein(a: &str, b: &str) -> usize {
    let b_chars: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b_chars.len()).collect();
    let mut curr = vec![0; b_chars.len() + 1];

    for (i, ca) in a.chars().enumerate() {
        curr[0] = i + 1;
        for (j, &cb) in b_chars.iter().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            curr[j + 1] = (prev[j + 1] + 1).min(curr[j] + 1).min(prev[j] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    prev[b_chars.len()]
}

fn normalize_word(word: &str) -> String {
    fold_accents(word).to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::{
        restore_dictionary_casing, restore_dictionary_casing_gated, POSTCORR_CONF_THRESHOLD,
        POSTCORR_MAX_DICT_WORDS,
    };
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

    #[test]
    fn fuzzy_disabled_above_dictionary_cap_but_exact_restore_stays() {
        let mut words: Vec<String> = (0..POSTCORR_MAX_DICT_WORDS)
            .map(|i| format!("motdico{:03}", i))
            .collect();
        words.push("Syntocinon".to_string());
        let dictionary: HashMap<String, Vec<String>> =
            words.into_iter().map(|w| (w, Vec::new())).collect();
        // 101 mots: le fuzzy est coupé, la faute reste...
        let out = restore_dictionary_casing("dose de sintocinon", &dictionary);
        assert_eq!(out, "dose de sintocinon");
        // ...mais la restauration de casse sur match exact fonctionne toujours.
        let out = restore_dictionary_casing("dose de syntocinon", &dictionary);
        assert_eq!(out, "dose de Syntocinon");
    }

    #[test]
    fn confidence_gate_blocks_fuzzy_on_confident_word() {
        let dictionary = dict(&["MacOS"]);
        let conf: HashMap<String, f32> = [("macon".to_string(), POSTCORR_CONF_THRESHOLD)].into();
        let out = restore_dictionary_casing_gated("le maçon construit", &dictionary, Some(&conf));
        assert_eq!(out, "le maçon construit");
    }

    #[test]
    fn confidence_gate_allows_fuzzy_on_unsure_word() {
        let dictionary = dict(&["célécoxib"]);
        let conf: HashMap<String, f32> =
            [("selecoxyb".to_string(), POSTCORR_CONF_THRESHOLD / 2.0)].into();
        let out = restore_dictionary_casing_gated("dose de Sélecoxyb", &dictionary, Some(&conf));
        assert_eq!(out, "dose de célécoxib");
    }

    #[test]
    fn confidence_gate_does_not_touch_exact_casing_restore() {
        let dictionary = dict(&["Syntocinon"]);
        let conf: HashMap<String, f32> = [("syntocinon".to_string(), 1.0)].into();
        let out = restore_dictionary_casing_gated("dose de syntocinon", &dictionary, Some(&conf));
        assert_eq!(out, "dose de Syntocinon");
    }

    #[test]
    fn confidence_map_normalizes_and_keeps_max() {
        let words = vec![
            ("Maçon,".to_string(), 0.3),
            ("maçon".to_string(), 0.9),
            ("mur".to_string(), 0.5),
        ];
        let map = super::confidence_map(&words);
        assert!((map["macon"] - 0.9).abs() < 1e-6);
        assert!((map["mur"] - 0.5).abs() < 1e-6);
    }

    #[test]
    fn fuzzy_corrects_misspelled_dictionary_word() {
        let dictionary = dict(&["célécoxib"]);
        let out = restore_dictionary_casing("dose de Sélecoxyb prescrite", &dictionary);
        assert_eq!(out, "dose de célécoxib prescrite");
    }

    #[test]
    fn fuzzy_picks_closest_key_not_far_one() {
        let dictionary = dict(&["Selenium", "Celeri"]);
        assert_eq!(restore_dictionary_casing("Seleri", &dictionary), "Celeri");
        assert_eq!(
            restore_dictionary_casing("Selenium", &dictionary),
            "Selenium"
        );
    }

    #[test]
    fn fuzzy_leaves_unrelated_common_word_intact() {
        let dictionary = dict(&["célécoxib"]);
        let out = restore_dictionary_casing("bonjour", &dictionary);
        assert_eq!(out, "bonjour");
    }

    #[test]
    fn fuzzy_never_touches_short_words() {
        let dictionary = dict(&["des", "les"]);
        let out = restore_dictionary_casing("les amis", &dictionary);
        assert_eq!(out, "les amis");
    }

    #[test]
    fn fuzzy_requires_long_word_for_two_edits() {
        // "parcil" is at distance 2 from "persil"; below 8 chars only one
        // edit is allowed, so the word must stay untouched.
        let dictionary = dict(&["persil"]);
        let out = restore_dictionary_casing("parcil", &dictionary);
        assert_eq!(out, "parcil");
    }

    #[test]
    fn fuzzy_skips_on_distance_tie() {
        // "barres" is at distance 1 from both "barrer" and "barret".
        let dictionary = dict(&["barrer", "barret"]);
        let out = restore_dictionary_casing("barres", &dictionary);
        assert_eq!(out, "barres");
    }

    #[test]
    fn fuzzy_preserves_surrounding_punctuation() {
        let dictionary = dict(&["célécoxib"]);
        let out = restore_dictionary_casing("(Sélecoxyb).", &dictionary);
        assert_eq!(out, "(célécoxib).");
    }
}
