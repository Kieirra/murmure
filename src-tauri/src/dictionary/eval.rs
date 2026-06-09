//! Manual evaluation harness for the dictionary pipeline (phrase boosting +
//! fuzzy post-correction). Not run in CI: it needs the local `eval/` corpus
//! (gitignored) and the bundled Parakeet model.
//!
//! Layout of `eval/` at the repo root:
//! - `phrases.txt`: one reference sentence per line, line N is the ground
//!   truth of `N.wav`
//! - `dictionary.txt`: one dictionary word per line
//! - `1.wav`, `2.wav`, ...: 16-bit PCM recordings of the sentences (any
//!   sample rate, mono or stereo)
//!
//! Run with:
//! `cargo test --release dictionary_eval -- --ignored --nocapture`

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::audio::helpers::read_wav_samples;
use crate::dictionary::restore_dictionary_casing;
use crate::engine::helpers::fold_accents;
use crate::engine::transcription_engine::TranscriptionEngine;
use crate::engine::{ParakeetEngine, ParakeetModelParams};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// Lowercased alphanumeric tokens, accents preserved: WER counts a missing
/// accent as an error (the pipeline is supposed to restore it) but ignores
/// casing and punctuation.
fn tokens(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(str::to_string)
        .collect()
}

/// Word-level edit distance (two-row Levenshtein over tokens).
fn word_errors(reference: &[String], hypothesis: &[String]) -> usize {
    let mut prev: Vec<usize> = (0..=hypothesis.len()).collect();
    let mut curr = vec![0; hypothesis.len() + 1];
    for (i, r) in reference.iter().enumerate() {
        curr[0] = i + 1;
        for (j, h) in hypothesis.iter().enumerate() {
            let cost = usize::from(r != h);
            curr[j + 1] = (prev[j + 1] + 1).min(curr[j] + 1).min(prev[j] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[hypothesis.len()]
}

/// Dictionary words present in `text`, matched accent-folded and lowercased
/// (the same normalization as the post-correction).
fn terms_in(text: &str, dico: &[String]) -> HashSet<String> {
    let toks: HashSet<String> = tokens(&fold_accents(text)).into_iter().collect();
    dico.iter()
        .filter(|w| toks.contains(&fold_accents(w).to_lowercase()))
        .cloned()
        .collect()
}

fn percent(errors: usize, total: usize) -> f32 {
    if total == 0 {
        return 0.0;
    }
    100.0 * errors as f32 / total as f32
}

fn sorted(set: HashSet<&String>) -> String {
    let mut list: Vec<&str> = set.into_iter().map(String::as_str).collect();
    if list.is_empty() {
        return "-".to_string();
    }
    list.sort_unstable();
    list.join(", ")
}

#[test]
#[ignore = "manual: needs the eval/ corpus and the bundled model"]
fn dictionary_eval() {
    let root = repo_root();
    let eval_dir = root.join("eval");

    let phrases = match std::fs::read_to_string(eval_dir.join("phrases.txt")) {
        Ok(content) => content,
        Err(_) => {
            println!("eval/phrases.txt introuvable, rien à évaluer");
            return;
        }
    };
    let dico: Vec<String> = std::fs::read_to_string(eval_dir.join("dictionary.txt"))
        .expect("eval/dictionary.txt introuvable")
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::to_string)
        .collect();

    let model_dir = root.join("resources/parakeet-tdt-0.6b-v3-int8");
    if !model_dir.exists() {
        println!("modèle absent: {}", model_dir.display());
        return;
    }

    let mut params = ParakeetModelParams::int8();
    params.tokenizer_path = Some(root.join("resources/tokenizer.json"));
    let mut engine = ParakeetEngine::new();
    engine
        .load_model_with_params(&model_dir, params)
        .expect("chargement du modèle");

    let dict_map: HashMap<String, Vec<String>> =
        dico.iter().map(|w| (w.clone(), Vec::new())).collect();

    let mut evaluated = 0usize;
    let mut total_ref_words = 0usize;
    let mut base_errors = 0usize;
    let mut boost_errors = 0usize;
    let mut expected_terms = 0usize;
    let mut base_hits = 0usize;
    let mut boost_hits = 0usize;
    let mut base_fp = 0usize;
    let mut boost_fp = 0usize;

    for (i, sentence) in phrases.lines().enumerate() {
        let sentence = sentence.trim();
        if sentence.is_empty() {
            continue;
        }
        let wav = eval_dir.join(format!("{}.wav", i + 1));
        if !wav.exists() {
            println!("[{:>2}] {} absent, phrase ignorée", i + 1, wav.display());
            continue;
        }
        let samples = read_wav_samples(&wav).expect("lecture wav");

        engine.set_boost_words(&[]);
        let baseline = engine
            .transcribe_samples(samples.clone(), None)
            .expect("transcription baseline")
            .text
            .trim()
            .to_string();

        engine.set_boost_words(&dico);
        let boosted_raw = engine
            .transcribe_samples(samples, None)
            .expect("transcription boostée")
            .text;
        let boosted = restore_dictionary_casing(boosted_raw.trim(), &dict_map);

        let ref_toks = tokens(sentence);
        let base_we = word_errors(&ref_toks, &tokens(&baseline));
        let boost_we = word_errors(&ref_toks, &tokens(&boosted));

        let gt_terms = terms_in(sentence, &dico);
        let base_terms = terms_in(&baseline, &dico);
        let boost_terms = terms_in(&boosted, &dico);

        evaluated += 1;
        total_ref_words += ref_toks.len();
        base_errors += base_we;
        boost_errors += boost_we;
        expected_terms += gt_terms.len();
        base_hits += base_terms.intersection(&gt_terms).count();
        boost_hits += boost_terms.intersection(&gt_terms).count();
        base_fp += base_terms.difference(&gt_terms).count();
        boost_fp += boost_terms.difference(&gt_terms).count();

        println!("[{:>2}] réf     : {}", i + 1, sentence);
        println!(
            "     base    : {} (WER {:.1}%)",
            baseline,
            percent(base_we, ref_toks.len())
        );
        println!(
            "     boosté  : {} (WER {:.1}%)",
            boosted,
            percent(boost_we, ref_toks.len())
        );
        println!(
            "     termes  : base {}/{} | boosté {}/{} | FP base: {} | FP boosté: {}",
            base_terms.intersection(&gt_terms).count(),
            gt_terms.len(),
            boost_terms.intersection(&gt_terms).count(),
            gt_terms.len(),
            sorted(base_terms.difference(&gt_terms).collect()),
            sorted(boost_terms.difference(&gt_terms).collect()),
        );
        println!();
    }

    if evaluated == 0 {
        println!("aucun fichier wav trouvé dans {}", eval_dir.display());
        return;
    }

    println!("=== Bilan sur {} phrase(s) ===", evaluated);
    println!(
        "WER          : base {:.1}% | boosté {:.1}%",
        percent(base_errors, total_ref_words),
        percent(boost_errors, total_ref_words)
    );
    println!(
        "Recall termes: base {}/{} | boosté {}/{}",
        base_hits, expected_terms, boost_hits, expected_terms
    );
    println!("Faux positifs: base {} | boosté {}", base_fp, boost_fp);
}
