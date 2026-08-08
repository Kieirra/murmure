//! Resolve transcribed text against what the active layout can type.
//! The ASCII fold is the second chance, never the first: a char the
//! layout knows how to produce is always kept as-is.

// Returns `None` when the char is not in the fold table, the caller
// then drops it.
fn fold_diacritic(c: char) -> Option<&'static str> {
    match c {
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' => Some("a"),
        'ç' => Some("c"),
        'è' | 'é' | 'ê' | 'ë' => Some("e"),
        'ì' | 'í' | 'î' | 'ï' => Some("i"),
        'ñ' => Some("n"),
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' => Some("o"),
        'ù' | 'ú' | 'û' | 'ü' => Some("u"),
        'ý' | 'ÿ' => Some("y"),
        'ß' => Some("ss"),
        'æ' => Some("ae"),
        'œ' => Some("oe"),
        'ð' => Some("d"),
        'þ' => Some("th"),

        'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' => Some("A"),
        'Ç' => Some("C"),
        'È' | 'É' | 'Ê' | 'Ë' => Some("E"),
        'Ì' | 'Í' | 'Î' | 'Ï' => Some("I"),
        'Ñ' => Some("N"),
        'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø' => Some("O"),
        'Ù' | 'Ú' | 'Û' | 'Ü' => Some("U"),
        'Ý' => Some("Y"),
        'Æ' => Some("AE"),
        'Œ' => Some("OE"),
        'Ð' => Some("D"),
        'Þ' => Some("Th"),

        'ā' | 'ă' | 'ą' => Some("a"),
        'Ā' | 'Ă' | 'Ą' => Some("A"),
        'ć' | 'č' | 'ĉ' | 'ċ' => Some("c"),
        'Ć' | 'Č' | 'Ĉ' | 'Ċ' => Some("C"),
        'ď' | 'đ' => Some("d"),
        'Ď' | 'Đ' => Some("D"),
        'ē' | 'ĕ' | 'ė' | 'ę' | 'ě' => Some("e"),
        'Ē' | 'Ĕ' | 'Ė' | 'Ę' | 'Ě' => Some("E"),
        'ĝ' | 'ğ' | 'ġ' | 'ģ' => Some("g"),
        'Ĝ' | 'Ğ' | 'Ġ' | 'Ģ' => Some("G"),
        'ĥ' | 'ħ' => Some("h"),
        'Ĥ' | 'Ħ' => Some("H"),
        'ĩ' | 'ī' | 'ĭ' | 'į' | 'ı' => Some("i"),
        'Ĩ' | 'Ī' | 'Ĭ' | 'Į' | 'İ' => Some("I"),
        'ĵ' => Some("j"),
        'Ĵ' => Some("J"),
        'ķ' => Some("k"),
        'Ķ' => Some("K"),
        'ĺ' | 'ļ' | 'ľ' | 'ŀ' | 'ł' => Some("l"),
        'Ĺ' | 'Ļ' | 'Ľ' | 'Ŀ' | 'Ł' => Some("L"),
        'ń' | 'ņ' | 'ň' => Some("n"),
        'Ń' | 'Ņ' | 'Ň' => Some("N"),
        'ō' | 'ŏ' | 'ő' => Some("o"),
        'Ō' | 'Ŏ' | 'Ő' => Some("O"),
        'ŕ' | 'ŗ' | 'ř' => Some("r"),
        'Ŕ' | 'Ŗ' | 'Ř' => Some("R"),
        'ś' | 'ŝ' | 'ş' | 'š' => Some("s"),
        'Ś' | 'Ŝ' | 'Ş' | 'Š' => Some("S"),
        'ţ' | 'ť' | 'ŧ' => Some("t"),
        'Ţ' | 'Ť' | 'Ŧ' => Some("T"),
        'ũ' | 'ū' | 'ŭ' | 'ů' | 'ű' | 'ų' => Some("u"),
        'Ũ' | 'Ū' | 'Ŭ' | 'Ů' | 'Ű' | 'Ų' => Some("U"),
        'ŵ' => Some("w"),
        'Ŵ' => Some("W"),
        'ŷ' | 'Ÿ' => Some("y"),
        'Ŷ' => Some("Y"),
        'ź' | 'ż' | 'ž' => Some("z"),
        'Ź' | 'Ż' | 'Ž' => Some("Z"),

        _ => None,
    }
}

// Returns `None` for chars not handled here, the caller then falls
// back to the diacritic fold.
fn fold_punctuation(c: char) -> Option<&'static str> {
    match c {
        // Curly single quotes → ASCII apostrophe.
        '\u{2018}' | '\u{2019}' | '\u{201A}' | '\u{201B}' => Some("'"),
        // Curly double quotes and French chevrons → dropped (no clean
        // ASCII analogue; Parakeet rarely emits them).
        '\u{201C}' | '\u{201D}' | '\u{201E}' | '\u{201F}' | '\u{00AB}' | '\u{00BB}' => Some(""),
        // Dashes → ASCII hyphen-minus.
        '\u{2013}' | '\u{2014}' | '\u{2015}' | '\u{2212}' => Some("-"),
        // Horizontal ellipsis → single period.
        '\u{2026}' => Some("."),
        // Non-breaking spaces → regular space.
        '\u{00A0}' | '\u{202F}' => Some(" "),
        _ => None,
    }
}

