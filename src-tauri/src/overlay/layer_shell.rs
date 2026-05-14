use std::ffi::c_void;
use std::os::raw::c_int;
use std::sync::OnceLock;

use glib::translate::ToGlibPtr;
use libloading::{Library, Symbol};
use log::{debug, warn};

const LIB_SONAME: &str = "libgtk-layer-shell.so.0";

const LAYER_OVERLAY: c_int = 3;
const KEYBOARD_MODE_NONE: c_int = 0;

// Mirrors GtkLayerShellEdge from gtk-layer-shell.h (C enum, signed int).
#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum Edge {
    Left = 0,
    Right = 1,
    Top = 2,
    Bottom = 3,
}

type FnIsSupported = unsafe extern "C" fn() -> c_int;
type FnInitForWindow = unsafe extern "C" fn(*mut c_void);
type FnSetLayer = unsafe extern "C" fn(*mut c_void, c_int);
type FnSetKeyboardMode = unsafe extern "C" fn(*mut c_void, c_int);
type FnSetExclusiveZone = unsafe extern "C" fn(*mut c_void, c_int);
type FnSetAnchor = unsafe extern "C" fn(*mut c_void, c_int, c_int);
type FnSetMargin = unsafe extern "C" fn(*mut c_void, c_int, c_int);

struct LayerShellLib {
    _library: Library,
    is_supported: FnIsSupported,
    init_for_window: FnInitForWindow,
    set_layer: FnSetLayer,
    set_keyboard_mode: FnSetKeyboardMode,
    set_exclusive_zone: FnSetExclusiveZone,
    set_anchor: FnSetAnchor,
    set_margin: FnSetMargin,
}

static LIB: OnceLock<Option<LayerShellLib>> = OnceLock::new();

pub fn is_disabled_by_env() -> bool {
    match std::env::var("MURMURE_NO_GTK_LAYER_SHELL") {
        Ok(value) => {
            let v = value.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "yes"
        }
        Err(_) => false,
    }
}

// Safety: the caller must store the returned `T` in the same struct that
// owns `Library`, so the function pointer stays valid for the lifetime of
// that struct. `LIB` keeps the struct in static storage for 'static.
unsafe fn load_symbol<T: Copy>(library: &Library, name: &[u8]) -> Option<T> {
    match library.get::<T>(name) {
        Ok(sym) => {
            let raw: Symbol<T> = sym;
            Some(*raw)
        }
        Err(err) => {
            let pretty = String::from_utf8_lossy(name);
            warn!(
                "{} loaded but symbol `{}` missing: {}",
                LIB_SONAME, pretty, err
            );
            None
        }
    }
}

fn try_load() -> Option<LayerShellLib> {
    let library = match unsafe { Library::new(LIB_SONAME) } {
        Ok(lib) => lib,
        Err(err) => {
            debug!("{} not loadable: {}", LIB_SONAME, err);
            return None;
        }
    };

    unsafe {
        let is_supported = load_symbol::<FnIsSupported>(&library, b"gtk_layer_is_supported\0")?;
        let init_for_window =
            load_symbol::<FnInitForWindow>(&library, b"gtk_layer_init_for_window\0")?;
        let set_layer = load_symbol::<FnSetLayer>(&library, b"gtk_layer_set_layer\0")?;
        let set_keyboard_mode =
            load_symbol::<FnSetKeyboardMode>(&library, b"gtk_layer_set_keyboard_mode\0")?;
        let set_exclusive_zone =
            load_symbol::<FnSetExclusiveZone>(&library, b"gtk_layer_set_exclusive_zone\0")?;
        let set_anchor = load_symbol::<FnSetAnchor>(&library, b"gtk_layer_set_anchor\0")?;
        let set_margin = load_symbol::<FnSetMargin>(&library, b"gtk_layer_set_margin\0")?;

        Some(LayerShellLib {
            _library: library,
            is_supported,
            init_for_window,
            set_layer,
            set_keyboard_mode,
            set_exclusive_zone,
            set_anchor,
            set_margin,
        })
    }
}

fn lib() -> Option<&'static LayerShellLib> {
    if is_disabled_by_env() {
        return None;
    }
    LIB.get_or_init(try_load).as_ref()
}

pub fn is_supported() -> bool {
    let Some(lib) = lib() else {
        return false;
    };
    let supported = unsafe { (lib.is_supported)() } != 0;
    if !supported {
        debug!(
            "{} loaded but compositor does not support wlr-layer-shell (likely GNOME/Mutter), using Tauri native overlay",
            LIB_SONAME
        );
    }
    supported
}

// Runs `op` with a raw `GtkWindow*` derived from the Tauri window.
// The `gtk::ApplicationWindow` and its glib `Stash` stay alive for the
// duration of the closure, which is all the C functions need.
fn with_gtk_window_ptr<R>(
    overlay_window: &tauri::WebviewWindow,
    op: impl FnOnce(*mut c_void) -> R,
) -> Option<R> {
    let app_window = match overlay_window.gtk_window() {
        Ok(w) => w,
        Err(e) => {
            warn!("Could not retrieve GTK window for layer-shell call: {}", e);
            return None;
        }
    };
    let stash = app_window.to_glib_none();
    let ptr: *mut gtk::ffi::GtkApplicationWindow = stash.0;
    if ptr.is_null() {
        return None;
    }
    Some(op(ptr as *mut c_void))
}

pub fn init_for_window(overlay_window: &tauri::WebviewWindow) -> bool {
    let Some(lib) = lib() else {
        return false;
    };
    with_gtk_window_ptr(overlay_window, |ptr| unsafe {
        (lib.init_for_window)(ptr);
        (lib.set_layer)(ptr, LAYER_OVERLAY);
        (lib.set_keyboard_mode)(ptr, KEYBOARD_MODE_NONE);
        (lib.set_exclusive_zone)(ptr, 0);
    })
    .is_some()
}

pub fn set_anchor(overlay_window: &tauri::WebviewWindow, edge: Edge, anchored: bool) {
    let Some(lib) = lib() else {
        return;
    };
    let _ = with_gtk_window_ptr(overlay_window, |ptr| unsafe {
        (lib.set_anchor)(ptr, edge as c_int, anchored as c_int);
    });
}

pub fn set_margin(overlay_window: &tauri::WebviewWindow, edge: Edge, margin_px: i32) {
    let Some(lib) = lib() else {
        return;
    };
    let _ = with_gtk_window_ptr(overlay_window, |ptr| unsafe {
        (lib.set_margin)(ptr, edge as c_int, margin_px as c_int);
    });
}
