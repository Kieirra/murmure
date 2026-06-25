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
    use log::{debug, warn};
    use std::sync::Mutex;
    use std::sync::OnceLock;
    use tauri::WebviewWindow;
    use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CallWindowProcW, DefWindowProcW, GetWindowLongPtrW, SetWindowLongPtrW, GWLP_WNDPROC,
        HTCLIENT, HTTRANSPARENT, WM_NCDESTROY, WM_NCHITTEST,
    };

    struct HitState {
        original_proc: isize,
        rects: Vec<InputRect>,
    }

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
                        let screen_x = (lparam & 0xFFFF) as i16 as i32;
                        let screen_y = ((lparam >> 16) & 0xFFFF) as i16 as i32;
                        let mut pt = windows_sys::Win32::Foundation::POINT {
                            x: screen_x,
                            y: screen_y,
                        };
                        windows_sys::Win32::Graphics::Gdi::ScreenToClient(hwnd, &mut pt);
                        if super::point_in_rects(&st.rects, pt.x, pt.y) {
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
    use objc2_foundation::{NSPoint, NSRect};
    use std::sync::Mutex;
    use std::sync::OnceLock;
    use tauri::WebviewWindow;

    struct HitState {
        rects: Vec<InputRect>,
        scale: f64,
        view_height_pt: f64,
    }

    static STATE: OnceLock<Mutex<Option<HitState>>> = OnceLock::new();
    static CLASS: OnceLock<usize> = OnceLock::new();

    fn state() -> &'static Mutex<Option<HitState>> {
        STATE.get_or_init(|| Mutex::new(None))
    }

    extern "C" fn hit_test(this: &AnyObject, _cmd: Sel, point: NSPoint) -> *mut AnyObject {
        let inside = match state().lock() {
            Ok(guard) => guard.as_ref().is_some_and(|st| {
                let px = (point.x * st.scale) as i32;
                let py = ((st.view_height_pt - point.y) * st.scale) as i32;
                super::point_in_rects(&st.rects, px, py)
            }),
            Err(_) => false,
        };
        if !inside {
            return std::ptr::null_mut();
        }
        let subclass = match CLASS.get() {
            Some(&ptr) if ptr != 0 => unsafe { &*(ptr as *const AnyClass) },
            _ => return std::ptr::null_mut(),
        };
        unsafe { msg_send![super(this, subclass), hitTest: point] }
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

    pub fn apply_input_region(window: &WebviewWindow, rects: &[InputRect]) {
        let ns_view = match window.ns_view() {
            Ok(v) if !v.is_null() => v as *mut AnyObject,
            _ => {
                debug!("input region: no ns_view yet, skipping");
                return;
            }
        };
        let scale = window.scale_factor().unwrap_or(1.0);

        unsafe {
            let view = &*ns_view;
            let base = view.class();
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
