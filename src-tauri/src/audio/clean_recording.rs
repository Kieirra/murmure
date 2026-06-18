use crate::audio::types::AudioState;
use crate::wake_word::wake_word::normalize_text;
use log::{debug, error};
use strsim::levenshtein;
use tauri::AppHandle;

pub(super) fn strip_and_record(app: &AppHandle, state: &AudioState, text: String) -> String {
    match state.strip_word.lock().take() {
        Some(word) => {
            let stripped = strip_trailing_wake_word(&text, &word);
            if stripped != text {
                if let Err(e) = crate::history::update_last_transcription(app, stripped.clone()) {
                    error!("Failed to update history after wake word strip: {}", e);
                }
            }
            stripped
        }
        None => text,
    }
}

fn strip_trailing_wake_word(text: &str, wake_word: &str) -> String {
    let ww = wake_word.trim();
    if ww.is_empty() {
        return text.to_string();
    }

    let trimmed = text.trim();
    let text_words: Vec<&str> = trimmed.split_whitespace().collect();

    let ww_normalized = normalize_text(ww);
    let ww_words: Vec<&str> = ww_normalized.split_whitespace().collect();

    if text_words.len() < ww_words.len() {
        return trimmed.to_string();
    }

    // Search within the last words with a margin of 2 for trailing noise from STT
    let margin = 2;
    let earliest_start = text_words.len().saturating_sub(ww_words.len() + margin);

    for start in earliest_start..=(text_words.len() - ww_words.len()) {
        let candidate = &text_words[start..start + ww_words.len()];

        let all_match = candidate.iter().zip(ww_words.iter()).all(|(tw, ww_w)| {
            let tw_norm = normalize_text(tw);
            let max_distance = if ww_w.len() <= 3 { 1 } else { 2 };
            levenshtein(&tw_norm, ww_w) <= max_distance
        });

        if all_match {
            let result = text_words[..start].join(" ");
            debug!(
                "Stripped trailing wake word \"{}\" from transcription",
                wake_word
            );
            return result;
        }
    }

    trimmed.to_string()
}

static FILLER_WORDS: &[&str] = &[
    "euh", "hmm", "hm", "mmm", "uh", "um", "uhm", "umm", "uhh", "uhhh", "ah", "mm", "mh", "eh",
    "ehh", "ha", "mm-hmm",
];

fn is_filler(word: &str) -> bool {
    let normalized = word.trim_matches(|c: char| !c.is_alphanumeric());
    if normalized.is_empty() {
        return false;
    }
    let lower = normalized.to_lowercase();
    FILLER_WORDS.contains(&lower.as_str())
}

