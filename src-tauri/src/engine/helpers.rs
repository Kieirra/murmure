use std::collections::HashMap;

use super::transcription_engine::TranscriptionSegment;
use super::types::{Segment, TimestampGranularity, TimestampedResult, Token, Utterance, Word};

/// Greedy longest-prefix tokenization of a single word against the vocab.
/// The word is lowercased and space-prefixed (SentencePiece word boundary).
/// Lowercase because the Parakeet vocab holds no capitalized tokens; casing is
/// restored in post-process. Returns `None` if any remainder cannot be matched.
pub fn tokenize_word_to_ids(word: &str, vocab_lookup: &HashMap<&str, i32>) -> Option<Vec<i32>> {
    let prefixed = format!(" {}", word.to_lowercase());
    let mut remainder: &str = &prefixed;
    let mut ids = Vec::new();

    while !remainder.is_empty() {
        let mut matched: Option<(&str, i32)> = None;
        for end in (1..=remainder.len()).rev() {
            if !remainder.is_char_boundary(end) {
                continue;
            }
            let candidate = &remainder[..end];
            if let Some(&id) = vocab_lookup.get(candidate) {
                matched = Some((candidate, id));
                break;
            }
        }
        let (piece, id) = matched?;
        ids.push(id);
        remainder = &remainder[piece.len()..];
    }

    if ids.is_empty() {
        None
    } else {
        Some(ids)
    }
}

/// Index from vocab token text to its id, skipping empty slots.
pub fn build_vocab_lookup(vocab: &[String]) -> HashMap<&str, i32> {
    vocab
        .iter()
        .enumerate()
        .filter(|(_, token)| !token.is_empty())
        .map(|(id, token)| (token.as_str(), id as i32))
        .collect()
}

pub fn convert_timestamps(
    timestamped_result: &TimestampedResult,
    granularity: TimestampGranularity,
) -> Vec<TranscriptionSegment> {
    match granularity {
        TimestampGranularity::Token => convert_to_raw_token_segments(timestamped_result),
        TimestampGranularity::Word => convert_to_hierarchical_word_segments(timestamped_result),
        TimestampGranularity::Segment => {
            convert_to_hierarchical_segment_segments(timestamped_result)
        }
    }
}

// Returns the raw tokens exactly as output by the model
fn convert_to_raw_token_segments(
    timestamped_result: &TimestampedResult,
) -> Vec<TranscriptionSegment> {
    let mut segments = Vec::new();

    for (i, (token, &timestamp)) in timestamped_result
        .tokens
        .iter()
        .zip(timestamped_result.timestamps.iter())
        .enumerate()
    {
        // Include ALL tokens, even empty ones, for debugging purposes
        let end_timestamp = timestamped_result
            .timestamps
            .get(i + 1)
            .copied()
            .unwrap_or(timestamp + 0.05); // Small default duration for tokens

        segments.push(TranscriptionSegment {
            start: timestamp,
            end: end_timestamp,
            text: token.clone(), // Raw token text, including spaces and subword pieces
        });
    }

    segments
}

// Uses hierarchical approach for clean word-level timestamps
fn convert_to_hierarchical_word_segments(
    timestamped_result: &TimestampedResult,
) -> Vec<TranscriptionSegment> {
    let segment_separators = ['.', '?', '!'];
    let word_separator = ' ';

    let utterance =
        build_utterance_from_tokens(timestamped_result, &segment_separators, word_separator);

    extract_word_segments(&utterance)
}

// Uses hierarchical approach for clean segment-level timestamps
fn convert_to_hierarchical_segment_segments(
    timestamped_result: &TimestampedResult,
) -> Vec<TranscriptionSegment> {
    let segment_separators = ['.', '?', '!'];
    let word_separator = ' ';

    let utterance =
        build_utterance_from_tokens(timestamped_result, &segment_separators, word_separator);

    extract_segment_segments(&utterance)
}

fn build_utterance_from_tokens(
    timestamped_result: &TimestampedResult,
    segment_separators: &[char],
    word_separator: char,
) -> Utterance {
    // Handle empty input
    if timestamped_result.tokens.is_empty() || timestamped_result.timestamps.is_empty() {
        return Utterance {
            text: timestamped_result.text.clone(),
            segments: if timestamped_result.text.trim().is_empty() {
                Vec::new()
            } else {
                vec![Segment {
                    text: timestamped_result.text.clone(),
                    t_start: 0.0,
                    t_end: 0.0,
                    words: Vec::new(),
                }]
            },
        };
    }

    // Step 1: Create tokens from the timestamped result
    let tokens = create_tokens_from_timestamped_result(timestamped_result);

    // Step 2: Group tokens into words
    let words = group_tokens_into_words_hierarchical(&tokens, word_separator);

    // Step 3: Group words into segments
    let segments = group_words_into_segments(&words, segment_separators);

    Utterance {
        text: timestamped_result.text.clone(),
        segments,
    }
}

fn create_tokens_from_timestamped_result(timestamped_result: &TimestampedResult) -> Vec<Token> {
    let mut tokens = Vec::new();

    for (i, (token_text, &timestamp)) in timestamped_result
        .tokens
        .iter()
        .zip(timestamped_result.timestamps.iter())
        .enumerate()
    {
        let t_end = timestamped_result
            .timestamps
            .get(i + 1)
            .copied()
            .unwrap_or(timestamp + 0.05); // Small default duration for final token

        tokens.push(Token {
            text: token_text.clone(),
            token_id: Some(i),
            t_start: timestamp,
            t_end,
            is_blank: token_text.trim().is_empty(),
        });
    }

    tokens
}

