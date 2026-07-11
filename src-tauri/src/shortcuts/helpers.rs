/// Physical scan-code → key name for OEM (position-dependent) keys.
#[cfg(target_os = "windows")]
fn scan_to_oem_name(scan: u32) -> Option<&'static str> {
    match scan {
        0x29 => Some("backquote"),
        0x56 => Some("intlbackslash"),
        0x0C => Some("minus"),
        0x0D => Some("equal"),
        0x1A => Some("bracketleft"),
        0x1B => Some("bracketright"),
        0x27 => Some("semicolon"),
        0x28 => Some("quote"),
        0x33 => Some("comma"),
        0x34 => Some("period"),
        0x35 => Some("slash"),
        0x2B => Some("backslash"),
        _ => None,
    }
}

/// Resolve an OEM key to its virtual-key code.
/// On Windows, OEM VK codes are keyboard-layout-dependent (e.g. the ² key on
/// French AZERTY is VK 0xDE, not 0xC0 as on US QWERTY). We resolve at runtime
/// via the physical scan code so the correct key is always detected.
/// On other platforms the hardcoded fallback is used (Linux/macOS map to the
/// same fixed VK values).
fn oem_vk(scan_code: u32, fallback: i32) -> Option<i32> {
    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::UI::Input::KeyboardAndMouse::MapVirtualKeyW;
        const MAPVK_VSC_TO_VK: u32 = 1;
        let vk = unsafe { MapVirtualKeyW(scan_code, MAPVK_VSC_TO_VK) };
        if vk != 0 {
            return Some(vk as i32);
        }
    }
    let _ = scan_code;
    Some(fallback)
}

fn key_name_to_vk(name: &str) -> Option<i32> {
    match name.trim().to_lowercase().as_str() {
        "win" | "meta" | "super" | "command" | "cmd" => Some(0x5B),
        "ctrl" | "control" => Some(0x11),
        "alt" | "menu" => Some(0x12),
        "shift" => Some(0x10),
        "a" => Some(0x41),
        "b" => Some(0x42),
        "c" => Some(0x43),
        "d" => Some(0x44),
        "e" => Some(0x45),
        "f" => Some(0x46),
        "g" => Some(0x47),
        "h" => Some(0x48),
        "i" => Some(0x49),
        "j" => Some(0x4A),
        "k" => Some(0x4B),
        "l" => Some(0x4C),
        "m" => Some(0x4D),
        "n" => Some(0x4E),
        "o" => Some(0x4F),
        "p" => Some(0x50),
        "q" => Some(0x51),
        "r" => Some(0x52),
        "s" => Some(0x53),
        "t" => Some(0x54),
        "u" => Some(0x55),
        "v" => Some(0x56),
        "w" => Some(0x57),
        "x" => Some(0x58),
        "y" => Some(0x59),
        "z" => Some(0x5A),
        "0" => Some(0x30),
        "1" => Some(0x31),
        "2" => Some(0x32),
        "3" => Some(0x33),
        "4" => Some(0x34),
        "5" => Some(0x35),
        "6" => Some(0x36),
        "7" => Some(0x37),
        "8" => Some(0x38),
        "9" => Some(0x39),
        "f1" => Some(0x70),
        "f2" => Some(0x71),
        "f3" => Some(0x72),
        "f4" => Some(0x73),
        "f5" => Some(0x74),
        "f6" => Some(0x75),
        "f7" => Some(0x76),
        "f8" => Some(0x77),
        "f9" => Some(0x78),
        "f10" => Some(0x79),
        "f11" => Some(0x7A),
        "f12" => Some(0x7B),
        // F13-F20
        "f13" => Some(0x7C),
        "f14" => Some(0x7D),
        "f15" => Some(0x7E),
        "f16" => Some(0x7F),
        "f17" => Some(0x80),
        "f18" => Some(0x81),
        "f19" => Some(0x82),
        "f20" => Some(0x83),
        // Numpad
        "kp0" => Some(0x60),
        "kp1" => Some(0x61),
        "kp2" => Some(0x62),
        "kp3" => Some(0x63),
        "kp4" => Some(0x64),
        "kp5" => Some(0x65),
        "kp6" => Some(0x66),
        "kp7" => Some(0x67),
        "kp8" => Some(0x68),
        "kp9" => Some(0x69),
        "kpmultiply" => Some(0x6A),
        "kpplus" => Some(0x6B),
        "kpminus" => Some(0x6D),
        "kpdivide" => Some(0x6F),
        // Special keys (physical position, cross-platform reliable)
        "backquote" | "`" | "²" => oem_vk(0x29, 0xC0),
        "intlbackslash" | "<" | ">" => oem_vk(0x56, 0xE2),
        "space" => Some(0x20),
        "enter" | "return" => Some(0x0D),
        "escape" | "esc" => Some(0x1B),
        "tab" => Some(0x09),
        "backspace" => Some(0x08),
        "delete" | "del" => Some(0x2E),
        "insert" | "ins" => Some(0x2D),
        "home" => Some(0x24),
        "end" => Some(0x23),
        "pageup" => Some(0x21),
        "pagedown" => Some(0x22),
        "arrowup" | "up" => Some(0x26),
        "arrowdown" | "down" => Some(0x28),
        "arrowleft" | "left" => Some(0x25),
        "arrowright" | "right" => Some(0x27),
        // Lock/pause keys (Windows + Linux; no macOS keycode)
        "pause" => Some(0x13),
        "scrolllock" => Some(0x91),
        // OEM keys (layout-dependent VK on Windows, resolved via scan code)
        "minus" | "-" => oem_vk(0x0C, 0xBD),
        "equal" | "=" => oem_vk(0x0D, 0xBB),
        "bracketleft" | "[" => oem_vk(0x1A, 0xDB),
        "bracketright" | "]" => oem_vk(0x1B, 0xDD),
        "semicolon" | ";" => oem_vk(0x27, 0xBA),
        "quote" | "'" => oem_vk(0x28, 0xDE),
        "comma" | "," => oem_vk(0x33, 0xBC),
        "period" | "." => oem_vk(0x34, 0xBE),
        "slash" | "/" => oem_vk(0x35, 0xBF),
        "backslash" | "\\" => oem_vk(0x2B, 0xDC),
        "mousebutton1" => Some(0x01), // VK_LBUTTON
        "mousebutton2" => Some(0x02), // VK_RBUTTON
        "mousebutton3" => Some(0x04), // VK_MBUTTON
        "mousebutton4" => Some(0x05), // VK_XBUTTON1 (Back)
        "mousebutton5" => Some(0x06), // VK_XBUTTON2 (Forward)
        _ => None,
    }
}

