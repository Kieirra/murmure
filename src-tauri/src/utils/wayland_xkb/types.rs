use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutInfo {
    pub rules: String,
    pub model: String,
    pub layout: String,
    pub variant: Option<String>,
}

impl LayoutInfo {
    pub fn new(layout: String, variant: Option<String>) -> Self {
        Self {
            rules: "evdev".to_string(),
            model: "pc105".to_string(),
            layout,
            variant,
        }
    }

    pub fn us_fallback() -> Self {
        Self::new("us".to_string(), None)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KeyMapping {
    pub evdev_keycode: u16,
    pub needs_shift: bool,
    // ISO_Level3_Shift, injected as KEY_RIGHTALT.
    pub needs_altgr: bool,
}

// `DeadKey` leaves the composition to the receiving application's
// toolkit, exactly like a physical keyboard would.
#[derive(Debug, Clone, Copy)]
pub enum CharStrokes {
    Direct(KeyMapping),
    DeadKey { dead: KeyMapping, base: KeyMapping },
}

pub struct CharMap {
    pub layout: LayoutInfo,
    pub map: HashMap<char, CharStrokes>,
    pub is_fallback: bool,
    pub fallback_reason: Option<&'static str>,
}
