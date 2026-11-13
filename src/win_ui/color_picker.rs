use super::*;
const SWATCHES: [lighting::Rgb; 10] = [
    [198, 244, 50],
    [255, 255, 255],
    [30, 180, 255],
    [51, 104, 255],
    [139, 92, 246],
    [236, 72, 153],
    [255, 59, 70],
    [255, 138, 40],
    [255, 211, 58],
    [37, 211, 148],
];
const HEX: i32 = 701;
const APPLY: i32 = 710;
const CANCEL: i32 = 711;
struct Picker {
    hsv: [f32; 3],
    original: lighting::Rgb,
    result: Option<lighting::Rgb>,
    font: HFONT,
    brush: HBRUSH,
    syncing: bool,
    drag: u8,
    error: bool,
}
fn from_rgb(c: lighting::Rgb) -> [f32; 3] {
    let [r, g, b] = c.map(|v| v as f32 / 255.);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;
    let h = if d == 0. {
        0.
    } else if max == r {
        ((g - b) / d).rem_euclid(6.) / 6.
    } else if max == g {
        ((b - r) / d + 2.) / 6.
    } else {
        ((r - g) / d + 4.) / 6.
    };
    [h, if max == 0. { 0. } else { d / max }, max]
}
fn to_rgb([h, s, v]: [f32; 3]) -> lighting::Rgb {
    let h = h.rem_euclid(1.) * 6.;
    let c = v * s;
    let x = c * (1. - (h.rem_euclid(2.) - 1.).abs());
    let m = v - c;
    let rgb = match h as u8 {
        0 => [c, x, 0.],
        1 => [x, c, 0.],
        2 => [0., c, x],
        3 => [0., x, c],
        4 => [x, 0., c],
        _ => [c, 0., x],
    };
    rgb.map(|n| ((n + m) * 255.).round().clamp(0., 255.) as u8)
}
fn parse_hex(value: &str) -> Option<lighting::Rgb> {
    let s = value.trim().strip_prefix('#').unwrap_or(value.trim());
    if s.len() != 6 || !s.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let n = u32::from_str_radix(s, 16).ok()?;
    Some([(n >> 16) as u8, (n >> 8) as u8, n as u8])
}
unsafe fn read(hwnd: HWND, id: i32) -> String {
    let mut buf = [0u16; 64];
    let n = GetWindowTextW(GetDlgItem(hwnd, id), buf.as_mut_ptr(), 64);
    String::from_utf16_lossy(&buf[..n as usize])
}
unsafe fn sync(hwnd: HWND, s: &mut Picker) {
    s.syncing = true;
    let c = to_rgb(s.hsv);
    text(
        GetDlgItem(hwnd, HEX),
        &format!("#{:02X}{:02X}{:02X}", c[0], c[1], c[2]),
    );
    for (i, v) in c.iter().enumerate() {
        text(GetDlgItem(hwnd, 702 + i as i32), &v.to_string());
    }
    s.syncing = false;
    s.error = false;
    EnableWindow(GetDlgItem(hwnd, APPLY), 1);
    InvalidateRect(hwnd, null(), 0);
}
unsafe fn point(hwnd: HWND, s: &mut Picker, x: i32, y: i32) {
    if s.drag == 1 {
        s.hsv[1] = ((x - 24) as f32 / 311.).clamp(0., 1.);
        s.hsv[2] = (1. - (y - 90) as f32 / 189.).clamp(0., 1.);
    }
    if s.drag == 2 {
        s.hsv[0] = ((x - 24) as f32 / 311.).clamp(0., 0.99999);
    }
    sync(hwnd, s);
}
unsafe fn pixels(dc: HDC, x: i32, y: i32, w: i32, h: i32, hue: f32, strip: bool) {
    let data: Vec<u32> = (0..w * h)
        .map(|i| {
            let c = if strip {
                to_rgb([(i % w) as f32 / (w - 1) as f32, 1., 1.])
            } else {
                to_rgb([
                    hue,
                    (i % w) as f32 / (w - 1) as f32,
                    1. - (i / w) as f32 / (h - 1) as f32,
                ])
            };
            (c[0] as u32) << 16 | (c[1] as u32) << 8 | c[2] as u32
        })
        .collect();
    let mut info: BITMAPINFO = std::mem::zeroed();
    info.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
    info.bmiHeader.biWidth = w;
    info.bmiHeader.biHeight = -h;
    info.bmiHeader.biPlanes = 1;
    info.bmiHeader.biBitCount = 32;
    info.bmiHeader.biCompression = BI_RGB;
    StretchDIBits(
        dc,
        x,
        y,
        w,
        h,
        0,
        0,
        w,
        h,
        data.as_ptr().cast(),
        &info,
        DIB_RGB_COLORS,
        SRCCOPY,
    );
}
unsafe fn ring(dc: HDC, x: i32, y: i32) {
    let brush = SelectObject(dc, GetStockObject(NULL_BRUSH));
    for (width, color) in [(4, 0x111111), (2, 0xffffff)] {
        let p = CreatePen(PS_SOLID, width, color);
        let old = SelectObject(dc, p);
        Ellipse(dc, x - 5, y - 5, x + 6, y + 6);
        SelectObject(dc, old);
        DeleteObject(p);
    }
    SelectObject(dc, brush);
}
unsafe fn paint_picker(hwnd: HWND, s: &Picker) {
    let mut ps: PAINTSTRUCT = std::mem::zeroed();
    let target = BeginPaint(hwnd, &mut ps);
    let dc = CreateCompatibleDC(target);
    let bmp = CreateCompatibleBitmap(target, 540, 490);
    let old = SelectObject(dc, bmp);
    theme::rect(dc, 0, 0, 540, 490, theme::BG, theme::BG, 0);
    theme::label(dc, "Rengini seç", 24, 20, 24, theme::WHITE, true);
    theme::label(dc, "Her tuşta senin tonun.", 24, 53, 12, 0x96938e, false);
    pixels(dc, 24, 90, 312, 190, s.hsv[0], false);
    pixels(dc, 24, 300, 312, 16, 0., true);
    ring(
        dc,
        24 + (s.hsv[1] * 311.) as i32,
        90 + ((1. - s.hsv[2]) * 189.) as i32,
    );
    ring(dc, 24 + (s.hsv[0] * 311.) as i32, 308);
    theme::label(dc, "ÖNİZLEME", 360, 88, 11, 0x96938e, true);
    theme::rect(
        dc,
        360,
        112,
        76,
        74,
        theme::rgb(s.original),
        theme::rgb(s.original),
        12,
    );
    theme::rect(
        dc,
        440,
        112,
        76,
        74,
        theme::rgb(to_rgb(s.hsv)),
        theme::rgb(to_rgb(s.hsv)),
        12,
    );
    theme::label(dc, "Önceki", 360, 194, 11, 0x96938e, false);
    theme::label(dc, "Yeni", 440, 194, 11, 0x96938e, false);
    theme::label(dc, "HEX", 360, 230, 11, 0x96938e, true);
    for (i, name) in ["R", "G", "B"].iter().enumerate() {
        theme::label(dc, name, 360 + i as i32 * 54, 300, 11, 0x96938e, true);
    }
    theme::label(dc, "HAZIR TONLAR", 24, 344, 11, 0x96938e, true);
    if s.error {
        theme::label(
            dc,
            "HEX: 6 hane · RGB: 0–255 girin.",
            24,
            407,
            12,
            0x7777ff,
            false,
        );
    }
    for (id, x, y, w, h) in [
        (701, 358, 248, 160, 34),
        (702, 358, 318, 50, 34),
        (703, 412, 318, 50, 34),
        (704, 466, 318, 50, 34),
    ] {
        let _ = id;
        theme::rect(dc, x, y, w, h, theme::PANEL, 0x393633, 8);
    }
    BitBlt(target, 0, 0, 540, 490, dc, 0, 0, SRCCOPY);
    SelectObject(dc, old);
    DeleteObject(bmp);
    DeleteDC(dc);
    EndPaint(hwnd, &ps);
}
unsafe extern "system" fn proc(hwnd: HWND, msg: u32, w: WPARAM, l: LPARAM) -> LRESULT {
    if msg == WM_NCCREATE {
        let cs = &*(l as *const CREATESTRUCTW);
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, cs.lpCreateParams as isize);
    }
    let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Picker;
    if ptr.is_null() {
        return DefWindowProcW(hwnd, msg, w, l);
    }
    let s = &mut *ptr;
    match msg {
        WM_PAINT => {
            paint_picker(hwnd, s);
            0
        }
        WM_ERASEBKGND => 1,
        WM_CTLCOLORSTATIC | WM_CTLCOLOREDIT => {
            let dc = w as HDC;
            SetTextColor(dc, theme::WHITE);
            SetBkColor(dc, theme::PANEL);
            s.brush as LRESULT
        }
        WM_LBUTTONDOWN => {
            let x = (l as u32 & 65535) as i16 as i32;
            let y = ((l as u32 >> 16) & 65535) as i16 as i32;
            s.drag = if (24..336).contains(&x) && (90..280).contains(&y) {
                1
            } else if (24..336).contains(&x) && (294..324).contains(&y) {
                2
            } else {
                0
            };
            if s.drag != 0 {
                SetCapture(hwnd);
                point(hwnd, s, x, y);
            }
            0
        }
        WM_MOUSEMOVE if s.drag != 0 => {
            point(
                hwnd,
                s,
                (l as u32 & 65535) as i16 as i32,
                ((l as u32 >> 16) & 65535) as i16 as i32,
            );
            0
        }
        WM_LBUTTONUP => {
            s.drag = 0;
            ReleaseCapture();
            0
        }
        WM_CAPTURECHANGED => {
            s.drag = 0;
            0
        }
        WM_COMMAND => {
            let id = (w & 65535) as i32;
            let code = (w >> 16) as u32;
            if code == EN_CHANGE && !s.syncing && (701..=704).contains(&id) {
                let c = if id == HEX {
                    parse_hex(&read(hwnd, HEX))
                } else {
                    let vals: Option<Vec<u8>> = (702..705)
                        .map(|id| read(hwnd, id).parse::<u8>().ok())
                        .collect();
                    vals.map(|v| [v[0], v[1], v[2]])
                };
                if let Some(c) = c {
                    let next = from_rgb(c);
                    s.hsv = [
                        if next[1] == 0. { s.hsv[0] } else { next[0] },
                        next[1],
                        next[2],
                    ];
                    s.error = false;
                    s.syncing = true;
                    if id == HEX {
                        for (i, v) in c.iter().enumerate() {
                            text(GetDlgItem(hwnd, 702 + i as i32), &v.to_string());
                        }
                    } else {
                        text(
                            GetDlgItem(hwnd, HEX),
                            &format!("#{:02X}{:02X}{:02X}", c[0], c[1], c[2]),
                        );
                    }
                    s.syncing = false;
                } else {
                    s.error = true;
                }
                EnableWindow(GetDlgItem(hwnd, APPLY), (!s.error) as i32);
                InvalidateRect(hwnd, null(), 0);
            }
            if (720..730).contains(&id) {
                s.hsv = from_rgb(SWATCHES[(id - 720) as usize]);
                sync(hwnd, s);
            }
            if id == APPLY && !s.error {
                s.result = Some(to_rgb(s.hsv));
                DestroyWindow(hwnd);
            }
            if id == CANCEL {
                DestroyWindow(hwnd);
            }
            0
        }
        WM_DRAWITEM => {
            let d = &*(l as *const DRAWITEMSTRUCT);
            let id = d.CtlID as i32;
            let (w, h) = (d.rcItem.right, d.rcItem.bottom);
            let pressed = d.itemState & ODS_SELECTED != 0;
            let background = CreateSolidBrush(theme::BG);
            FillRect(d.hDC, &d.rcItem, background);
            DeleteObject(background);
            if (720..730).contains(&id) {
                let c = SWATCHES[(id - 720) as usize];
                theme::rect(
                    d.hDC,
                    2,
                    2,
                    w - 4,
                    h - 4,
                    theme::rgb(c),
                    if d.itemState & ODS_FOCUS != 0 {
                        theme::WHITE
                    } else {
                        theme::rgb(c)
                    },
                    10,
                );
            } else {
                let primary = id == APPLY;
                theme::rect(
                    d.hDC,
                    0,
                    0,
                    w,
                    h,
                    if primary {
                        if pressed {
                            0x24c49e
                        } else {
                            0x32f4c6
                        }
                    } else {
                        theme::PANEL
                    },
                    if primary { 0x32f4c6 } else { 0x393633 },
                    10,
                );
                SelectObject(d.hDC, s.font);
                SetBkMode(d.hDC, TRANSPARENT as i32);
                SetTextColor(d.hDC, if primary { theme::BG } else { theme::WHITE });
                let mut rr = d.rcItem;
                let title = wide(if primary { "Uygula" } else { "Vazgeç" });
                DrawTextW(
                    d.hDC,
                    title.as_ptr(),
                    -1,
                    &mut rr,
                    DT_CENTER | DT_VCENTER | DT_SINGLELINE,
                );
                if d.itemState & ODS_FOCUS != 0 {
                    let mut rr = d.rcItem;
                    InflateRect(&mut rr, -4, -4);
                    DrawFocusRect(d.hDC, &rr);
                }
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
pub unsafe fn show(owner: HWND, value: lighting::Rgb, font: HFONT) -> Option<lighting::Rgb> {
    let name = wide("RS7ColorPicker");
    let instance = GetModuleHandleW(null());
    let mut wc: WNDCLASSW = std::mem::zeroed();
    wc.lpfnWndProc = Some(proc);
    wc.hInstance = instance;
    wc.lpszClassName = name.as_ptr();
    wc.hCursor = LoadCursorW(null_mut(), IDC_ARROW);
    wc.hIcon = crate::brand::icon(32);
    RegisterClassW(&wc);
    let mut s = Box::new(Picker {
        hsv: from_rgb(value),
        original: value,
        result: None,
        font,
        brush: CreateSolidBrush(theme::PANEL),
        syncing: false,
        drag: 0,
        error: false,
    });
    let mut bounds = RECT {
        left: 0,
        top: 0,
        right: 540,
        bottom: 490,
    };
    let style = WS_CAPTION | WS_SYSMENU | WS_POPUP | WS_CLIPCHILDREN;
    AdjustWindowRectEx(&mut bounds, style, 0, WS_EX_DLGMODALFRAME);
    let mut parent: RECT = std::mem::zeroed();
    GetWindowRect(owner, &mut parent);
    let w = bounds.right - bounds.left;
    let h = bounds.bottom - bounds.top;
    let hwnd = CreateWindowExW(
        WS_EX_DLGMODALFRAME,
        name.as_ptr(),
        wide("RS7 · Renk seçimi").as_ptr(),
        style,
        parent.left + ((parent.right - parent.left - w) / 2).max(0),
        parent.top + ((parent.bottom - parent.top - h) / 2).max(0),
        w,
        h,
        owner,
        null_mut(),
        instance,
        (&mut *s as *mut Picker).cast(),
    );
    if hwnd.is_null() {
        DeleteObject(s.brush);
        return None;
    }
    let dark = 1i32;
    windows_sys::Win32::Graphics::Dwm::DwmSetWindowAttribute(
        hwnd,
        20,
        (&dark as *const i32).cast(),
        4,
    );
    for (id, x, y, width) in [
        (701, 370, 257, 136),
        (702, 366, 327, 34),
        (703, 420, 327, 34),
        (704, 474, 327, 34),
    ] {
        let edit = child(
            hwnd,
            "EDIT",
            "",
            id,
            x,
            y,
            width,
            20,
            WS_TABSTOP | ES_AUTOHSCROLL as u32,
            font,
        );
        SendMessageW(edit, EM_SETLIMITTEXT, if id == HEX { 7 } else { 3 }, 0);
    }
    for i in 0..10 {
        child(
            hwnd,
            "BUTTON",
            "",
            720 + i,
            24 + i * 31,
            368,
            28,
            28,
            WS_TABSTOP | BS_OWNERDRAW as u32,
            font,
        );
    }
    child(
        hwnd,
        "BUTTON",
        "Vazgeç",
        CANCEL,
        256,
        440,
        120,
        36,
        WS_TABSTOP | BS_OWNERDRAW as u32,
        font,
    );
    child(
        hwnd,
        "BUTTON",
        "Uygula",
        APPLY,
        388,
        440,
        128,
        36,
        WS_TABSTOP | BS_OWNERDRAW as u32,
        font,
    );
    sync(hwnd, &mut s);
    EnableWindow(owner, 0);
    ShowWindow(hwnd, SW_SHOW);
    SetFocus(GetDlgItem(hwnd, HEX));
    SendMessageW(GetDlgItem(hwnd, HEX), EM_SETSEL, 0, -1);
    let mut msg: MSG = std::mem::zeroed();
    while IsWindow(hwnd) != 0 {
        let status = GetMessageW(&mut msg, null_mut(), 0, 0);
        if status <= 0 {
            if status == 0 {
                PostQuitMessage(msg.wParam as i32);
            }
            break;
        }
        if msg.message == WM_KEYDOWN && GetAncestor(msg.hwnd, GA_ROOT) == hwnd {
            if msg.wParam == VK_ESCAPE as usize {
                DestroyWindow(hwnd);
                continue;
            }
            if msg.wParam == VK_RETURN as usize {
                if !s.error {
                    s.result = Some(to_rgb(s.hsv));
                    DestroyWindow(hwnd);
                }
                continue;
            }
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
    SetForegroundWindow(owner);
    DeleteObject(s.brush);
    s.result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rgb_hsv_roundtrip_and_hex_validation() {
        for r in [0, 1, 64, 128, 254, 255] {
            for g in [0, 64, 198, 255] {
                for b in [0, 50, 128, 255] {
                    let c = [r, g, b];
                    assert_eq!(to_rgb(from_rgb(c)), c);
                }
            }
        }
        assert_eq!(parse_hex(" #c6F432 "), Some([198, 244, 50]));
        for bad in ["#123", "#GG0000", "#1234567", "💚000"] {
            assert_eq!(parse_hex(bad), None);
        }
    }
}
