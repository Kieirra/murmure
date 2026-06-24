// Per-platform native input region for the recording overlay.
//
// The overlay is a transparent always-on-top window. We want clicks to land
// only on its opaque widgets (cancel cross, visualizer, streaming text box)
// and to pass through to whatever sits behind it everywhere else. A global
// `set_ignore_cursor_events` toggle cannot express "this pixel yes, that one
// no", so each platform gets its native input-region primitive instead.
//
// `set_ignore_cursor_events(false)` must stay in effect (the window keeps
// capturing) so the native region we install below is the thing that decides
// pass-through per pixel. Toggling ignore-cursor-events on would make the
// whole window transparent to input and override the region.
//
// Rects are physical pixels, origin top-left of the webview. An empty slice
// means "nothing clickable" (everything passes through).

#[derive(serde::Deserialize, Clone, Copy)]
pub struct InputRect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[cfg(target_os = "linux")]
pub use linux::apply_input_region;

#[cfg(target_os = "windows")]
pub use windows::apply_input_region;

#[cfg(target_os = "macos")]
pub use macos::apply_input_region;

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn apply_input_region(_window: &tauri::WebviewWindow, _rects: &[InputRect]) {}

#[cfg(target_os = "linux")]
mod linux {
    use super::InputRect;
    use gtk::cairo;
    use gtk::prelude::WidgetExt;
    use log::{debug, warn};
    use tauri::WebviewWindow;

    // Must run on the GTK main thread (callers dispatch via run_on_main_thread).
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

        // gdk_window_input_shape_combine_region works on both backends:
        // XShape on X11, wl_surface.set_input_region on Wayland.
        //
        // The shape is interpreted in the GdkWindow coordinate space, which
        // GDK keeps device-independent: on a scale-factor-N surface a physical
        // pixel maps to 1/N window units, so divide our physical rects by it.
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
        debug!(
            "input region applied: {} rect(s), scale {}",
            rects.len(),
            scale
        );
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use super::InputRect;
    use log::{debug, warn};
    use std::sync::Mutex;
    use std::sync::OnceLock;
    use tauri::WebviewWindow;
    use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CallWindowProcW, DefWindowProcW, GetWindowLongPtrW, SetWindowLongPtrW, GWLP_WNDPROC,
        HTCLIENT, HTTRANSPARENT, WM_NCDESTROY, WM_NCHITTEST,
    };

    // SetWindowRgn is intentionally avoided: it clips the visual render and
    // breaks the per-pixel-alpha transparent overlay. Instead we subclass the
    // window proc and answer WM_NCHITTEST with HTTRANSPARENT outside the union
    // of rects, so clicks fall through to the window behind. WebView2 hosts
    // its own composition child windows, but WM_NCHITTEST on the top-level is
    // honoured for pass-through routing by the OS hit-test walk.

    struct HitState {
        original_proc: isize,
        // Physical pixels, origin top-left of the window client area.
        rects: Vec<InputRect>,
    }

    // Single overlay window, so one global slot is enough. Keyed by HWND to
    // ignore stray messages if the handle is ever recycled.
    static STATE: OnceLock<Mutex<Option<(isize, HitState)>>> = OnceLock::new();

    fn state() -> &'static Mutex<Option<(isize, HitState)>> {
        STATE.get_or_init(|| Mutex::new(None))
    }

    pub fn apply_input_region(window: &WebviewWindow, rects: &[InputRect]) {
        let hwnd = match window.hwnd() {
            Ok(h) => h.0 as HWND,
            Err(e) => {
                debug!("input region: no HWND yet: {}", e);
                return;
            }
        };

        let mut guard = match state().lock() {
            Ok(g) => g,
            Err(_) => {
                warn!("input region: state lock poisoned, skipping");
                return;
            }
        };

        match guard.as_mut() {
            Some((stored, st)) if *stored == hwnd as isize => {
                st.rects = rects.to_vec();
                return;
            }
            _ => {}
        }

        // First install on this HWND: subclass and remember the original proc.
        // Safety: hwnd is a live top-level window owned by Tauri. We swap its
        // WndProc for our own and chain to the original via CallWindowProcW.
        let original_proc = unsafe { GetWindowLongPtrW(hwnd, GWLP_WNDPROC) };
        if original_proc == 0 {
            warn!("input region: GetWindowLongPtrW failed, leaving window fully clickable");
            return;
        }
        unsafe {
            SetWindowLongPtrW(hwnd, GWLP_WNDPROC, hit_test_proc as usize as isize);
        }
        *guard = Some((
            hwnd as isize,
            HitState {
                original_proc,
                rects: rects.to_vec(),
            },
        ));
        debug!("input region: WndProc subclassed for overlay hit-testing");
    }

    fn point_in_rects(rects: &[InputRect], x: i32, y: i32) -> bool {
        rects
            .iter()
            .any(|r| x >= r.x && x < r.x + r.width as i32 && y >= r.y && y < r.y + r.height as i32)
    }

    // Safety: matches the WNDPROC ABI. Invoked by the OS for the subclassed
    // window only. We read the shared state behind a Mutex and always chain to
    // the original proc for messages we do not special-case.
    unsafe extern "system" fn hit_test_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        let original = {
            let guard = match state().lock() {
                Ok(g) => g,
                Err(_) => return DefWindowProcW(hwnd, msg, wparam, lparam),
            };
            match guard.as_ref() {
                Some((stored, st)) if *stored == hwnd as isize => {
                    if msg == WM_NCHITTEST {
                        // LPARAM packs screen coords; convert to client space.
                        let screen_x = (lparam & 0xFFFF) as i16 as i32;
                        let screen_y = ((lparam >> 16) & 0xFFFF) as i16 as i32;
                        let mut pt = windows_sys::Win32::Foundation::POINT {
                            x: screen_x,
                            y: screen_y,
                        };
                        windows_sys::Win32::Graphics::Gdi::ScreenToClient(hwnd, &mut pt);
                        if point_in_rects(&st.rects, pt.x, pt.y) {
                            return HTCLIENT as LRESULT;
                        }
                        return HTTRANSPARENT as LRESULT;
                    }
                    st.original_proc
                }
                _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
            }
        };

        if msg == WM_NCDESTROY {
            if let Ok(mut guard) = state().lock() {
                if matches!(guard.as_ref(), Some((stored, _)) if *stored == hwnd as isize) {
                    *guard = None;
                }
            }
        }

        CallWindowProcW(
            Some(std::mem::transmute::<
                isize,
                unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT,
            >(original)),
            hwnd,
            msg,
            wparam,
            lparam,
        )
    }
}

