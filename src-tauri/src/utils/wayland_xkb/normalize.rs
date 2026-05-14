//! ASCII-fold transcribed text so the Direct path covers accents
//! without bailing to clipboard. Latin diacritics + curly punctuation
//! become ASCII, unmappable chars stay as-is for the runtime pre-scan.

// Returns `None` when the char is not in the fold table, the caller
// forwards it untouched.
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
// back to the diacritic fold and forwards as-is.
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

pub fn normalize_for_direct_typing(text: &str) -> String {
    // Most ASR output is already ASCII; sizing the output to the input
    // avoids a growth dance for the common case while still letting the
    // 'ß'→"ss" / 'œ'→"oe" expansions add bytes without panic.
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        if let Some(s) = fold_punctuation(c) {
            out.push_str(s);
        } else if let Some(s) = fold_diacritic(c) {
            out.push_str(s);
        } else {
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_clean_passes_through_unchanged() {
        let s = "Hello, world! 123.";
        assert_eq!(normalize_for_direct_typing(s), s);
    }

    #[test]
    fn empty_passes_through() {
        assert_eq!(normalize_for_direct_typing(""), "");
    }

    #[test]
    fn chars_outside_fold_table_stay_intact_for_fallback() {
        // Mathematical infinity, CJK char: not in either table, must be
        // forwarded so the pre-scan sees them and triggers the fallback.
        let s = "x\u{221E}y\u{4E2D}";
        assert_eq!(normalize_for_direct_typing(s), s);
    }

    #[test]
    fn mixed_input_handles_diacritic_and_punctuation_in_one_pass() {
        assert_eq!(
            normalize_for_direct_typing("L\u{2019}été \u{2013} ça va\u{2026}"),
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
            let actual = normalize_for_direct_typing(input);
            assert_eq!(actual, *expected, "input: {}", input);
        }
    }
}