fn group_tokens_into_words_hierarchical(tokens: &[Token], word_separator: char) -> Vec<Word> {
    let mut words = Vec::new();
    let mut current_word_tokens = Vec::new();

    for token in tokens {
        if token.is_blank {
            continue;
        }

        // Check if this token starts a new word
        // This handles subword tokens (like from tokenizers) and space-separated tokens
        let starts_new_word = token.text.starts_with(word_separator) ||
                             token.text.starts_with(" ") || // SentencePiece word boundary
                             (current_word_tokens.is_empty() && !token.text.trim().is_empty());

        if starts_new_word && !current_word_tokens.is_empty() {
            // Finish the current word
            let word = create_word_from_tokens(&current_word_tokens);
            if !word.text.is_empty() {
                words.push(word);
            }
            current_word_tokens.clear();
        }

        current_word_tokens.push(token.clone());
    }

    // Add the final word if there are tokens
    if !current_word_tokens.is_empty() {
        let word = create_word_from_tokens(&current_word_tokens);
        if !word.text.is_empty() {
            words.push(word);
        }
    }

    words
}

fn create_word_from_tokens(tokens: &[Token]) -> Word {
    if tokens.is_empty() {
        return Word {
            text: String::new(),
            t_start: 0.0,
            t_end: 0.0,
            tokens: Vec::new(),
        };
    }

    // Aggregate timestamps: word start = first token start, word end = last token end
    let t_start = tokens.first().unwrap().t_start;
    let t_end = tokens.last().unwrap().t_end;

    // Combine token text, handling subword tokens properly
    let text = tokens
        .iter()
        .map(|t| {
            // Handle SentencePiece tokens that start with
            if t.text.starts_with(" ") {
                t.text.strip_prefix(" ").unwrap_or(&t.text)
            } else if t.text.starts_with(' ') {
                t.text.strip_prefix(' ').unwrap_or(&t.text)
            } else {
                &t.text
            }
        })
        .collect::<String>()
        .trim()
        .to_string();

    Word {
        text,
        t_start,
        t_end,
        tokens: tokens.to_vec(),
    }
}

fn group_words_into_segments(words: &[Word], segment_separators: &[char]) -> Vec<Segment> {
    if words.is_empty() {
        return Vec::new();
    }

    let mut segments = Vec::new();
    let mut current_segment_words = Vec::new();

    for (i, word) in words.iter().enumerate() {
        current_segment_words.push(word.clone());

        // Check if word ends with segment separator or if it's the last word
        let ends_segment =
            word.text.chars().any(|c| segment_separators.contains(&c)) || i == words.len() - 1; // Always end on last word

        if ends_segment {
            let segment = create_segment_from_words(&current_segment_words);
            if !segment.text.is_empty() {
                segments.push(segment);
            }
            current_segment_words.clear();
        }
    }

    // Handle case where no punctuation was found - create one big segment
    if segments.is_empty() && !words.is_empty() {
        let segment = create_segment_from_words(words);
        if !segment.text.is_empty() {
            segments.push(segment);
        }
    }

    segments
}

fn create_segment_from_words(words: &[Word]) -> Segment {
    if words.is_empty() {
        return Segment {
            text: String::new(),
            t_start: 0.0,
            t_end: 0.0,
            words: Vec::new(),
        };
    }

    // Aggregate timestamps: segment start = first word start, segment end = last word end
    let t_start = words.first().unwrap().t_start;
    let t_end = words.last().unwrap().t_end;

    // Combine word text with spaces
    let text = words
        .iter()
        .map(|w| w.text.as_str())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    Segment {
        text,
        t_start,
        t_end,
        words: words.to_vec(),
    }
}

fn extract_word_segments(utterance: &Utterance) -> Vec<TranscriptionSegment> {
    let mut segments = Vec::new();

    for segment in &utterance.segments {
        for word in &segment.words {
            if !word.text.trim().is_empty() {
                segments.push(TranscriptionSegment {
                    start: word.t_start,
                    end: word.t_end,
                    text: word.text.clone(),
                });
            }
        }
    }

    segments
}

fn extract_segment_segments(utterance: &Utterance) -> Vec<TranscriptionSegment> {
    utterance
        .segments
        .iter()
        .filter(|segment| !segment.text.trim().is_empty())
        .map(|segment| TranscriptionSegment {
            start: segment.t_start,
            end: segment.t_end,
            text: segment.text.clone(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toy_vocab() -> Vec<String> {
        // Mirrors the relevant slots of the real Parakeet vocab (▁ -> space).
        let mut vocab = vec![String::new(); 4388];
        vocab[289] = "on".to_string();
        vocab[434] = "to".to_string();
        vocab[4387] = "cin".to_string();
        vocab[4208] = " syn".to_string();
        vocab
    }

    #[test]
    fn tokenize_syntocinon_matches_expected_ids() {
        let vocab = toy_vocab();
        let lookup = build_vocab_lookup(&vocab);
        let ids = tokenize_word_to_ids("syntocinon", &lookup);
        assert_eq!(ids, Some(vec![4208, 434, 4387, 289]));
    }

    #[test]
    fn tokenize_is_case_insensitive() {
        let vocab = toy_vocab();
        let lookup = build_vocab_lookup(&vocab);
        let ids = tokenize_word_to_ids("Syntocinon", &lookup);
        assert_eq!(ids, Some(vec![4208, 434, 4387, 289]));
    }

    #[test]
    fn tokenize_returns_none_when_not_tokenizable() {
        let vocab = toy_vocab();
        let lookup = build_vocab_lookup(&vocab);
        // 'z' is absent from the toy vocab, so the remainder cannot be matched.
        assert_eq!(tokenize_word_to_ids("zzz", &lookup), None);
    }
}
