use super::*;
const ID: i32 = 215;
#[derive(Clone, Copy, Debug)]
struct Geometry {
    top: i32,
    length: i32,
    travel: i32,
    max: i32,
}
fn geometry(track: i32, view: i32, content: i32, position: i32) -> Geometry {
    let track = track.max(1);
    let content = content.max(1);
    let max = (content - view).max(0);
    let length = if max == 0 {
        track
    } else {
        ((track as i64 * view as i64 / content as i64) as i32).clamp(32.min(track), track)
    };
    let travel = track - length;
    let top = if max == 0 {
        0
    } else {
        (position.clamp(0, max) as i64 * travel as i64 / max as i64) as i32
    };
    Geometry {
        top,
        length,
        travel,
        max,
    }
}
unsafe fn shape(hwnd: HWND, s: &Ui) -> Geometry {
    let mut r: RECT = std::mem::zeroed();
    GetClientRect(hwnd, &mut r);
    let l = responsive::current(GetParent(hwnd), s);
    geometry(r.bottom, l.h, l.content, l.scroll)
}
unsafe extern "system" fn proc(hwnd: HWND, msg: u32, w: WPARAM, l: LPARAM) -> LRESULT {
    let parent = GetParent(hwnd);
    let ptr = GetWindowLongPtrW(parent, GWLP_USERDATA) as *const Ui;
    if ptr.is_null() {
        return DefWindowProcW(hwnd, msg, w, l);
    }
    let s = &*ptr;
    match msg {
        WM_ERASEBKGND => 1,
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let dc = BeginPaint(hwnd, &mut ps);
            let mut r: RECT = std::mem::zeroed();
            GetClientRect(hwnd, &mut r);
            let g = shape(hwnd, s);
            theme::rect(dc, 0, 0, r.right, r.bottom, theme::BG, theme::BG, 0);
            theme::rect(dc, 2, 0, r.right - 4, r.bottom, 0x1c1d19, 0x1c1d19, 8);
            let active = s.scroll_hover.get() || s.scroll_drag.get().is_some();
            let c = if active { 0x32f4c6 } else { 0x59624e };
            theme::rect(dc, 2, g.top, r.right - 4, g.length, c, c, 8);
            EndPaint(hwnd, &ps);
            0
        }
        WM_MOUSEMOVE => {
            if !s.scroll_hover.replace(true) {
                let mut t: TRACKMOUSEEVENT = std::mem::zeroed();
                t.cbSize = std::mem::size_of::<TRACKMOUSEEVENT>() as u32;
                t.dwFlags = TME_LEAVE;
                t.hwndTrack = hwnd;
                TrackMouseEvent(&mut t);
                InvalidateRect(hwnd, null(), 0);
            }
            if let Some((start, position)) = s.scroll_drag.get() {
                let y = ((l as u32 >> 16) & 65535) as i16 as i32;
                let g = shape(hwnd, s);
                if g.travel > 0 {
                    let target =
                        position + ((y - start) as i64 * g.max as i64 / g.travel as i64) as i32;
                    let now = responsive::current(parent, s).scroll;
                    responsive::scroll(parent, s, target - now);
                }
            }
            0
        }
        WM_MOUSELEAVE => {
            s.scroll_hover.set(false);
            InvalidateRect(hwnd, null(), 0);
            0
        }
        WM_LBUTTONDOWN => {
            SetFocus(hwnd);
            let y = ((l as u32 >> 16) & 65535) as i16 as i32;
            let g = shape(hwnd, s);
            let current = responsive::current(parent, s);
            if y >= g.top && y < g.top + g.length {
                s.scroll_drag.set(Some((y, current.scroll)));
                SetCapture(hwnd);
                InvalidateRect(hwnd, null(), 0);
            } else {
                responsive::scroll(
                    parent,
                    s,
                    if y < g.top {
                        -current.h + 64
                    } else {
                        current.h - 64
                    },
                );
            }
            0
        }
        WM_LBUTTONUP => {
            s.scroll_drag.set(None);
            if GetCapture() == hwnd {
                ReleaseCapture();
            }
            InvalidateRect(hwnd, null(), 0);
            0
        }
        WM_CAPTURECHANGED => {
            s.scroll_drag.set(None);
            InvalidateRect(hwnd, null(), 0);
            0
        }
        WM_KEYDOWN => {
            let current = responsive::current(parent, s);
            let delta = match w as u16 {
                VK_UP => -48,
                VK_DOWN => 48,
                VK_PRIOR => -current.h + 64,
                VK_NEXT => current.h - 64,
                VK_HOME => -current.scroll,
                VK_END => current.content,
                _ => 0,
            };
            responsive::scroll(parent, s, delta);
            0
        }
        WM_MOUSEWHEEL => {
            SendMessageW(parent, msg, w, l);
            0
        }
        WM_GETDLGCODE => DLGC_WANTARROWS as LRESULT,
        _ => DefWindowProcW(hwnd, msg, w, l),
    }
}
pub unsafe fn update(parent: HWND, s: &Ui) {
    let l = responsive::current(parent, s);
    let mut hwnd = GetDlgItem(parent, ID);
    if hwnd.is_null() {
        let class = wide("RS7LightScrollbar");
        let instance = GetModuleHandleW(null());
        let mut wc: WNDCLASSW = std::mem::zeroed();
        wc.lpfnWndProc = Some(proc);
        wc.hInstance = instance;
        wc.lpszClassName = class.as_ptr();
        wc.hCursor = LoadCursorW(null_mut(), IDC_ARROW);
        RegisterClassW(&wc);
        hwnd = CreateWindowExW(
            0,
            class.as_ptr(),
            wide("Dikey kaydırma").as_ptr(),
            WS_CHILD | WS_TABSTOP,
            l.w - 16,
            8,
            12,
            l.h - 16,
            parent,
            ID as usize as HMENU,
            instance,
            null(),
        );
    }
    MoveWindow(hwnd, l.w - 16, 8, 12, (l.h - 16).max(1), 0);
    ShowWindow(hwnd, if l.content > l.h { SW_SHOW } else { SW_HIDE });
    InvalidateRect(hwnd, null(), 0);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn thumb_reaches_both_ends_and_has_minimum_size() {
        for (track, view, content) in [(600, 616, 1200), (300, 316, 20000), (900, 916, 1000)] {
            let a = geometry(track, view, content, 0);
            assert_eq!(a.top, 0);
            assert!(a.length >= 32 && a.length <= track);
            let b = geometry(track, view, content, i32::MAX);
            assert_eq!(b.top + b.length, track);
            assert_eq!(geometry(track, view, content, -100).top, 0);
        }
    }
    #[test]
    fn no_overflow_fills_track() {
        let g = geometry(500, 516, 400, 100);
        assert_eq!((g.top, g.length, g.max), (0, 500, 0));
    }
}
