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

/// Restore the dictionary's canonical spelling on whole-word matches, with a
/// strict fuzzy fallback. Exact normalized matches (distance 0) just rewrite
/// casing/accents to the stored spelling; near matches under the strict
/// thresholds are corrected to the closest dictionary key (mur acoustique /
/// segmentation cases the greedy boost cannot fix). Ambiguous near matches
/// (a tie on the minimal distance between two keys) are left untouched.
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
            match best_dictionary_match(word, &normalized) {
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
) -> Option<&'a String> {
    let target = normalize_word(word);
    if target.is_empty() {
        return None;
    }

    if let Some(canonical) = normalized.get(&target) {
        return Some(canonical);
    }

    let target_len = target.chars().count();
    if target_len < POSTCORR_MIN_LEN {
        return None;
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
