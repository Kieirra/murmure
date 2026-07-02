#[derive(serde::Deserialize, Clone, Copy)]
pub struct InputRect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn point_in_rects(rects: &[InputRect], x: i32, y: i32) -> bool {
    rects
        .iter()
        .any(|r| x >= r.x && x < r.x + r.width as i32 && y >= r.y && y < r.y + r.height as i32)
}

#[cfg(target_os = "linux")]
pub use linux::apply_input_region;

#[cfg(target_os = "windows")]
pub use windows::apply_input_region;

#[cfg(target_os = "macos")]
pub use macos::apply_input_region;

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn apply_input_region(_window: &tauri::WebviewWindow, _rects: &[InputRect]) {}

#[cfg(target_os = "windows")]
pub use windows::on_overlay_shown;

// macOS: never touch ignoresMouseEvents. Explicitly setting it (even to
// false) disables the window server's built-in click-through on fully
// transparent pixels, which is what lets clicks reach the app behind.
#[cfg(target_os = "macos")]
pub fn on_overlay_shown(_window: &tauri::WebviewWindow) {}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn on_overlay_shown(window: &tauri::WebviewWindow) {
    let _ = window.set_ignore_cursor_events(false);
}

#[cfg(target_os = "linux")]
mod linux {
    use super::InputRect;
    use gtk::cairo;
    use gtk::prelude::WidgetExt;
    use log::{debug, warn};
    use tauri::WebviewWindow;

    pub fn apply_input_region(window: &WebviewWindow, rects: &[InputRect]) {
        let gtk_window = match window.gtk_window() {
            Ok(w) => w,
            Err(e) => {
                debug!("input region: no GTK window yet: {}", e);
                return;
            }
        };

        let Some(gdk_window) = gtk_window.window() else {
            debug!("input region: GTK window not realised yet, skipping");
            return;
        };
        if gdk_window.is_destroyed() {
            debug!("input region: GDK window destroyed, skipping");
            return;
        }

        let scale = gdk_window.scale_factor().max(1);

        let region = cairo::Region::create();
        for r in rects {
            let rect = cairo::RectangleInt::new(
                r.x / scale,
                r.y / scale,
                (r.width as i32 + scale - 1) / scale,
                (r.height as i32 + scale - 1) / scale,
            );
            if let Err(e) = region.union_rectangle(&rect) {
                warn!("input region: failed to union rect: {}", e);
                return;
            }
        }

        gdk_window.input_shape_combine_region(&region, 0, 0);
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use super::InputRect;
    use std::sync::{Mutex, OnceLock};
    use std::thread::{self, Thread};
    use std::time::Duration;
    use tauri::{AppHandle, Manager, WebviewWindow};
    use windows_sys::Win32::Foundation::{HWND, POINT};
    use windows_sys::Win32::Graphics::Gdi::ScreenToClient;
    use windows_sys::Win32::UI::WindowsAndMessaging::{GetCursorPos, IsWindowVisible};

    const OVERLAY_LABEL: &str = "recording_overlay";

    fn rects() -> &'static Mutex<Vec<InputRect>> {
        static RECTS: OnceLock<Mutex<Vec<InputRect>>> = OnceLock::new();
        RECTS.get_or_init(|| Mutex::new(Vec::new()))
    }

    pub fn apply_input_region(_window: &WebviewWindow, regions: &[InputRect]) {
        if let Ok(mut guard) = rects().lock() {
            *guard = regions.to_vec();
        }
    }

    pub fn on_overlay_shown(window: &WebviewWindow) {
        let _ = window.set_ignore_cursor_events(true);
        tracker(window.app_handle().clone()).unpark();
    }

    fn tracker(app: AppHandle) -> &'static Thread {
        static TRACKER: OnceLock<Thread> = OnceLock::new();
        TRACKER.get_or_init(|| thread::spawn(move || run(app)).thread().clone())
    }

    fn run(app: AppHandle) {
        let mut applied: Option<bool> = None;
        loop {
            thread::sleep(Duration::from_millis(32));

            let Some(window) = app.get_webview_window(OVERLAY_LABEL) else {
                thread::park();
                continue;
            };
            let Ok(handle) = window.hwnd() else {
                thread::park();
                continue;
            };
            let hwnd = handle.0 as HWND;
            if unsafe { IsWindowVisible(hwnd) } == 0 {
                applied = None;
                thread::park();
                continue;
            }

            let mut point = POINT { x: 0, y: 0 };
            if unsafe { GetCursorPos(&mut point) } == 0 {
                continue;
            }
            unsafe { ScreenToClient(hwnd, &mut point) };

            let inside = rects()
                .lock()
                .map(|guard| super::point_in_rects(&guard, point.x, point.y))
                .unwrap_or(false);

            if applied != Some(inside) {
                applied = Some(inside);
                let _ = app.run_on_main_thread(move || {
                    let _ = window.set_ignore_cursor_events(!inside);
                });
            }
        }
    }
}

