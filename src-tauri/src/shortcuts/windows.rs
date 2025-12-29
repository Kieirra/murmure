//! Windows keyboard shortcuts - uses rdev with WH_KEYBOARD_LL hook.
//! This approach captures local VK events from devices like Philips SpeechMike.
//! Re-exports from the common rdev module.

pub use super::rdev_common::init_shortcuts;