fn vk_to_key_name(vk: i32) -> String {
    // On Windows, OEM VK codes are keyboard-layout-dependent; resolve via scan code
    // to always return the correct physical key name (e.g. VK 0xDE → "backquote"
    // on French AZERTY, not "quote" as it would be on US QWERTY).
    #[cfg(target_os = "windows")]
    {
        if matches!(vk, 0xBA..=0xC0 | 0xDB..=0xDE | 0xE2) {
            use windows_sys::Win32::UI::Input::KeyboardAndMouse::MapVirtualKeyW;
            const MAPVK_VK_TO_VSC: u32 = 0;
            let scan = unsafe { MapVirtualKeyW(vk as u32, MAPVK_VK_TO_VSC) };
            if let Some(name) = scan_to_oem_name(scan) {
                return name.to_string();
            }
        }
    }

    match vk {
        0x5B => "win".to_string(),
        0x11 => "ctrl".to_string(),
        0x12 => "alt".to_string(),
        0x10 => "shift".to_string(),
        0x41..=0x5A => {
            let offset = (vk - 0x41) as u8;
            ((b'a' + offset) as char).to_string()
        }
        0x30..=0x39 => {
            let offset = (vk - 0x30) as u8;
            ((b'0' + offset) as char).to_string()
        }
        0x70..=0x83 => format!("f{}", vk - 0x70 + 1),
        // Numpad
        0x60..=0x69 => format!("kp{}", vk - 0x60),
        0x6A => "kpmultiply".to_string(),
        0x6B => "kpplus".to_string(),
        0x6D => "kpminus".to_string(),
        0x6F => "kpdivide".to_string(),
        // Special keys
        0xC0 => "backquote".to_string(),
        0xE2 => "intlbackslash".to_string(),
        0x20 => "space".to_string(),
        0x0D => "enter".to_string(),
        0x1B => "escape".to_string(),
        0x09 => "tab".to_string(),
        0x08 => "backspace".to_string(),
        0x2E => "delete".to_string(),
        0x2D => "insert".to_string(),
        0x24 => "home".to_string(),
        0x23 => "end".to_string(),
        0x21 => "pageup".to_string(),
        0x22 => "pagedown".to_string(),
        0x26 => "arrowup".to_string(),
        0x28 => "arrowdown".to_string(),
        0x25 => "arrowleft".to_string(),
        0x27 => "arrowright".to_string(),
        0x13 => "pause".to_string(),
        0x91 => "scrolllock".to_string(),
        // OEM keys
        0xBD => "minus".to_string(),
        0xBB => "equal".to_string(),
        0xDB => "bracketleft".to_string(),
        0xDD => "bracketright".to_string(),
        0xBA => "semicolon".to_string(),
        0xDE => "quote".to_string(),
        0xBC => "comma".to_string(),
        0xBE => "period".to_string(),
        0xBF => "slash".to_string(),
        0xDC => "backslash".to_string(),
        0x01 => "mousebutton1".to_string(),
        0x02 => "mousebutton2".to_string(),
        0x04 => "mousebutton3".to_string(),
        0x05 => "mousebutton4".to_string(),
        0x06 => "mousebutton5".to_string(),
        _ => format!("key{}", vk),
    }
}

pub fn parse_binding_keys(binding: &str) -> Vec<i32> {
    let mut keys = Vec::new();
    for token in binding.split('+') {
        if let Some(vk) = key_name_to_vk(token) {
            if !keys.contains(&vk) {
                keys.push(vk);
            }
        }
    }
    keys
}

pub fn keys_to_string(keys: &[i32]) -> String {
    keys.iter()
        .map(|vk| vk_to_key_name(*vk))
        .collect::<Vec<_>>()
        .join("+")
}
