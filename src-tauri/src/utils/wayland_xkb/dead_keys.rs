//! Dead key compositions covering the Parakeet vocabulary, as
//! `(dead keysym, base char, composed char)`. Letters without a Unicode
//! decomposition (`ß ø æ œ ł đ ħ`, Greek and Cyrillic bases) are direct
//! keys on their native layouts and are found by the level probing.

use xkbcommon::xkb::keysyms;

pub const DEAD_KEY_COMPOSITIONS: &[(u32, char, char)] = &[
    (keysyms::KEY_dead_grave, 'A', 'À'),
    (keysyms::KEY_dead_grave, 'E', 'È'),
    (keysyms::KEY_dead_grave, 'I', 'Ì'),
    (keysyms::KEY_dead_grave, 'a', 'à'),
    (keysyms::KEY_dead_grave, 'e', 'è'),
    (keysyms::KEY_dead_grave, 'i', 'ì'),
    (keysyms::KEY_dead_grave, 'o', 'ò'),
    (keysyms::KEY_dead_grave, 'u', 'ù'),
    (keysyms::KEY_dead_acute, 'A', 'Á'),
    (keysyms::KEY_dead_acute, 'E', 'É'),
    (keysyms::KEY_dead_acute, 'I', 'Í'),
    (keysyms::KEY_dead_acute, 'O', 'Ó'),
    (keysyms::KEY_dead_acute, 'U', 'Ú'),
    (keysyms::KEY_dead_acute, 'Y', 'Ý'),
    (keysyms::KEY_dead_acute, 'a', 'á'),
    (keysyms::KEY_dead_acute, 'e', 'é'),
    (keysyms::KEY_dead_acute, 'i', 'í'),
    (keysyms::KEY_dead_acute, 'o', 'ó'),
    (keysyms::KEY_dead_acute, 'u', 'ú'),
    (keysyms::KEY_dead_acute, 'y', 'ý'),
    (keysyms::KEY_dead_acute, 'C', 'Ć'),
    (keysyms::KEY_dead_acute, 'c', 'ć'),
    (keysyms::KEY_dead_acute, 'l', 'ĺ'),
    (keysyms::KEY_dead_acute, 'N', 'Ń'),
    (keysyms::KEY_dead_acute, 'n', 'ń'),
    (keysyms::KEY_dead_acute, 'r', 'ŕ'),
    (keysyms::KEY_dead_acute, 'S', 'Ś'),
    (keysyms::KEY_dead_acute, 's', 'ś'),
    (keysyms::KEY_dead_acute, 'Z', 'Ź'),
    (keysyms::KEY_dead_acute, 'z', 'ź'),
    (keysyms::KEY_dead_acute, 'Α', 'Ά'),
    (keysyms::KEY_dead_acute, 'Ε', 'Έ'),
    (keysyms::KEY_dead_acute, 'Η', 'Ή'),
    (keysyms::KEY_dead_acute, 'Ο', 'Ό'),
    (keysyms::KEY_dead_acute, 'α', 'ά'),
    (keysyms::KEY_dead_acute, 'ε', 'έ'),
    (keysyms::KEY_dead_acute, 'η', 'ή'),
    (keysyms::KEY_dead_acute, 'ι', 'ί'),
    (keysyms::KEY_dead_acute, 'ο', 'ό'),
    (keysyms::KEY_dead_acute, 'υ', 'ύ'),
    (keysyms::KEY_dead_acute, 'ω', 'ώ'),
    (keysyms::KEY_dead_circumflex, 'A', 'Â'),
    (keysyms::KEY_dead_circumflex, 'E', 'Ê'),
    (keysyms::KEY_dead_circumflex, 'I', 'Î'),
    (keysyms::KEY_dead_circumflex, 'O', 'Ô'),
    (keysyms::KEY_dead_circumflex, 'a', 'â'),
    (keysyms::KEY_dead_circumflex, 'e', 'ê'),
    (keysyms::KEY_dead_circumflex, 'i', 'î'),
    (keysyms::KEY_dead_circumflex, 'o', 'ô'),
    (keysyms::KEY_dead_circumflex, 'u', 'û'),
    (keysyms::KEY_dead_tilde, 'A', 'Ã'),
    (keysyms::KEY_dead_tilde, 'O', 'Õ'),
    (keysyms::KEY_dead_tilde, 'a', 'ã'),
    (keysyms::KEY_dead_tilde, 'n', 'ñ'),
    (keysyms::KEY_dead_tilde, 'o', 'õ'),
    (keysyms::KEY_dead_macron, 'A', 'Ā'),
    (keysyms::KEY_dead_macron, 'a', 'ā'),
    (keysyms::KEY_dead_macron, 'E', 'Ē'),
    (keysyms::KEY_dead_macron, 'e', 'ē'),
    (keysyms::KEY_dead_macron, 'I', 'Ī'),
    (keysyms::KEY_dead_macron, 'i', 'ī'),
    (keysyms::KEY_dead_macron, 'U', 'Ū'),
    (keysyms::KEY_dead_macron, 'u', 'ū'),
    (keysyms::KEY_dead_breve, 'A', 'Ă'),
    (keysyms::KEY_dead_breve, 'a', 'ă'),
    (keysyms::KEY_dead_breve, 'И', 'Й'),
    (keysyms::KEY_dead_breve, 'и', 'й'),
    (keysyms::KEY_dead_abovedot, 'C', 'Ċ'),
    (keysyms::KEY_dead_abovedot, 'c', 'ċ'),
    (keysyms::KEY_dead_abovedot, 'E', 'Ė'),
    (keysyms::KEY_dead_abovedot, 'e', 'ė'),
    (keysyms::KEY_dead_abovedot, 'G', 'Ġ'),
    (keysyms::KEY_dead_abovedot, 'g', 'ġ'),
    (keysyms::KEY_dead_abovedot, 'Z', 'Ż'),
    (keysyms::KEY_dead_abovedot, 'z', 'ż'),
    (keysyms::KEY_dead_diaeresis, 'A', 'Ä'),
    (keysyms::KEY_dead_diaeresis, 'O', 'Ö'),
    (keysyms::KEY_dead_diaeresis, 'U', 'Ü'),
    (keysyms::KEY_dead_diaeresis, 'a', 'ä'),
    (keysyms::KEY_dead_diaeresis, 'e', 'ë'),
    (keysyms::KEY_dead_diaeresis, 'i', 'ï'),
    (keysyms::KEY_dead_diaeresis, 'o', 'ö'),
    (keysyms::KEY_dead_diaeresis, 'u', 'ü'),
    (keysyms::KEY_dead_diaeresis, 'ι', 'ϊ'),
    (keysyms::KEY_dead_diaeresis, 'І', 'Ї'),
    (keysyms::KEY_dead_diaeresis, 'і', 'ї'),
    (keysyms::KEY_dead_abovering, 'A', 'Å'),
    (keysyms::KEY_dead_abovering, 'a', 'å'),
    (keysyms::KEY_dead_abovering, 'U', 'Ů'),
    (keysyms::KEY_dead_abovering, 'u', 'ů'),
    (keysyms::KEY_dead_doubleacute, 'O', 'Ő'),
    (keysyms::KEY_dead_doubleacute, 'o', 'ő'),
    (keysyms::KEY_dead_doubleacute, 'U', 'Ű'),
    (keysyms::KEY_dead_doubleacute, 'u', 'ű'),
    (keysyms::KEY_dead_caron, 'C', 'Č'),
    (keysyms::KEY_dead_caron, 'c', 'č'),
    (keysyms::KEY_dead_caron, 'D', 'Ď'),
    (keysyms::KEY_dead_caron, 'd', 'ď'),
    (keysyms::KEY_dead_caron, 'E', 'Ě'),
    (keysyms::KEY_dead_caron, 'e', 'ě'),
    (keysyms::KEY_dead_caron, 'L', 'Ľ'),
    (keysyms::KEY_dead_caron, 'l', 'ľ'),
    (keysyms::KEY_dead_caron, 'N', 'Ň'),
    (keysyms::KEY_dead_caron, 'n', 'ň'),
    (keysyms::KEY_dead_caron, 'R', 'Ř'),
    (keysyms::KEY_dead_caron, 'r', 'ř'),
    (keysyms::KEY_dead_caron, 'S', 'Š'),
    (keysyms::KEY_dead_caron, 's', 'š'),
    (keysyms::KEY_dead_caron, 'T', 'Ť'),
    (keysyms::KEY_dead_caron, 't', 'ť'),
    (keysyms::KEY_dead_caron, 'Z', 'Ž'),
    (keysyms::KEY_dead_caron, 'z', 'ž'),
    (keysyms::KEY_dead_cedilla, 'C', 'Ç'),
    (keysyms::KEY_dead_cedilla, 'c', 'ç'),
    (keysyms::KEY_dead_cedilla, 'G', 'Ģ'),
    (keysyms::KEY_dead_cedilla, 'g', 'ģ'),
    (keysyms::KEY_dead_cedilla, 'K', 'Ķ'),
    (keysyms::KEY_dead_cedilla, 'k', 'ķ'),
    (keysyms::KEY_dead_cedilla, 'L', 'Ļ'),
    (keysyms::KEY_dead_cedilla, 'l', 'ļ'),
    (keysyms::KEY_dead_cedilla, 'N', 'Ņ'),
    (keysyms::KEY_dead_cedilla, 'n', 'ņ'),
    (keysyms::KEY_dead_ogonek, 'A', 'Ą'),
    (keysyms::KEY_dead_ogonek, 'a', 'ą'),
    (keysyms::KEY_dead_ogonek, 'E', 'Ę'),
    (keysyms::KEY_dead_ogonek, 'e', 'ę'),
    (keysyms::KEY_dead_ogonek, 'I', 'Į'),
    (keysyms::KEY_dead_ogonek, 'i', 'į'),
    (keysyms::KEY_dead_ogonek, 'U', 'Ų'),
    (keysyms::KEY_dead_ogonek, 'u', 'ų'),
];

#[cfg(test)]
mod tests {
    use super::super::char_map::DEAD_KEYSYM_RANGE;
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn composed_chars_are_unique() {
        let mut seen: HashSet<char> = HashSet::new();
        for (_, _, composed) in DEAD_KEY_COMPOSITIONS {
            assert!(
                seen.insert(*composed),
                "composed char '{}' (U+{:04X}) is declared twice",
                composed,
                *composed as u32
            );
        }
    }

    #[test]
    fn every_dead_keysym_is_in_the_probed_range() {
        for (sym, _, _) in DEAD_KEY_COMPOSITIONS {
            assert!(
                DEAD_KEYSYM_RANGE.contains(sym),
                "keysym 0x{:04X} is outside the dead key range probed by char_map",
                sym
            );
        }
    }

    #[test]
    fn base_and_composed_chars_always_differ() {
        for (_, base, composed) in DEAD_KEY_COMPOSITIONS {
            assert_ne!(base, composed, "base '{}' composes to itself", base);
        }
    }
}