#[cfg(target_os = "macos")]
mod macos {
    use super::InputRect;
    use log::{debug, warn};
    use objc2::runtime::{AnyClass, AnyObject, ClassBuilder, Sel};
    use objc2::{msg_send, sel};
    use objc2_foundation::{NSPoint, NSRect};
    use std::sync::Mutex;
    use std::sync::OnceLock;
    use tauri::WebviewWindow;

    static RECTS: OnceLock<Mutex<Vec<InputRect>>> = OnceLock::new();
    static CLASS: OnceLock<usize> = OnceLock::new();

    fn rects() -> &'static Mutex<Vec<InputRect>> {
        RECTS.get_or_init(|| Mutex::new(Vec::new()))
    }

    extern "C" fn hit_test(this: &AnyObject, _cmd: Sel, point: NSPoint) -> *mut AnyObject {
        // hitTest: receives the point in the superview's coordinate system;
        // convert through AppKit so flipped views resolve correctly, and read
        // bounds/scale live so window resizes and monitor moves stay accurate.
        let inside = unsafe {
            let superview: *mut AnyObject = msg_send![this, superview];
            let local: NSPoint = if superview.is_null() {
                point
            } else {
                msg_send![this, convertPoint: point, fromView: superview]
            };
            let flipped: bool = msg_send![this, isFlipped];
            let bounds: NSRect = msg_send![this, bounds];
            let y_pt = if flipped {
                local.y
            } else {
                bounds.size.height - local.y
            };
            let ns_window: *mut AnyObject = msg_send![this, window];
            let scale: f64 = if ns_window.is_null() {
                1.0
            } else {
                msg_send![ns_window, backingScaleFactor]
            };
            let px = (local.x * scale) as i32;
            let py = (y_pt * scale) as i32;
            rects()
                .lock()
                .map(|guard| super::point_in_rects(&guard, px, py))
                .unwrap_or(false)
        };
        if !inside {
            return std::ptr::null_mut();
        }
        let subclass = match CLASS.get() {
            Some(&ptr) if ptr != 0 => unsafe { &*(ptr as *const AnyClass) },
            _ => return std::ptr::null_mut(),
        };
        // objc_msgSendSuper starts the lookup in the class it is given; pass
        // the superclass, passing `subclass` would re-enter this override.
        let Some(superclass) = subclass.superclass() else {
            return std::ptr::null_mut();
        };
        unsafe { msg_send![super(this, superclass), hitTest: point] }
    }

    fn ensure_subclass(base: &AnyClass) -> Option<&'static AnyClass> {
        let raw = CLASS.get_or_init(|| {
            let Some(mut builder) = ClassBuilder::new(c"MurmureOverlayHitView", base) else {
                warn!("input region: could not declare overlay hit-test subclass");
                return 0;
            };
            unsafe {
                builder.add_method(sel!(hitTest:), hit_test as extern "C" fn(_, _, _) -> _);
            }
            builder.register() as *const AnyClass as usize
        });
        match *raw {
            0 => None,
            ptr => Some(unsafe { &*(ptr as *const AnyClass) }),
        }
    }

    pub fn apply_input_region(window: &WebviewWindow, regions: &[InputRect]) {
        let ns_view = match window.ns_view() {
            Ok(v) if !v.is_null() => v as *mut AnyObject,
            _ => {
                debug!("input region: no ns_view yet, skipping");
                return;
            }
        };

        match rects().lock() {
            Ok(mut guard) => *guard = regions.to_vec(),
            Err(_) => {
                warn!("input region: rects lock poisoned, skipping");
                return;
            }
        }

        unsafe {
            let view = &*ns_view;
            let base = view.class();
            let Some(hit_class) = ensure_subclass(base) else {
                return;
            };
            if !std::ptr::eq(base, hit_class) {
                AnyObject::set_class(view, hit_class);
            }
        }
        debug!(
            "input region: ns_view hit-test installed, {} rect(s)",
            regions.len()
        );
    }
}
