//! Compile an XKB keymap and build a reverse `char -> KeyMapping`
//! table over the Parakeet subset. Only base + Shift levels are probed
//! since the subset is ASCII on every Latin layout we target.

use super::subset;
use super::types::{CharMap, KeyMapping, LayoutInfo};
use std::collections::HashMap;
use xkbcommon::xkb;

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

    let mut map: HashMap<char, KeyMapping> = HashMap::new();

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

        for needs_shift in [false, true] {
            let mods = if needs_shift { 1u32 << shift_mod } else { 0 };
            state.update_mask(mods, 0, 0, 0, 0, 0);
            let sym = state.key_get_one_sym(keycode);
            let utf32 = xkb::keysym_to_utf32(sym);
            if utf32 == 0 {
                continue;
            }
            let Some(c) = char::from_u32(utf32) else {
                continue;
            };
            if !subset::is_in_subset(c) {
                continue;
            }
            map.entry(c).or_insert(KeyMapping {
                evdev_keycode: evdev,
                needs_shift,
            });
        }
    }

    Ok(CharMap {
        layout: info.clone(),
        map,
        is_fallback: false,
        fallback_reason: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::wayland_xkb::subset;

    fn fr_oss() -> LayoutInfo {
        LayoutInfo::new("fr".into(), Some("oss".into()))
    }

    #[test]
    fn build_char_map_us_layout_covers_subset() {
        let info = LayoutInfo::us_fallback();
        let cm = build_char_map(&info).expect("US keymap must compile");
        for c in subset::PARAKEET_SUBSET {
            assert!(
                cm.map.contains_key(c),
                "char '{}' (U+{:04X}) missing in US layout map",
                c,
                *c as u32
            );
        }
    }

    #[test]
    fn build_char_map_fr_oss_covers_subset() {
        let info = fr_oss();
        let cm = build_char_map(&info).expect("FR oss keymap must compile");
        for c in subset::PARAKEET_SUBSET {
            assert!(
                cm.map.contains_key(c),
                "char '{}' (U+{:04X}) missing in FR oss layout map",
                c,
                *c as u32
            );
        }
    }

    #[test]
    fn build_char_map_us_lowercase_letters_no_shift() {
        let info = LayoutInfo::us_fallback();
        let cm = build_char_map(&info).unwrap();
        let a = cm.map.get(&'a').expect("'a' must be mapped");
        assert!(!a.needs_shift, "US layout: 'a' must not require shift");
        let upper = cm.map.get(&'A').expect("'A' must be mapped");
        assert!(upper.needs_shift, "US layout: 'A' must require shift");
    }

    #[test]
    fn digits_in_cz_layout_require_shift() {
        let info = LayoutInfo::new("cz".into(), None);
        let cm = build_char_map(&info).expect("CZ keymap must compile");
        let one = cm.map.get(&'1').expect("digit '1' must be mapped in CZ layout");
        assert!(one.needs_shift, "CZ layout: digit '1' must require shift");
    }

    #[test]
    fn build_char_map_de_layout_covers_subset() {
        // German PC105 has all the new punctuation reachable at level 0
        // or Shift; this guards the 6 extension chars on a non-FR/non-US
        // Latin layout.
        let info = LayoutInfo::new("de".into(), None);
        let cm = build_char_map(&info).expect("DE keymap must compile");
        for c in subset::PARAKEET_SUBSET {
            assert!(
                cm.map.contains_key(c),
                "char '{}' (U+{:04X}) missing in DE layout map",
                c,
                *c as u32
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
                // Empty keymap is the recovery path the fallback chain
                // already handles: every paste in Direct mode will pre-scan
                // unmapped and route to clipboard+Ctrl+V.
                assert!(
                    cm.map.is_empty(),
                    "unknown layout must yield an empty char map (got {} entries)",
                    cm.map.len()
                );
            }
            Err(_) => {
                // Err is the expected path on stricter libxkbcommon builds.
                // `compile_with_fallback` catches it and retries with US.
            }
        }
    }
}