#[cfg(target_os = "macos")]
mod macos {
    use super::InputRect;
    use log::{debug, warn};
    use objc2::runtime::{AnyClass, AnyObject, ClassBuilder, Sel};
    use objc2::{msg_send, sel};
    use std::sync::Mutex;
    use std::sync::OnceLock;
    use tauri::WebviewWindow;

    // macOS works in points with a bottom-left origin; our rects are physical
    // pixels with a top-left origin. hitTest: receives the point in the
    // superview space, so we flip Y against the view height and scale to px.
    //
    // We swap the content NSView's class for a subclass whose hitTest: returns
    // nil outside the union of rects, making AppKit route those clicks to the
    // window behind.

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct NSPoint {
        x: f64,
        y: f64,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct NSSize {
        width: f64,
        height: f64,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct NSRect {
        origin: NSPoint,
        size: NSSize,
    }

    struct HitState {
        rects: Vec<InputRect>,
        scale: f64,
        view_height_pt: f64,
    }

    static STATE: OnceLock<Mutex<Option<HitState>>> = OnceLock::new();
    // Stores the registered subclass pointer as usize; 0 means registration
    // failed and we should leave the view untouched (fully clickable).
    static CLASS: OnceLock<usize> = OnceLock::new();

    fn state() -> &'static Mutex<Option<HitState>> {
        STATE.get_or_init(|| Mutex::new(None))
    }

    fn point_in_rects(rects: &[InputRect], x: i32, y: i32) -> bool {
        rects
            .iter()
            .any(|r| x >= r.x && x < r.x + r.width as i32 && y >= r.y && y < r.y + r.height as i32)
    }

    // Safety: ABI of `- (NSView *)hitTest:(NSPoint)`. Reads the shared region;
    // outside it returns nil so AppKit routes the click to the window behind.
    // Inside, it forwards to the original view class via the dynamic super.
    extern "C" fn hit_test(this: &AnyObject, _cmd: Sel, point: NSPoint) -> *mut AnyObject {
        let inside = match state().lock() {
            Ok(guard) => guard.as_ref().is_some_and(|st| {
                let px = (point.x * st.scale) as i32;
                let py = ((st.view_height_pt - point.y) * st.scale) as i32;
                point_in_rects(&st.rects, px, py)
            }),
            Err(_) => false,
        };
        if !inside {
            return std::ptr::null_mut();
        }
        // super of our subclass is the original WKWebView content class.
        let subclass = match CLASS.get() {
            Some(&ptr) if ptr != 0 => unsafe { &*(ptr as *const AnyClass) },
            _ => return std::ptr::null_mut(),
        };
        unsafe { msg_send![super(this, subclass), hitTest: point] }
    }

    // Subclass the live content view's own class (not NSView) so the instance
    // size matches and object_setClass stays sound; WKWebView carries ivars a
    // bare NSView subclass would not.
    fn ensure_subclass(base: &AnyClass) -> Option<&'static AnyClass> {
        let raw = CLASS.get_or_init(|| {
            let Some(mut builder) = ClassBuilder::new("MurmureOverlayHitView", base) else {
                warn!("input region: could not declare overlay hit-test subclass");
                return 0;
            };
            // Safety: signature matches NSView's hitTest:.
            unsafe {
                builder.add_method(
                    sel!(hitTest:),
                    hit_test as extern "C" fn(&AnyObject, Sel, NSPoint) -> *mut AnyObject,
                );
            }
            builder.register() as *const AnyClass as usize
        });
        match *raw {
            0 => None,
            ptr => Some(unsafe { &*(ptr as *const AnyClass) }),
        }
    }

    pub fn apply_input_region(window: &WebviewWindow, rects: &[InputRect]) {
        let ns_view = match window.ns_view() {
            Ok(v) if !v.is_null() => v as *mut AnyObject,
            _ => {
                debug!("input region: no ns_view yet, skipping");
                return;
            }
        };
        let scale = window.scale_factor().unwrap_or(1.0);

        // Safety: ns_view is the live content NSView owned by the window.
        unsafe {
            let view = &*ns_view;
            let base = view.class();
            // Register the subclass against the very first view class we see;
            // later calls reuse it and only refresh the region state.
            let Some(hit_class) = ensure_subclass(base) else {
                return;
            };

            let frame: NSRect = msg_send![ns_view, frame];
            match state().lock() {
                Ok(mut guard) => {
                    *guard = Some(HitState {
                        rects: rects.to_vec(),
                        scale,
                        view_height_pt: frame.size.height,
                    })
                }
                Err(_) => {
                    warn!("input region: state lock poisoned, skipping");
                    return;
                }
            }

            if !std::ptr::eq(base, hit_class) {
                AnyObject::set_class(view, hit_class);
            }
        }
        debug!(
            "input region: ns_view hit-test installed, {} rect(s)",
            rects.len()
        );
    }
}
