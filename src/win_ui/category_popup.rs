//! Theme-matched category popup with native accessible buttons and keyboard navigation.
use super::*;
struct Menu {
    selected: usize,
    hovered: std::cell::Cell<usize>,
    choice: std::cell::Cell<Option<usize>>,
    row: i32,
    width: i32,
    font: HFONT,
}
fn title(index: usize) -> &'static str {
    if index == 0 {
        "Tümü"
    } else {
        lighting::CATEGORIES[index - 1]
    }
}
unsafe fn finish(hwnd: HWND, s: &Menu, index: usize) {
    s.choice.set(Some(index));
    DestroyWindow(hwnd);
}
unsafe extern "system" fn proc(hwnd: HWND, msg: u32, w: WPARAM, l: LPARAM) -> LRESULT {
    if msg == WM_NCCREATE {
        let c = &*(l as *const CREATESTRUCTW);
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, c.lpCreateParams as isize);
    }
    let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const Menu;
    if ptr.is_null() {
        return DefWindowProcW(hwnd, msg, w, l);
    }
    let s = &*ptr;
    let at = |l: LPARAM| {
        let x = (l as u32 & 65535) as i16 as i32;
        let y = ((l as u32 >> 16) & 65535) as i16 as i32;
        if x >= 6 && x < s.width - 6 && y >= 8 && y < 8 + s.row * 8 {
            Some(((y - 8) / s.row) as usize)
        } else {
            None
        }
    };
    match msg {
        WM_ERASEBKGND => 1,
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let dc = BeginPaint(hwnd, &mut ps);
            theme::rect(
                dc,
                0,
                0,
                s.width,
                s.row * 8 + 16,
                theme::PANEL,
                0x393633,
                14,
            );
            EndPaint(hwnd, &ps);
            0
        }
        WM_DRAWITEM => {
            let d = &*(l as *const DRAWITEMSTRUCT);
            let i = (d.CtlID - 900) as usize;
            if i >= 8 {
                return 0;
            }
            let dc = d.hDC;
            let h = d.rcItem.bottom;
            let width = d.rcItem.right;
            let b = CreateSolidBrush(theme::PANEL);
            FillRect(dc, &d.rcItem, b);
            DeleteObject(b);
            let active = s.hovered.get() == i;
            let chosen = s.selected == i;
            theme::rect(
                dc,
                0,
                0,
                width,
                h,
                if active {
                    0x243122
                } else if chosen {
                    0x20281c
                } else {
                    theme::PANEL
                },
                if active { 0x344630 } else { theme::PANEL },
                9,
            );
            let old = SelectObject(dc, s.font);
            SetBkMode(dc, TRANSPARENT as i32);
            SetTextColor(dc, if chosen { 0x32f4c6 } else { theme::WHITE });
            let t = wide(title(i));
            let mut r = RECT {
                left: 36,
                top: 0,
                right: width - 12,
                bottom: h,
            };
            DrawTextW(
                dc,
                t.as_ptr(),
                -1,
                &mut r,
                DT_SINGLELINE | DT_VCENTER | DT_END_ELLIPSIS,
            );
            if chosen {
                let t = wide("✓");
                r.left = 12;
                r.right = 34;
                DrawTextW(dc, t.as_ptr(), -1, &mut r, DT_SINGLELINE | DT_VCENTER);
            }
            SelectObject(dc, old);
            1
        }
        WM_MOUSEMOVE => {
            if let Some(i) = at(l) {
                if s.hovered.replace(i) != i {
                    RedrawWindow(hwnd, null(), null_mut(), RDW_INVALIDATE | RDW_ALLCHILDREN);
                }
            }
            0
        }
        WM_LBUTTONDOWN => {
            if at(l).is_none() {
                DestroyWindow(hwnd);
            }
            0
        }
        WM_LBUTTONUP => {
            if let Some(i) = at(l) {
                finish(hwnd, s, i);
            } else {
                DestroyWindow(hwnd);
            }
            0
        }
        WM_COMMAND => {
            let id = (w & 65535) as usize;
            if (900..908).contains(&id) {
                finish(hwnd, s, id - 900);
            }
            0
        }
        WM_KEYDOWN => {
            let i = s.hovered.get();
            match w as u16 {
                VK_ESCAPE => {
                    DestroyWindow(hwnd);
                }
                VK_RETURN | VK_SPACE => finish(hwnd, s, i),
                VK_DOWN => s.hovered.set((i + 1) % 8),
                VK_UP => s.hovered.set((i + 7) % 8),
                VK_HOME => s.hovered.set(0),
                VK_END => s.hovered.set(7),
                _ => {}
            }
            if IsWindow(hwnd) != 0 {
                RedrawWindow(hwnd, null(), null_mut(), RDW_INVALIDATE | RDW_ALLCHILDREN);
            }
            0
        }
        WM_ACTIVATE if w as u32 & 65535 == WA_INACTIVE => {
            DestroyWindow(hwnd);
            0
        }
        WM_CAPTURECHANGED => {
            if IsWindow(hwnd) != 0 {
                DestroyWindow(hwnd);
            }
            0
        }
        WM_CLOSE => {
            DestroyWindow(hwnd);
            0
        }
        _ => DefWindowProcW(hwnd, msg, w, l),
    }
}
pub unsafe fn show(
    owner: HWND,
    anchor: HWND,
    category: Option<usize>,
    font: HFONT,
    scale: f32,
) -> Option<usize> {
    let class = wide("RS7LightCategoryPopup");
    let instance = GetModuleHandleW(null());
    let mut wc: WNDCLASSW = std::mem::zeroed();
    wc.hInstance = instance;
    wc.lpfnWndProc = Some(proc);
    wc.lpszClassName = class.as_ptr();
    wc.hCursor = LoadCursorW(null_mut(), IDC_ARROW);
    RegisterClassW(&wc);
    let row = (36. * scale).round() as i32;
    let mut a: RECT = std::mem::zeroed();
    GetWindowRect(anchor, &mut a);
    let width = (a.right - a.left).max((230. * scale) as i32);
    let height = row * 8 + 16;
    let mut info: MONITORINFO = std::mem::zeroed();
    info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
    GetMonitorInfoW(MonitorFromRect(&a, MONITOR_DEFAULTTONEAREST), &mut info);
    let work = info.rcWork;
    let width = width.min(work.right - work.left);
    let x = a.left.clamp(work.left, (work.right - width).max(work.left));
    let y = if a.bottom + 6 + height <= work.bottom {
        a.bottom + 6
    } else {
        (a.top - height - 6).max(work.top)
    };
    let s = Box::new(Menu {
        selected: category.map_or(0, |i| i + 1),
        hovered: std::cell::Cell::new(category.map_or(0, |i| i + 1)),
        choice: std::cell::Cell::new(None),
        row,
        width,
        font,
    });
    let hwnd = CreateWindowExW(
        WS_EX_TOOLWINDOW,
        class.as_ptr(),
        wide("Kategoriler").as_ptr(),
        WS_POPUP | WS_CLIPCHILDREN,
        x,
        y,
        width,
        height,
        owner,
        null_mut(),
        instance,
        (&*s as *const Menu).cast(),
    );
    if hwnd.is_null() {
        return None;
    }
    let region = CreateRoundRectRgn(0, 0, width + 1, height + 1, 14, 14);
    if SetWindowRgn(hwnd, region, 0) == 0 {
        DeleteObject(region);
    }
    for i in 0..8 {
        child(
            hwnd,
            "BUTTON",
            title(i),
            900 + i as i32,
            6,
            8 + i as i32 * row,
            width - 12,
            row,
            BS_OWNERDRAW as u32 | WS_TABSTOP,
            font,
        );
    }
    ShowWindow(hwnd, SW_SHOW);
    SetForegroundWindow(hwnd);
    SetFocus(hwnd);
    SetCapture(hwnd);
    let mut msg: MSG = std::mem::zeroed();
    while IsWindow(hwnd) != 0 {
        let status = GetMessageW(&mut msg, null_mut(), 0, 0);
        if status <= 0 {
            if status == 0 {
                PostQuitMessage(msg.wParam as i32);
            }
            break;
        }
        if msg.message == WM_KEYDOWN && (msg.hwnd == hwnd || GetParent(msg.hwnd) == hwnd) {
            SendMessageW(hwnd, WM_KEYDOWN, msg.wParam, msg.lParam);
        } else {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    if IsWindow(hwnd) != 0 {
        DestroyWindow(hwnd);
    }
    if GetCapture() == hwnd {
        ReleaseCapture();
    }
    if IsWindow(owner) != 0 {
        SetForegroundWindow(owner);
    }
    s.choice.get()
}
