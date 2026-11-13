use super::*;
struct Notice {
    accept: std::cell::Cell<bool>,
    font: HFONT,
    preview: bool,
    version: String,
}
unsafe extern "system" fn proc(hwnd: HWND, msg: u32, w: WPARAM, l: LPARAM) -> LRESULT {
    if msg == WM_NCCREATE {
        let c = &*(l as *const CREATESTRUCTW);
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, c.lpCreateParams as isize);
    }
    let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const Notice;
    if ptr.is_null() {
        return DefWindowProcW(hwnd, msg, w, l);
    }
    let s = &*ptr;
    match msg {
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let target = BeginPaint(hwnd, &mut ps);
            let dc = CreateCompatibleDC(target);
            let bmp = CreateCompatibleBitmap(target, 520, 240);
            let old = SelectObject(dc, bmp);
            theme::rect(dc, 0, 0, 520, 240, theme::BG, theme::BG, 0);
            theme::label(dc, "Yeni güncelleme hazır", 24, 22, 24, theme::WHITE, true);
            theme::label(
                dc,
                &format!("RS7 Light v{}", s.version),
                24,
                62,
                16,
                0x32f4c6,
                true,
            );
            theme::label(
                dc,
                if s.preview {
                    "Bu bir önizleme; indirme veya kurulum yapılmaz."
                } else {
                    "Güncelleme indirilip doğrulanacak; ayarların korunacak."
                },
                24,
                102,
                13,
                0x96938e,
                false,
            );
            theme::label(
                dc,
                "Daha sonra seçersen ışıkların çalışmaya devam eder.",
                24,
                126,
                13,
                0x96938e,
                false,
            );
            BitBlt(target, 0, 0, 520, 240, dc, 0, 0, SRCCOPY);
            SelectObject(dc, old);
            DeleteObject(bmp);
            DeleteDC(dc);
            EndPaint(hwnd, &ps);
            0
        }
        WM_ERASEBKGND => 1,
        WM_DRAWITEM => {
            let d = &*(l as *const DRAWITEMSTRUCT);
            let yes = d.CtlID == 970;
            let b = CreateSolidBrush(theme::BG);
            FillRect(d.hDC, &d.rcItem, b);
            DeleteObject(b);
            theme::rect(
                d.hDC,
                0,
                0,
                d.rcItem.right,
                d.rcItem.bottom,
                if yes { 0x32f4c6 } else { theme::PANEL },
                if yes { 0x32f4c6 } else { 0x393633 },
                10,
            );
            let old = SelectObject(d.hDC, s.font);
            SetBkMode(d.hDC, TRANSPARENT as i32);
            SetTextColor(d.hDC, if yes { theme::BG } else { theme::WHITE });
            let mut r = d.rcItem;
            let t = wide(if yes { "Güncelle" } else { "Daha sonra" });
            DrawTextW(
                d.hDC,
                t.as_ptr(),
                -1,
                &mut r,
                DT_CENTER | DT_VCENTER | DT_SINGLELINE,
            );
            SelectObject(d.hDC, old);
            if d.itemState & ODS_FOCUS != 0 {
                InflateRect(&mut r, -4, -4);
                DrawFocusRect(d.hDC, &r);
            }
            1
        }
        WM_COMMAND => {
            match w & 65535 {
                970 => {
                    s.accept.set(true);
                    DestroyWindow(hwnd);
                }
                971 => {
                    DestroyWindow(hwnd);
                }
                _ => {}
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
pub unsafe fn show(owner: HWND, version: &str, preview: bool, font: HFONT) -> bool {
    let class = wide("RS7LightUpdateNotice");
    let instance = GetModuleHandleW(null());
    let mut wc: WNDCLASSW = std::mem::zeroed();
    wc.hInstance = instance;
    wc.lpfnWndProc = Some(proc);
    wc.lpszClassName = class.as_ptr();
    wc.hCursor = LoadCursorW(null_mut(), IDC_ARROW);
    wc.hIcon = crate::brand::icon(32);
    RegisterClassW(&wc);
    let s = Box::new(Notice {
        accept: std::cell::Cell::new(false),
        font,
        preview,
        version: version.into(),
    });
    let style = WS_POPUP | WS_CAPTION | WS_SYSMENU | WS_CLIPCHILDREN;
    let mut r = RECT {
        left: 0,
        top: 0,
        right: 520,
        bottom: 240,
    };
    AdjustWindowRectEx(&mut r, style, 0, WS_EX_DLGMODALFRAME);
    let (w, h) = (r.right - r.left, r.bottom - r.top);
    let mut info: MONITORINFO = std::mem::zeroed();
    info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
    GetMonitorInfoW(
        MonitorFromWindow(owner, MONITOR_DEFAULTTONEAREST),
        &mut info,
    );
    let a = info.rcWork;
    let hwnd = CreateWindowExW(
        WS_EX_DLGMODALFRAME | WS_EX_TOPMOST,
        class.as_ptr(),
        wide("RS7 Light · Güncelleme").as_ptr(),
        style,
        a.left + (a.right - a.left - w) / 2,
        a.top + (a.bottom - a.top - h) / 2,
        w,
        h,
        owner,
        null_mut(),
        instance,
        (&*s as *const Notice).cast(),
    );
    if hwnd.is_null() {
        return false;
    }
    let dark = 1i32;
    windows_sys::Win32::Graphics::Dwm::DwmSetWindowAttribute(
        hwnd,
        20,
        (&dark as *const i32).cast(),
        4,
    );
    child(
        hwnd,
        "BUTTON",
        "Daha sonra",
        971,
        204,
        184,
        140,
        36,
        BS_OWNERDRAW as u32 | WS_TABSTOP,
        font,
    );
    child(
        hwnd,
        "BUTTON",
        "Güncelle",
        970,
        356,
        184,
        140,
        36,
        BS_OWNERDRAW as u32 | WS_TABSTOP,
        font,
    );
    EnableWindow(owner, 0);
    ShowWindow(hwnd, SW_SHOW);
    SetForegroundWindow(hwnd);
    SetFocus(GetDlgItem(hwnd, 971));
    let mut msg: MSG = std::mem::zeroed();
    while IsWindow(hwnd) != 0 {
        let status = GetMessageW(&mut msg, null_mut(), 0, 0);
        if status <= 0 {
            if status == 0 {
                PostQuitMessage(msg.wParam as i32);
            }
            break;
        }
        if msg.message == WM_KEYDOWN
            && msg.wParam == VK_ESCAPE as usize
            && (msg.hwnd == hwnd || GetParent(msg.hwnd) == hwnd)
        {
            DestroyWindow(hwnd);
            continue;
        }
        if msg.message == WM_KEYDOWN
            && msg.wParam == VK_RETURN as usize
            && (msg.hwnd == hwnd || GetParent(msg.hwnd) == hwnd)
        {
            let id = GetDlgCtrlID(GetFocus());
            SendMessageW(hwnd, WM_COMMAND, if id == 970 { 970 } else { 971 }, 0);
            continue;
        }
        if IsDialogMessageW(hwnd, &msg) == 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    if IsWindow(hwnd) != 0 {
        DestroyWindow(hwnd);
    }
    EnableWindow(owner, 1);
    if IsWindowVisible(owner) != 0 {
        SetForegroundWindow(owner);
    }
    s.accept.get()
}
