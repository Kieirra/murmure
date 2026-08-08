//! Compile an XKB keymap and build a reverse `char -> CharStrokes`
//! table over everything the layout can produce: base, Shift, AltGr and
//! Shift+AltGr levels, plus dead key compositions.

use super::dead_keys::DEAD_KEY_COMPOSITIONS;
use super::types::{CharMap, CharStrokes, KeyMapping, LayoutInfo};
use std::collections::HashMap;
use xkbcommon::xkb;

// ISO_Level3_Shift is bound to the virtual Mod5 modifier by the evdev
// rule set; `MOD_NAME_*` has no constant for it.
const ALTGR_MOD_NAME: &str = "Mod5";

const KEY_ENTER_EVDEV: u16 = 28;
const KEY_TAB_EVDEV: u16 = 15;

pub(super) const DEAD_KEYSYM_RANGE: std::ops::RangeInclusive<u32> = 0xfe50..=0xfe93;

// Ordered from the simplest chord to the most complex so `or_insert`
// keeps the cheapest way to reach a char on a given key.
const MODIFIER_COMBOS: [(bool, bool); 4] =
    [(false, false), (true, false), (false, true), (true, true)];

pub fn build_char_map(info: &LayoutInfo) -> Result<CharMap, String> {
    let ctx = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    // Pass &str slices throughout so the `S: Borrow<str>` generic stays homogenous.
    let variant = info.variant.as_deref().unwrap_or("");
    let keymap = xkb::Keymap::new_from_names(
        &ctx,
        info.rules.as_str(),
        info.model.as_str(),
        info.layout.as_str(),
        variant,
        None,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .ok_or_else(|| format!("xkb_keymap_new_from_names returned null for {:?}", info))?;

    let mut state = xkb::State::new(&keymap);
    let shift_mod = keymap.mod_get_index(xkb::MOD_NAME_SHIFT);
    let altgr_mod = keymap.mod_get_index(ALTGR_MOD_NAME);
    let combos: &[(bool, bool)] = match altgr_mod == xkb::MOD_INVALID {
        true => &MODIFIER_COMBOS[..2],
        false => &MODIFIER_COMBOS,
    };

    let mut map: HashMap<char, CharStrokes> = HashMap::new();
    let mut dead_map: HashMap<u32, KeyMapping> = HashMap::new();

    let min = keymap.min_keycode().raw();
    let max = keymap.max_keycode().raw();
    for raw in min..=max {
        // evdev keycodes are XKB keycode - 8. Anything below 8 cannot be
        // synthesised through /dev/uinput, so we skip it. We also cap at
        // u16::MAX because input-linux's keycode type is u16.
        if raw < 8 {
            continue;
        }
        let Ok(evdev) = u16::try_from(raw - 8) else {
            continue;
        };
        let keycode = xkb::Keycode::new(raw);

        for &(needs_shift, needs_altgr) in combos {
            let mut mods = 0u32;
            if needs_shift {
                mods |= 1u32 << shift_mod;
            }
            if needs_altgr {
                mods |= 1u32 << altgr_mod;
            }
            state.update_mask(mods, 0, 0, 0, 0, 0);
            let sym = state.key_get_one_sym(keycode);
            let mapping = KeyMapping {
                evdev_keycode: evdev,
                needs_shift,
                needs_altgr,
            };

            if let Some(c) = char::from_u32(xkb::keysym_to_utf32(sym)).filter(|c| !c.is_control()) {
                map.entry(c).or_insert(CharStrokes::Direct(mapping));
            } else if DEAD_KEYSYM_RANGE.contains(&sym.raw()) {
                dead_map.entry(sym.raw()).or_insert(mapping);
            }
        }
    }

    insert_control_keys(&mut map);
    insert_compositions(&mut map, &dead_map);

    Ok(CharMap {
        layout: info.clone(),
        map,
        is_fallback: false,
        fallback_reason: None,
    })
}

// `keysym_to_utf32` maps Return to U+000D, so '\n' never comes out of the probing.
// It is wired to Shift+Enter, which adds a line where plain Enter would submit.
fn insert_control_keys(map: &mut HashMap<char, CharStrokes>) {
    for (c, evdev_keycode, needs_shift) in
        [('\n', KEY_ENTER_EVDEV, true), ('\t', KEY_TAB_EVDEV, false)]
    {
        map.insert(
            c,
            CharStrokes::Direct(KeyMapping {
                evdev_keycode,
                needs_shift,
                needs_altgr: false,
            }),
        );
    }
}

// A direct stroke always beats a composition, and a composition needs
// its base char to be directly reachable (a dead key cannot compose
// another dead key sequence).
fn insert_compositions(map: &mut HashMap<char, CharStrokes>, dead_map: &HashMap<u32, KeyMapping>) {
    for &(dead_sym, base_char, composed) in DEAD_KEY_COMPOSITIONS {
        if map.contains_key(&composed) {
            continue;
        }
        let Some(&dead) = dead_map.get(&dead_sym) else {
            continue;
        };
        let Some(CharStrokes::Direct(base)) = map.get(&base_char).copied() else {
            continue;
        };
        map.insert(composed, CharStrokes::DeadKey { dead, base });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Non-regression floor: the ASCII range Direct mode covered before dead keys.
    const ASCII_BASELINE: &str =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ,.!?'-:;()";

    fn fr_oss() -> LayoutInfo {
        LayoutInfo::new("fr".into(), Some("oss".into()))
    }

    fn direct(cm: &CharMap, c: char, label: &str) -> KeyMapping {
        match cm.map.get(&c) {
            Some(CharStrokes::Direct(m)) => *m,
            Some(CharStrokes::DeadKey { .. }) => {
                panic!("char '{}' must be direct in {} layout", c, label)
            }
            None => panic!("char '{}' must be mapped in {} layout", c, label),
        }
    }

    fn assert_all_mapped(cm: &CharMap, chars: &str, label: &str) {
        for c in chars.chars() {
            assert!(
                cm.map.contains_key(&c),
                "char '{}' (U+{:04X}) missing in {} layout map",
                c,
                c as u32,
                label
            );
        }
    }

    fn assert_all_dead_key(cm: &CharMap, chars: &str, label: &str) {
        for c in chars.chars() {
            match cm.map.get(&c) {
                Some(CharStrokes::DeadKey { .. }) => {}
                other => panic!(
                    "char '{}' must be a dead key sequence in {} layout, got {:?}",
                    c, label, other
                ),
            }
        }
    }

    #[test]
    fn build_char_map_us_layout_covers_baseline() {
        let info = LayoutInfo::us_fallback();
        let cm = build_char_map(&info).expect("US keymap must compile");
        assert_all_mapped(&cm, ASCII_BASELINE, "US");
    }

    #[test]
    fn build_char_map_fr_oss_covers_baseline() {
        let cm = build_char_map(&fr_oss()).expect("FR oss keymap must compile");
        assert_all_mapped(&cm, ASCII_BASELINE, "FR oss");
    }

    #[test]
    fn build_char_map_de_layout_covers_baseline() {
        let info = LayoutInfo::new("de".into(), None);
        let cm = build_char_map(&info).expect("DE keymap must compile");
        assert_all_mapped(&cm, ASCII_BASELINE, "DE");
    }

    #[test]
    fn build_char_map_us_lowercase_letters_no_shift() {
        let info = LayoutInfo::us_fallback();
        let cm = build_char_map(&info).unwrap();
        let a = direct(&cm, 'a', "US");
        assert!(!a.needs_shift, "US layout: 'a' must not require shift");
        let upper = direct(&cm, 'A', "US");
        assert!(upper.needs_shift, "US layout: 'A' must require shift");
    }

    #[test]
    fn digits_in_cz_layout_require_shift() {
        let info = LayoutInfo::new("cz".into(), None);
        let cm = build_char_map(&info).expect("CZ keymap must compile");
        let one = direct(&cm, '1', "CZ");
        assert!(one.needs_shift, "CZ layout: digit '1' must require shift");
    }

    #[test]
    fn newline_and_tab_map_to_their_control_keys() {
        let cm = build_char_map(&LayoutInfo::us_fallback()).unwrap();
        let enter = direct(&cm, '\n', "US");
        assert_eq!(enter.evdev_keycode, KEY_ENTER_EVDEV);
        assert!(
            enter.needs_shift,
            "'\\n' must be Shift+Enter so it does not submit"
        );
        assert!(!enter.needs_altgr);
        let tab = direct(&cm, '\t', "US");
        assert_eq!(tab.evdev_keycode, KEY_TAB_EVDEV);
        assert!(!tab.needs_shift && !tab.needs_altgr);
    }

    #[test]
    fn control_chars_are_never_mapped_to_a_key() {
        let cm = build_char_map(&LayoutInfo::us_fallback()).expect("US must compile");
        for c in ['\u{8}', '\u{1b}', '\u{7f}', '\r'] {
            assert!(
                !cm.map.contains_key(&c),
                "US layout must not map control char U+{:04X}, pressing it would be destructive",
                c as u32
            );
        }
        assert!(cm.map.contains_key(&'\n'), "'\\n' must stay mapped");
        assert!(cm.map.contains_key(&'\t'), "'\\t' must stay mapped");
    }

    #[test]
    fn fr_oss_typical_accents_are_direct_keys() {
        let cm = build_char_map(&fr_oss()).expect("FR oss keymap must compile");
        for c in "éèàçù".chars() {
            let m = direct(&cm, c, "FR oss");
            assert!(
                !m.needs_altgr,
                "FR oss: '{}' must be reachable without AltGr",
                c
            );
        }
    }

    // fr(oss) exposes the circumflex and diaeresis vowels on level 3,
    // so they resolve to AltGr chords rather than to the dead_circumflex
    // composition. Either way they are typed, never folded.
    #[test]
    fn fr_oss_circumflex_and_diaeresis_vowels_are_typable() {
        let cm = build_char_map(&fr_oss()).expect("FR oss keymap must compile");
        assert_all_mapped(&cm, "êîôûëü", "FR oss");
    }

    #[test]
    fn fr_oss_reaches_currency_and_chevrons_via_altgr() {
        let cm = build_char_map(&fr_oss()).expect("FR oss keymap must compile");
        for c in "€«»".chars() {
            let m = direct(&cm, c, "FR oss");
            assert!(m.needs_altgr, "FR oss: '{}' must need AltGr", c);
        }
    }

    #[test]
    fn de_layout_types_umlauts_and_eszett_directly() {
        let cm = build_char_map(&LayoutInfo::new("de".into(), None)).expect("DE must compile");
        for c in "äöüß".chars() {
            let m = direct(&cm, c, "DE");
            assert!(!m.needs_altgr, "DE: '{}' must be a plain key", c);
        }
    }

    #[test]
    fn es_layout_types_enye_directly_and_accents_via_dead_key() {
        let cm = build_char_map(&LayoutInfo::new("es".into(), None)).expect("ES must compile");
        direct(&cm, 'ñ', "ES");
        assert_all_dead_key(&cm, "áéíóú", "ES");
    }

    #[test]
    fn pl_layout_reaches_its_diacritics_via_altgr() {
        let cm = build_char_map(&LayoutInfo::new("pl".into(), None)).expect("PL must compile");
        for c in "ąćęłńóśźż".chars() {
            let m = direct(&cm, c, "PL");
            assert!(m.needs_altgr, "PL: '{}' must need AltGr", c);
        }
    }

    #[test]
    fn cz_layout_mixes_direct_diacritics_and_caron_dead_key() {
        let cm = build_char_map(&LayoutInfo::new("cz".into(), None)).expect("CZ must compile");
        assert_all_mapped(&cm, "ěščřžýáíéúů", "CZ");
        assert_all_dead_key(&cm, "ďťň", "CZ");
    }

    #[test]
    fn fi_layout_types_nordic_vowels_directly() {
        let cm = build_char_map(&LayoutInfo::new("fi".into(), None)).expect("FI must compile");
        for c in "åäö".chars() {
            let m = direct(&cm, c, "FI");
            assert!(!m.needs_altgr, "FI: '{}' must be a plain key", c);
        }
    }

    #[test]
    fn us_layout_cannot_type_accented_letters() {
        let cm = build_char_map(&LayoutInfo::us_fallback()).expect("US must compile");
        for c in "éàçß".chars() {
            assert!(
                !cm.map.contains_key(&c),
                "US layout must not map '{}', resolve_for_typing has to fold it",
                c
            );
        }
    }

    // Unknown layout: build_char_map must not panic so the caller can
    // retry with US fallback. libxkbcommon may return NULL or an empty
    // keymap depending on version, both are accepted.
    #[test]
    fn build_char_map_unknown_layout_does_not_panic() {
        let info = LayoutInfo::new("zz_definitely_not_a_real_layout".into(), None);
        match build_char_map(&info) {
            Ok(cm) => {
                assert!(
                    cm.map.len() <= 2,
                    "unknown layout must yield only the control keys (got {} entries)",
                    cm.map.len()
                );
            }
            Err(_) => {
                // `compile_with_fallback` catches it and retries with US.
            }
        }
    }
}
