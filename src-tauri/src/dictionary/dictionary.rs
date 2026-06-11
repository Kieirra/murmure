use crate::engine::helpers::fold_accents;
use crate::engine::ParakeetEngine;
use std::collections::HashMap;

/// Resync the phrase-boosting words from the user dictionary onto the engine.
/// Must run before transcription so the boost tree reflects the current vocab.
pub fn sync_boost_words(engine: &mut ParakeetEngine, dictionary: &[String]) {
    engine.set_boost_words(dictionary);
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

/// Words whose model confidence is at or above this are never
/// fuzzy-corrected: the model was sure of what it heard, so a near-miss
/// dictionary key must not capture it. Calibrated on the eval/ corpus
/// against both bundled encoders; set above 1.0 to disable the gate.
pub const POSTCORR_CONF_THRESHOLD: f32 = 0.80;

/// Above this many dictionary words the fuzzy step is disabled entirely:
/// every key is a potential false-positive attractor and the Levenshtein
/// scan grows with the key count. Exact-match casing restore stays on.
pub const POSTCORR_MAX_DICT_WORDS: usize = 100;

/// A word appearing several times keeps its max confidence: protecting a
/// confident occurrence beats correcting a mumbled duplicate. Engine words
/// are split on non-alphanumeric characters so the keys match the lookup
/// segmentation of the restore pass ("l'ordinateur" gates "ordinateur").
pub fn confidence_map(word_confidences: &[(String, f32)]) -> HashMap<String, f32> {
    let mut map: HashMap<String, f32> = HashMap::new();
    for (word, conf) in word_confidences {
        for fragment in word.split(|c: char| !c.is_alphanumeric()) {
            let key = normalize_word(fragment);
            if key.is_empty() {
                continue;
            }
            map.entry(key)
                .and_modify(|c| *c = c.max(*conf))
                .or_insert(*conf);
        }
    }
    map
}

/// Production correction path: casing/fuzzy restore gated by the per-word
/// confidences coming from the engine.
pub fn correct_transcription(
    text: &str,
    dictionary: &[String],
    word_confidences: &[(String, f32)],
) -> String {
    let confidences = confidence_map(word_confidences);
    restore_dictionary_casing_gated(text, dictionary, Some(&confidences))
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
pub fn restore_dictionary_casing(text: &str, dictionary: &[String]) -> String {
    restore_dictionary_casing_gated(text, dictionary, None)
}

/// Same as `restore_dictionary_casing`, with the fuzzy step gated by the model
/// confidence of each word (see `POSTCORR_CONF_THRESHOLD`). `None`, or a word
/// missing from the map, behaves like the ungated version.
pub fn restore_dictionary_casing_gated(
    text: &str,
    dictionary: &[String],
    confidences: Option<&HashMap<String, f32>>,
) -> String {
    if dictionary.is_empty() {
        return text.to_string();
    }

    let fuzzy_enabled = dictionary.len() <= POSTCORR_MAX_DICT_WORDS;
    let normalized: HashMap<String, &String> = dictionary
        .iter()
        .map(|key| (normalize_word(key), key))
        .collect();

    let segments: Vec<(&str, &str)> = text
        .split_inclusive(|c: char| !c.is_alphanumeric())
        .map(|segment| {
            let boundary_len = segment
                .chars()
                .rev()
                .take_while(|c| !c.is_alphanumeric())
                .map(char::len_utf8)
                .sum::<usize>();
            segment.split_at(segment.len() - boundary_len)
        })
        .collect();

    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < segments.len() {
        let (word, trailing) = segments[i];
        if let Some(canonical) = best_dictionary_match(word, &normalized, confidences, fuzzy_enabled)
        {
            out.push_str(canonical);
            out.push_str(trailing);
        } else if let Some(canonical) =
            bigram_match(&segments, i, &normalized, confidences, fuzzy_enabled)
        {
            out.push_str(canonical);
            out.push_str(segments[i + 1].1);
            i += 2;
            continue;
        } else {
            out.push_str(word);
            out.push_str(trailing);
        }
        i += 1;
    }
    out
}

/// Joined words of this length and above (normalized, in chars) may absorb
/// 3 edits in the bigram pass; shorter ones keep the unigram thresholds.
const POSTCORR_BIGRAM_LONG_LEN: usize = 12;

/// Repair a dictionary word the model split in two ("Saint Occinon" for
/// "Syntocinon", "app image" for "AppImage"). Only fires when the first word
/// had no match of its own, the separator is a space or hyphen and the second
/// word does not match a dictionary entry either. An exact joined match is a
/// segmentation repair and applies unconditionally, like the exact casing
/// restore; a fuzzy join requires both fragments below the confidence gate.
fn bigram_match<'a>(
    segments: &[(&str, &str)],
    i: usize,
    normalized: &'a HashMap<String, &'a String>,
    confidences: Option<&HashMap<String, f32>>,
    fuzzy_enabled: bool,
) -> Option<&'a String> {
    let (first, separator) = segments[i];
    let &(second, _) = segments.get(i + 1)?;
    if first.is_empty() || second.is_empty() || !matches!(separator, " " | "-") {
        return None;
    }

    let joined = format!("{}{}", normalize_word(first), normalize_word(second));
    if let Some(canonical) = normalized.get(&joined) {
        return Some(canonical);
    }

    if !fuzzy_enabled
        || joined.chars().count() < POSTCORR_LONG_LEN
        || best_dictionary_match(second, normalized, confidences, fuzzy_enabled).is_some()
    {
        return None;
    }

    if let Some(map) = confidences {
        let fragments_conf = [normalize_word(first), normalize_word(second)]
            .iter()
            .filter_map(|key| map.get(key))
            .fold(0.0f32, |acc, &conf| acc.max(conf));
        if fragments_conf >= POSTCORR_CONF_THRESHOLD {
            return None;
        }
    }

    let max_distance = if joined.chars().count() >= POSTCORR_BIGRAM_LONG_LEN {
        3
    } else {
        max_distance_for(joined.chars().count())
    };
    fuzzy_match(&joined, normalized, max_distance)
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

    let gated = confidences
        .and_then(|map| map.get(&target))
        .is_some_and(|&conf| conf >= POSTCORR_CONF_THRESHOLD);
    if gated {
        return None;
    }

    fuzzy_match(&target, normalized, max_distance_for(target_len))
}

