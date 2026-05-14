//! ASCII subset typeable via XKB in direct mode. Chars outside fall
//! back to clipboard+Ctrl+V since most accented chars need AltGr,
//! dead keys, or compose sequences (out of scope).

pub const PARAKEET_SUBSET: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', ' ', ',', '.', '!', '?', '\'', '-', ':', ';', '(', ')',
];

pub fn is_in_subset(c: char) -> bool {
    PARAKEET_SUBSET.contains(&c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subset_contains_all_declared_chars() {
        for &c in PARAKEET_SUBSET {
            assert!(is_in_subset(c), "char '{}' must be in subset", c);
        }
    }

    #[test]
    fn subset_rejects_accents_and_control_chars() {
        assert!(!is_in_subset('é'));
        assert!(!is_in_subset('ç'));
        assert!(!is_in_subset('\n'));
        assert!(!is_in_subset('\t'));
        assert!(!is_in_subset('@'));
        // Typographic quotes and dashes remain outside the subset. The
        // normalize step ASCII-folds them before injection.
        assert!(!is_in_subset('\u{2019}')); // right single quote
        assert!(!is_in_subset('\u{2013}')); // en dash
    }
}