// Dropping a char keeps the rest of the dictation flowing instead of
// bailing to the clipboard. The second member of the pair counts the
// input chars that produced nothing, so callers can log a number
// without ever logging the text.
pub fn resolve_for_typing(text: &str, is_typable: impl Fn(char) -> bool) -> (String, usize) {
    // Most ASR output is already ASCII; sizing the output to the input
    // avoids a growth dance for the common case while still letting the
    // 'ß'→"ss" / 'œ'→"oe" expansions add bytes without panic.
    let mut out = String::with_capacity(text.len());
    let mut dropped = 0usize;
    for c in text.chars() {
        let before = out.len();
        if is_typable(c) {
            out.push(c);
        } else if let Some(folded) = fold_punctuation(c).or_else(|| fold_diacritic(c)) {
            out.extend(folded.chars().filter(|&f| is_typable(f)));
        }
        if out.len() == before {
            dropped += 1;
        }
    }
    (out, dropped)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Stands in for a US layout: nothing outside ASCII can be typed.
    fn ascii_only(c: char) -> bool {
        c.is_ascii()
    }

    fn fold_to_ascii(text: &str) -> String {
        resolve_for_typing(text, ascii_only).0
    }

    #[test]
    fn ascii_clean_passes_through_unchanged() {
        let s = "Hello, world! 123.";
        assert_eq!(fold_to_ascii(s), s);
    }

    #[test]
    fn empty_passes_through() {
        assert_eq!(fold_to_ascii(""), "");
    }

    #[test]
    fn typable_chars_are_never_folded() {
        let s = "L\u{2019}été \u{2013} ça va\u{2026} Straße";
        assert_eq!(resolve_for_typing(s, |_| true).0, s);
    }

    #[test]
    fn folds_only_the_chars_the_layout_cannot_type() {
        let typable = |c: char| c.is_ascii() || c == 'é';
        assert_eq!(
            resolve_for_typing("Straße et été", typable).0,
            "Strasse et été"
        );
    }

    #[test]
    fn fold_output_is_filtered_by_typability_too() {
        // 'ß' folds to "ss" but the layout has no 's': nothing is typed.
        let no_s = |c: char| c.is_ascii() && c != 's';
        assert_eq!(resolve_for_typing("aßb", no_s).0, "ab");
    }

    #[test]
    fn dropped_count_only_counts_chars_that_produced_nothing() {
        // 'é' folds to "e" (kept), the emoji and the chevrons produce
        // nothing.
        let (out, dropped) = resolve_for_typing("é\u{1F600}a\u{00AB}b\u{00BB}", ascii_only);
        assert_eq!(out, "eab");
        assert_eq!(dropped, 3);
    }

    #[test]
    fn dropped_count_is_zero_when_everything_is_typable() {
        let (_, dropped) = resolve_for_typing("Hello, world!", ascii_only);
        assert_eq!(dropped, 0);
    }

    #[test]
    fn dropped_count_covers_every_char_when_nothing_is_typable() {
        let (out, dropped) = resolve_for_typing("abc", |_| false);
        assert!(out.is_empty());
        assert_eq!(dropped, 3);
    }

    #[test]
    fn untypable_and_unfoldable_chars_are_skipped() {
        // Emoji, CJK and Cyrillic have no fold entry: dropped, and the
        // surrounding text is still typed.
        assert_eq!(fold_to_ascii("a\u{1F600}b\u{4E2D}c\u{0439}d"), "abcd");
    }

    #[test]
    fn mixed_input_handles_diacritic_and_punctuation_in_one_pass() {
        assert_eq!(
            fold_to_ascii("L\u{2019}été \u{2013} ça va\u{2026}"),
            "L'ete - ca va.",
        );
    }

    #[test]
    fn folds_match_expected_ascii() {
        let cases: &[(&str, &str)] = &[
            // French accents
            ("Café", "Cafe"),
            ("naïve", "naive"),
            ("été", "ete"),
            ("François", "Francois"),
            ("où", "ou"),
            // German accents and eszett
            ("über", "uber"),
            ("Straße", "Strasse"),
            ("schön", "schon"),
            // Spanish and Portuguese accents
            ("España", "Espana"),
            ("São Paulo", "Sao Paulo"),
            ("açúcar", "acucar"),
            // Ligatures
            ("cœur", "coeur"),
            ("Œuvre", "OEuvre"),
            ("naïveté æquus", "naivete aequus"),
            // Typographic apostrophes
            ("l\u{2019}arbre", "l'arbre"),
            ("\u{2018}quoted\u{2019}", "'quoted'"),
            // Em and en dashes
            ("rendez\u{2013}vous", "rendez-vous"),
            ("yes\u{2014}or no", "yes-or no"),
            // Horizontal ellipsis
            ("fini\u{2026}", "fini."),
            // French chevrons and curly double quotes dropped
            ("\u{00AB}bonjour\u{00BB}", "bonjour"),
            ("\u{201C}hello\u{201D}", "hello"),
            // Non-breaking spaces normalised
            ("a\u{00A0}b", "a b"),
            ("a\u{202F}b", "a b"),
        ];
        for (input, expected) in cases {
            let actual = fold_to_ascii(input);
            assert_eq!(actual, *expected, "input: {}", input);
        }
    }
}