/// Single strictly-closest key within `max_distance`, or `None` on a tie.
fn fuzzy_match<'a>(
    target: &str,
    normalized: &'a HashMap<String, &'a String>,
    max_distance: usize,
) -> Option<&'a String> {
    let mut best: Option<(usize, &String)> = None;
    let mut tied = false;
    for (key, canonical) in normalized {
        let dist = strsim::levenshtein(target, key);
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
        Some((dist, canonical)) if !tied && dist <= max_distance => {
            log::debug!(
                "dictionary post-correction: {} -> {} (d={})",
                target,
                canonical,
                dist
            );
            Some(canonical)
        }
        _ => None,
    }
}

/// Diagnostic for the eval harness: the fuzzy corrections the thresholds
/// would allow on `text`, gate ignored, with each word's model confidence.
#[cfg(test)]
pub fn fuzzy_correction_candidates(
    text: &str,
    dictionary: &[String],
    confidences: &HashMap<String, f32>,
) -> Vec<String> {
    let normalized: HashMap<String, &String> = dictionary
        .iter()
        .map(|key| (normalize_word(key), key))
        .collect();
    let mut out = Vec::new();
    for word in text.split(|c: char| !c.is_alphanumeric()) {
        let target = normalize_word(word);
        if target.is_empty() || normalized.contains_key(&target) {
            continue;
        }
        if let Some(canonical) = best_dictionary_match(word, &normalized, None, true) {
            let dist = strsim::levenshtein(&target, &normalize_word(canonical));
            let conf = confidences
                .get(&target)
                .map(|c| format!("{:.3}", c))
                .unwrap_or_else(|| "?".into());
            out.push(format!("{}→{} d={} p={}", word, canonical, dist, conf));
        }
    }
    out
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

    fn dict(words: &[&str]) -> Vec<String> {
        words.iter().map(|w| w.to_string()).collect()
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
        let dictionary: Vec<String> = Vec::new();
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
        let mut dictionary: Vec<String> = (0..POSTCORR_MAX_DICT_WORDS)
            .map(|i| format!("motdico{:03}", i))
            .collect();
        dictionary.push("Syntocinon".to_string());
        let out = restore_dictionary_casing("dose de sintocinon", &dictionary);
        assert_eq!(out, "dose de sintocinon");
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
    fn bigram_exact_join_repairs_segmentation() {
        let dictionary = dict(&["AppImage"]);
        let out = restore_dictionary_casing("disponible en app image.", &dictionary);
        assert_eq!(out, "disponible en AppImage.");
    }

    #[test]
    fn bigram_exact_join_repairs_hyphenation_even_when_confident() {
        let dictionary = dict(&["frontend"]);
        let conf: HashMap<String, f32> = [("front".to_string(), 1.0), ("end".to_string(), 1.0)].into();
        let out = restore_dictionary_casing_gated("le front-end moderne", &dictionary, Some(&conf));
        assert_eq!(out, "le frontend moderne");
    }

    #[test]
    fn bigram_fuzzy_joins_split_unknown_word() {
        let dictionary = dict(&["Syntocinon"]);
        let out = restore_dictionary_casing("J'ai pris du Saint-Occinon.", &dictionary);
        assert_eq!(out, "J'ai pris du Syntocinon.");
    }

    #[test]
    fn bigram_fuzzy_blocked_on_confident_fragment() {
        let dictionary = dict(&["Syntocinon"]);
        let conf: HashMap<String, f32> = [("saint".to_string(), 1.0)].into();
        let out =
            restore_dictionary_casing_gated("J'ai pris du Saint-Occinon.", &dictionary, Some(&conf));
        assert_eq!(out, "J'ai pris du Saint-Occinon.");
    }

    #[test]
    fn bigram_not_joined_across_sentence_punctuation() {
        let dictionary = dict(&["Syntocinon"]);
        let out = restore_dictionary_casing("Saint. Occinon est là.", &dictionary);
        assert_eq!(out, "Saint. Occinon est là.");
    }

    #[test]
    fn confidence_gate_blocks_fuzzy_on_elided_word() {
        let dictionary = dict(&["ordinateurs"]);
        // The engine emits "l'ordinateur" as one confident word; the gate
        // must cover the "ordinateur" fragment the restore pass looks up.
        let words = vec![("l'ordinateur".to_string(), 1.0)];
        let conf = super::confidence_map(&words);
        let out = restore_dictionary_casing_gated("l'ordinateur", &dictionary, Some(&conf));
        assert_eq!(out, "l'ordinateur");
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