pub(super) fn strip_fillers_and_repeats(text: &str) -> String {
    let words: Vec<&str> = text.split_whitespace().filter(|w| !is_filler(w)).collect();
    if words.is_empty() {
        return String::new();
    }

    let mut result: Vec<&str> = Vec::with_capacity(words.len());
    let mut i = 0;

    while i < words.len() {
        let current_lower = words[i].to_lowercase();
        let mut count = 1;

        while i + count < words.len() && words[i + count].to_lowercase() == current_lower {
            count += 1;
        }

        if count >= 3 {
            result.push(words[i]);
            result.push(words[i + 1]);
        } else {
            for j in 0..count {
                result.push(words[i + j]);
            }
        }

        i += count;
    }

    result.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_exact_match_single_word() {
        assert_eq!(
            strip_trailing_wake_word("bonjour validate", "validate"),
            "bonjour"
        );
    }

    #[test]
    fn strip_exact_match_multi_word() {
        assert_eq!(
            strip_trailing_wake_word("bonjour le monde alix validate", "alix validate"),
            "bonjour le monde"
        );
    }

    #[test]
    fn strip_fuzzy_match_accent() {
        // STT transcribes "validé" instead of "validate" — Levenshtein ≤ 2
        assert_eq!(
            strip_trailing_wake_word("bonjour alix validé", "alix validate"),
            "bonjour"
        );
    }

    #[test]
    fn strip_fuzzy_match_typo() {
        // STT transcribes "validatte" — Levenshtein ≤ 2
        assert_eq!(
            strip_trailing_wake_word("bonjour alix validatte", "alix validate"),
            "bonjour"
        );
    }

    #[test]
    fn strip_fuzzy_match_missing_char() {
        // STT transcribes "validat" — Levenshtein = 1
        assert_eq!(
            strip_trailing_wake_word("bonjour alix validat", "alix validate"),
            "bonjour"
        );
    }

    #[test]
    fn strip_with_trailing_noise() {
        // Trailing noise word after wake word — margin handles it
        assert_eq!(
            strip_trailing_wake_word("bonjour alix validate ok", "alix validate"),
            "bonjour"
        );
    }

    #[test]
    fn strip_case_insensitive() {
        assert_eq!(
            strip_trailing_wake_word("bonjour Alix Validate", "alix validate"),
            "bonjour"
        );
    }

    #[test]
    fn strip_no_match_returns_original() {
        assert_eq!(
            strip_trailing_wake_word("bonjour le monde", "alix validate"),
            "bonjour le monde"
        );
    }

    #[test]
    fn strip_empty_wake_word() {
        assert_eq!(
            strip_trailing_wake_word("bonjour le monde", ""),
            "bonjour le monde"
        );
    }

    #[test]
    fn strip_text_shorter_than_wake_word() {
        assert_eq!(
            strip_trailing_wake_word("validate", "alix validate"),
            "validate"
        );
    }

    #[test]
    fn strip_only_wake_word() {
        assert_eq!(
            strip_trailing_wake_word("alix validate", "alix validate"),
            ""
        );
    }

    #[test]
    fn strip_with_punctuation_from_stt() {
        assert_eq!(
            strip_trailing_wake_word("bonjour alix validate.", "alix validate"),
            "bonjour"
        );
    }

    #[test]
    fn dedup_four_to_two() {
        assert_eq!(strip_fillers_and_repeats("je je je je vais"), "je je vais");
    }

    #[test]
    fn dedup_two_kept_unchanged() {
        assert_eq!(strip_fillers_and_repeats("oui oui"), "oui oui");
    }

    #[test]
    fn dedup_five_to_two() {
        assert_eq!(
            strip_fillers_and_repeats("the the the the the cat"),
            "the the cat"
        );
    }

    #[test]
    fn dedup_three_to_two_case_insensitive() {
        assert_eq!(
            strip_fillers_and_repeats("Hello HELLO hello world"),
            "Hello HELLO world"
        );
    }

    #[test]
    fn dedup_no_repetition() {
        assert_eq!(
            strip_fillers_and_repeats("normal sentence"),
            "normal sentence"
        );
    }

    #[test]
    fn dedup_empty_string() {
        assert_eq!(strip_fillers_and_repeats(""), "");
    }

    #[test]
    fn dedup_single_word() {
        assert_eq!(strip_fillers_and_repeats("word"), "word");
    }

    #[test]
    fn dedup_multiple_groups() {
        assert_eq!(
            strip_fillers_and_repeats("the the the cat the the the dog"),
            "the the cat the the dog"
        );
    }

    #[test]
    fn dedup_exactly_three_to_two() {
        assert_eq!(
            strip_fillers_and_repeats("hello hello hello world"),
            "hello hello world"
        );
    }

    #[test]
    fn dedup_one_occurrence_unchanged() {
        assert_eq!(strip_fillers_and_repeats("hello world"), "hello world");
    }

    #[test]
    fn filler_isolated_removed() {
        assert_eq!(strip_fillers_and_repeats("je euh vais"), "je vais");
    }

    #[test]
    fn filler_repeated_fully_removed() {
        assert_eq!(strip_fillers_and_repeats("euh euh euh bonjour"), "bonjour");
    }

    #[test]
    fn filler_substring_in_real_word_kept() {
        assert_eq!(
            strip_fillers_and_repeats("aujourd'hui ah hammer"),
            "aujourd'hui hammer"
        );
    }

    #[test]
    fn filler_mm_hmm_removed() {
        assert_eq!(
            strip_fillers_and_repeats("oui mm-hmm bonjour"),
            "oui bonjour"
        );
    }
}
