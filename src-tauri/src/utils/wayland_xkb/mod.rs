pub mod char_map;
mod dead_keys;
pub mod layout_detect;
mod normalize;
pub mod types;
pub mod wayland_xkb;

pub use wayland_xkb::{
    current_fallback_payload, init_char_map, lookup, recompile, resolve_for_typing,
    LayoutFallbackPayload,
};
