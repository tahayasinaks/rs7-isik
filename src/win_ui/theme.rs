use super::*;
pub const BG: u32 = 0x0c0b0b;
pub const PANEL: u32 = 0x161514;
pub const WHITE: u32 = 0xffffff;
const MUTED: u32 = 0x96938e;
const LINE: u32 = 0x302d29;
const LIME: u32 = 0x32f4c6;
// Win32 STATIC_STYLES (winuser.h).
const SS_RIGHT: u32 = 2;
const SS_ENDELLIPSIS: u32 = 0x4000;
pub(super) fn rgb(c: [u8; 3]) -> u32 {
    c[0] as u32 | ((c[1] as u32) << 8) | ((c[2] as u32) << 16)
}
pub(super) unsafe fn rect(dc: HDC, x: i32, y: i32, w: i32, h: i32, fill: u32, border: u32, r: i32) {
    let brush = CreateSolidBrush(fill);
    let pen = CreatePen(PS_SOLID, 1, border);
    let oldb = SelectObject(dc, brush);
    let oldp = SelectObject(dc, pen);
    RoundRect(dc, x, y, x + w, y + h, r, r);
    SelectObject(dc, oldb);
    SelectObject(dc, oldp);
    DeleteObject(brush);
    DeleteObject(pen);
}
pub(super) unsafe fn label(
    dc: HDC,
    value: &str,
    x: i32,
    y: i32,
    size: i32,
    color: u32,
    bold: bool,
) {
    let font = CreateFontW(
        -size,
        0,
        0,
        0,
        if bold { 600 } else { 400 },
        0,
        0,
        0,
        DEFAULT_CHARSET as u32,
        0,
        0,
        CLEARTYPE_QUALITY as u32,
        0,
        wide("Segoe UI").as_ptr(),
    );
    let old = SelectObject(dc, font);
    SetTextColor(dc, color);
    SetBkMode(dc, TRANSPARENT as i32);
    let t = wide(value);
    TextOutW(dc, x, y, t.as_ptr(), (t.len() - 1) as i32);
    SelectObject(dc, old);
    DeleteObject(font);
}
unsafe fn btn(hwnd: HWND, s: &Ui, id: i32, t: &str, x: i32, y: i32, w: i32, h: i32) {
    child(
        hwnd,
        "BUTTON",
        t,
        id,
        x,
        y,
        w,
        h,
        BS_OWNERDRAW as u32 | WS_TABSTOP,
        s.font,
    );
}

pub unsafe fn build(hwnd: HWND, s: &mut Ui) {
    btn(hwnd, s, CONNECT, "Klavyeyi bağla", 856, 38, 140, 42);
    btn(hwnd, s, DISCONNECT, "Bağlantıyı kes", 856, 38, 140, 42);
    btn(hwnd, s, START, "Başlat", 1008, 38, 124, 42);
    btn(hwnd, s, STOP, "Durdur", 1008, 38, 124, 42);
    for slot in 0..crate::library::PAGE_SIZE as i32 {
        btn(
            hwnd,
            s,
            400 + slot,
            "",
            40 + (slot % 3) * 232,
            550 + (slot / 3) * 106,
            220,
            100,
        );
    }
    let search = child(
        hwnd,
        "EDIT",
        "",
        SEARCH,
        300,
        505,
        212,
        25,
        WS_TABSTOP | ES_AUTOHSCROLL as u32,
        s.font,
    );
    SendMessageW(search, 0x1501, 0, wide("Efekt ara…").as_ptr() as isize);
    windows_sys::Win32::UI::Controls::SetWindowTheme(search, wide("DarkMode_CFD").as_ptr(), null());
    btn(
        hwnd,
        s,
        CATEGORY_FILTER,
        "Kategori: Tümü",
        544,
        500,
        188,
        34,
    );
    btn(hwnd, s, PREVIOUS, "← Önceki", 40, 766, 104, 28);
    btn(hwnd, s, NEXT, "Sonraki →", 620, 766, 112, 28);
    let settings = s.worker.settings.lock().unwrap().clone();
    for (id, y, min, max, pos) in [
        (BRIGHT, 596, 0, 100, (settings.brightness * 100.) as i32),
        (SPEED, 654, 20, 300, (settings.speed * 100.) as i32),
    ] {
        child(
            hwnd,
            "STATIC",
            if id == BRIGHT { "Parlaklık" } else { "Hız" },
            if id == BRIGHT { 211 } else { 212 },
            790,
            y - 22,
            200,
            22,
            0,
            s.font,
        );
        let h = child(
            hwnd,
            "msctls_trackbar32",
            "",
            id,
            786,
            y,
            318,
            28,
            WS_TABSTOP | TBS_NOTICKS,
            s.font,
        );
        let value = child(
            hwnd,
            "STATIC",
            "",
            0,
            1032,
            y - 22,
            72,
            24,
            SS_RIGHT,
            s.font,
        );
        if id == BRIGHT {
            s.brightness = value;
        } else {
            s.speed = value;
        }
        SendMessageW(h, TBM_SETRANGE, 1, ((max << 16) | min) as isize);
        SendMessageW(h, TBM_SETPOS, 1, pos as isize);
        text(h, if id == BRIGHT { "Parlaklık" } else { "Hız" });
        windows_sys::Win32::UI::Controls::SetWindowTheme(
            h,
            wide("DarkMode_Explorer").as_ptr(),
            null(),
        );
    }
    for (i, t) in ["Gökkuşağı", "Tek renk", "İki renk"].iter().enumerate() {
        btn(
            hwnd,
            s,
            600 + i as i32,
            t,
            790 + i as i32 * 104,
            698,
            98,
            32,
        );
    }
    s.first = child(
        hwnd,
        "BUTTON",
        "",
        FIRST,
        790,
        744,
        150,
        36,
        BS_OWNERDRAW as u32 | WS_TABSTOP,
        s.font,
    );
    s.second = child(
        hwnd,
        "BUTTON",
        "",
        SECOND,
        950,
        744,
        150,
        36,
        BS_OWNERDRAW as u32 | WS_TABSTOP,
        s.font,
    );
    text(
        s.brightness,
        &format!("%{}", (settings.brightness * 100.).round() as i32),
    );
    text(s.speed, &format!("{:.2}×", settings.speed));
    for (h, c) in [(s.first, settings.first), (s.second, settings.second)] {
        text(h, &format!("#{:02X}{:02X}{:02X}", c[0], c[1], c[2]));
    }
    s.status = child(
        hwnd,
        "STATIC",
        if s.preview {
            "Tasarım önizlemesi · Klavyeniz etkilenmez; canlı kaynaklar örnek veridir."
        } else {
            "Başlamak için klavyeyi USB ile bağlayın."
        },
        0,
        40,
        810,
        1088,
        22,
        SS_ENDELLIPSIS as u32,
        s.font,
    );
    if s.preview {
        text(GetDlgItem(hwnd, CONNECT), "USB kapalı");
    }
    btn(
        hwnd,
        s,
        AUTOSTART,
        if s.autostart.get() {
            "Windows ile başlat · Açık"
        } else {
            "Windows ile başlat · Kapalı"
        },
        790,
        804,
        310,
        40,
    );
    refresh_library(hwnd, s);
    mode_controls(hwnd, s);
    sync_actions(hwnd, s);
    SetTimer(hwnd, 1, 33, None);
}
unsafe fn visibility(h: HWND, show: bool) {
    if (GetWindowLongPtrW(h, GWL_STYLE) as u32 & WS_VISIBLE != 0) != show {
        ShowWindow(h, if show { SW_SHOW } else { SW_HIDE });
    }
}
pub unsafe fn sync_actions(hwnd: HWND, s: &Ui) {
    let (connected, running) = {
        let v = s.worker.status.lock().unwrap();
        (
            v.connected,
            if s.preview {
                s.preview_running.get()
            } else {
                v.running
            },
        )
    };
    visibility(GetDlgItem(hwnd, CONNECT), !connected);
    visibility(GetDlgItem(hwnd, DISCONNECT), connected);
    visibility(GetDlgItem(hwnd, START), !running);
    visibility(GetDlgItem(hwnd, STOP), running);
    EnableWindow(GetDlgItem(hwnd, CONNECT), (!s.preview && !connected) as i32);
    EnableWindow(
        GetDlgItem(hwnd, DISCONNECT),
        (!s.preview && connected) as i32,
    );
    EnableWindow(GetDlgItem(hwnd, START), (s.preview || connected) as i32);
    EnableWindow(GetDlgItem(hwnd, STOP), 1);
}
unsafe fn line(dc: HDC, points: &[(i32, i32)], color: u32, width: i32) {
    let pen = CreatePen(PS_SOLID, width, color);
    let old = SelectObject(dc, pen);
    let pts = points
        .iter()
        .map(|&(x, y)| POINT { x, y })
        .collect::<Vec<_>>();
    Polyline(dc, pts.as_ptr(), pts.len() as i32);
    SelectObject(dc, old);
    DeleteObject(pen);
}
unsafe fn thumbnail(
    dc: HDC,
    idx: usize,
    settings: &lighting::Settings,
    selected: bool,
    hovered: bool,
    elapsed: f32,
) {
    let mut demo = if selected {
        settings.clone()
    } else {
        lighting::Settings {
            effect: idx,
            palette: 0,
            brightness: 0.9,
            ..Default::default()
        }
    };
    demo.effect = idx;
    let t = if hovered { elapsed } else { 0.45 };
    let input = lighting::LiveInput {
        audio: [0.45, 0.8],
        screen: std::array::from_fn(|i| lighting::hsv_for_preview(i as f32 / 90.)),
        cpu: 0.55,
        memory: 0.7,
        hour: 12,
        minute: 34,
    };
    let presses = [lighting::Press {
        x: 7.,
        y: 2.,
        time: (t / 2.5).floor() * 2.5,
    }];
    let frame = lighting::render_live(&demo, t * demo.speed, t, &presses, &input);
    rect(dc, 10, 8, 200, 48, BG, 0x252524, 7);
    static LED_INDICES: std::sync::OnceLock<Vec<usize>> = std::sync::OnceLock::new();
    let indices = LED_INDICES.get_or_init(|| {
        crate::layout::keys()
            .iter()
            .map(|key| {
                (0..90)
                    .find(|&i| lighting::coord(i) == (key.row, key.col))
                    .unwrap()
            })
            .collect()
    });
    for (key, &i) in crate::layout::keys().iter().zip(indices) {
        let c = frame[i];
        let x = 13 + (key.u * 12.).round() as i32;
        let y = 11 + key.row as i32 * 7;
        let w = (key.width * 12.).round() as i32 - 1;
        let fill = rgb(c.map(|v| 12 + ((v as u16 * 9) / 10) as u8));
        rect(dc, x, y, w, 6, fill, fill, 1);
    }
}
pub unsafe fn animate_thumbnail(hwnd: HWND, s: &Ui) {
    let id = s.hovered.get();
    if (400..400 + crate::library::PAGE_SIZE as i32).contains(&id) {
        InvalidateRect(GetDlgItem(hwnd, id), null(), 0);
    }
}
pub unsafe fn button(s: &Ui, d: &DRAWITEMSTRUCT) {
    // Compose the whole card off-screen: never expose its cleared background.
    let w = d.rcItem.right - d.rcItem.left;
    let h = d.rcItem.bottom - d.rcItem.top;
    let dc = CreateCompatibleDC(d.hDC);
    let bitmap = CreateCompatibleBitmap(d.hDC, w, h);
    if dc.is_null() || bitmap.is_null() {
        if !bitmap.is_null() {
            DeleteObject(bitmap);
        }
        if !dc.is_null() {
            DeleteDC(dc);
        }
        button_contents(s, d);
        return;
    }
    let old = SelectObject(dc, bitmap);
    let mut buffered = *d;
    buffered.hDC = dc;
    SetBkMode(dc, TRANSPARENT as i32);
    button_contents(s, &buffered);
    BitBlt(d.hDC, 0, 0, w, h, dc, 0, 0, SRCCOPY);
    SelectObject(dc, old);
    DeleteObject(bitmap);
    DeleteDC(dc);
}
unsafe fn button_contents(s: &Ui, d: &DRAWITEMSTRUCT) {
    let id = d.CtlID as i32;
    let dc = d.hDC;
    let w = d.rcItem.right - d.rcItem.left;
    let h = d.rcItem.bottom - d.rcItem.top;
    let settings = s.worker.settings.lock().unwrap().clone();
    let effect = (400..400 + crate::library::PAGE_SIZE as i32).contains(&id);
    let palette = (600..603).contains(&id);
    let idx = if effect {
        s.library.borrow().slot((id - 400) as usize)
    } else {
        None
    };
    let selected = idx == Some(settings.effect)
        || (palette && settings.palette == (id - 600) as usize)
        || (id == AUTOSTART && s.autostart.get());
    let disabled = d.itemState & ODS_DISABLED != 0;
    let pressed = d.itemState & ODS_SELECTED != 0;
    let focused = d.itemState & ODS_FOCUS != 0;
    let hovered = s.hovered.get() == id;
    let primary = id == START || id == STOP;
    let fill = if primary {
        if disabled {
            0x3a492c
        } else if pressed {
            0x24c49e
        } else {
            LIME
        }
    } else if selected {
        0x1b241c
    } else if pressed || hovered {
        0x242422
    } else {
        PANEL
    };
    // Paint the entire owner-draw surface so rounded corners blend cleanly.
    let brush = CreateSolidBrush(
        if effect
            || palette
            || matches!(
                id,
                FIRST | SECOND | PREVIOUS | NEXT | CATEGORY_FILTER | AUTOSTART
            )
        {
            PANEL
        } else {
            BG
        },
    );
    FillRect(dc, &d.rcItem, brush);
    DeleteObject(brush);
    rect(
        dc,
        0,
        0,
        w,
        h,
        fill,
        if selected {
            LIME
        } else if focused {
            WHITE
        } else if hovered {
            0x55524d
        } else {
            LINE
        },
        12,
    );
    let mut buf = [0u16; 128];
    let n = GetWindowTextW(d.hwndItem, buf.as_mut_ptr(), 128);
    let title = String::from_utf16_lossy(&buf[..n as usize]);
    let color = if disabled {
        0x666560
    } else if primary {
        BG
    } else if selected {
        LIME
    } else {
        WHITE
    };
    if let Some(idx) = idx {
        let scale = (s.font_px.get() as f32 / 16.).max(1.);
        let thumb_w = (w - 24).max(1);
        let thumb_h = (thumb_w as f32 * 48. / 200.).round() as i32;
        let saved = SaveDC(dc);
        SetMapMode(dc, MM_ANISOTROPIC);
        SetWindowExtEx(dc, 200, 48, null_mut());
        SetViewportExtEx(dc, thumb_w, thumb_h, null_mut());
        SetWindowOrgEx(dc, 10, 8, null_mut());
        SetViewportOrgEx(dc, 12, 10, null_mut());
        thumbnail(
            dc,
            idx,
            &settings,
            selected,
            hovered,
            s.started.elapsed().as_secs_f32(),
        );
        RestoreDC(dc, saved);
        let title_y = thumb_h + 20;
        let title_size = (14. * scale).round() as i32;
        label(
            dc,
            &title,
            12,
            title_y,
            if title.chars().count() > 23 {
                title_size - 2
            } else {
                title_size
            },
            color,
            true,
        );
        let font = CreateFontW(
            -(11. * scale).round() as i32,
            0,
            0,
            0,
            400,
            0,
            0,
            0,
            DEFAULT_CHARSET as u32,
            0,
            0,
            CLEARTYPE_QUALITY as u32,
            0,
            wide("Segoe UI").as_ptr(),
        );
        let old = SelectObject(dc, font);
        SetTextColor(dc, MUTED);
        let mut rr = RECT {
            left: 12,
            top: title_y + title_size + 6,
            right: w - 12,
            bottom: h - 8,
        };
        let t = wide(lighting::DESCRIPTIONS[idx]);
        DrawTextW(dc, t.as_ptr(), -1, &mut rr, DT_WORDBREAK | DT_END_ELLIPSIS);
        SelectObject(dc, old);
        DeleteObject(font);
        if selected {
            rect(dc, w - 16, title_y + 5, 5, 5, LIME, LIME, 5);
        }
    } else if id == FIRST || id == SECOND {
        let saved = SaveDC(dc);
        SetMapMode(dc, MM_ANISOTROPIC);
        SetWindowExtEx(dc, 150, 36, null_mut());
        SetViewportExtEx(dc, w, h, null_mut());
        let c = if id == FIRST {
            settings.first
        } else {
            settings.second
        };
        rect(dc, 11, 10, 16, 16, rgb(c), rgb(c), 5);
        let caption = if settings.palette == 2 && settings.effect != 9 {
            format!("{} · {}", if id == FIRST { "1" } else { "2" }, title)
        } else {
            format!("Seç · {}", title)
        };
        label(dc, &caption, 37, 9, 12, color, false);
        RestoreDC(dc, saved);
    } else {
        let mut rr = RECT {
            left: 6,
            top: 0,
            right: w - 6,
            bottom: h,
        };
        SelectObject(
            dc,
            if !primary && !s.scaled_font.get().is_null() {
                s.scaled_font.get()
            } else {
                s.font
            },
        );
        SetBkMode(dc, TRANSPARENT as i32);
        SetTextColor(dc, color);
        let t = wide(&title);
        DrawTextW(
            dc,
            t.as_ptr(),
            -1,
            &mut rr,
            DT_CENTER | DT_VCENTER | DT_SINGLELINE,
        );
    }
}
pub unsafe fn paint(hwnd: HWND, s: &Ui) {
    let l = super::responsive::current(hwnd, s);
    let mut ps: PAINTSTRUCT = std::mem::zeroed();
    let target = BeginPaint(hwnd, &mut ps);
    let dc = CreateCompatibleDC(target);
    let bitmap = CreateCompatibleBitmap(target, l.w, l.h);
    let old = SelectObject(dc, bitmap);
    rect(dc, 0, 0, l.w, l.h, BG, BG, 0);
    SetViewportOrgEx(dc, 0, -l.scroll, null_mut());
    DrawIconEx(
        dc,
        27,
        18,
        crate::brand::icon(76),
        76,
        76,
        0,
        null_mut(),
        DI_NORMAL,
    );
    label(dc, "RS7", 124, 33, 40, WHITE, true);
    rect(dc, 228, 32, 1, 50, LINE, LINE, 0);
    label(dc, "ATK RS7 Air", 252, 38, 16, WHITE, true);
    label(
        dc,
        "Senin renklerin. Senin kontrolün.",
        252,
        62,
        12,
        MUTED,
        false,
    );
    let (connected, running) = {
        let v = s.worker.status.lock().unwrap();
        (
            v.connected,
            if s.preview {
                s.preview_running.get()
            } else {
                v.running
            },
        )
    };
    let (bx, by) = if l.narrow { (24, 118) } else { (l.w - 534, 42) };
    rect(dc, bx, by, 206, 30, PANEL, LINE, 14);
    rect(
        dc,
        bx + 14,
        by + 12,
        5,
        5,
        if running { LIME } else { MUTED },
        if running { LIME } else { MUTED },
        5,
    );
    label(
        dc,
        if s.preview {
            "Önizleme"
        } else if running {
            "Efekt çalışıyor"
        } else if connected {
            "USB ile bağlı"
        } else {
            "Bağlantı bekleniyor"
        },
        bx + 28,
        by + 5,
        12,
        MUTED,
        false,
    );
    let a = l.hero;
    rect(dc, a.x, a.y, a.w, a.h, PANEL, LINE, 22);
    label(
        dc,
        "Senin klavyen, senin ritmin.",
        a.x + 24,
        a.y + 17,
        if l.narrow { 19 } else { 23 },
        WHITE,
        true,
    );
    let settings = s.worker.settings.lock().unwrap().clone();
    let badge_w = if l.narrow { 160 } else { 184 };
    let badge_x = a.x + a.w - badge_w - 20;
    rect(dc, badge_x, a.y + 18, badge_w, 28, 0x203024, 0x203024, 12);
    label(
        dc,
        EFFECTS[settings.effect],
        badge_x + 10,
        a.y + 23,
        11,
        LIME,
        false,
    );
    let mut frame = s.worker.status.lock().unwrap().frame;
    if s.preview && s.preview_running.get() {
        let t = s.started.elapsed().as_secs_f32();
        let input = lighting::LiveInput {
            audio: [(t * 1.7).sin().abs() * 0.8, (t * 1.3).sin().abs() * 0.7],
            screen: std::array::from_fn(|i| lighting::hsv_for_preview(i as f32 / 90. + t * 0.1)),
            cpu: 0.57,
            memory: 0.68,
            hour: 12,
            minute: 34,
        };
        let mut presses = s.preview_presses.borrow_mut();
        presses.retain(|p| t - p.time < 11.);
        frame = lighting::render_live(&settings, t * settings.speed, t, &presses, &input);
    }

    let k = l.keyboard;
    let saved = SaveDC(dc);
    SetMapMode(dc, MM_ANISOTROPIC);
    SetWindowExtEx(dc, 1016, 280, null_mut());
    SetViewportExtEx(dc, k.w, k.h, null_mut());
    SetWindowOrgEx(dc, 64, 170, null_mut());
    SetViewportOrgEx(dc, k.x, k.y - l.scroll, null_mut());
    super::keyboard::paint(dc, &frame, settings.effect == 64);
    RestoreDC(dc, saved);
    let q = |v: i32| (v as f32 * l.scale).round() as i32;
    let a = l.library;
    rect(dc, a.x, a.y, a.w, a.h, PANEL, LINE, 20);
    label(
        dc,
        "Efekt kütüphanesi",
        a.x + 16,
        a.y + q(16),
        q(20),
        WHITE,
        true,
    );
    let lib = s.library.borrow();
    rect(
        dc,
        a.x + q(200),
        a.y + q(18),
        q(34),
        q(22),
        0x222820,
        0x222820,
        8,
    );
    label(
        dc,
        &lib.filtered.len().to_string(),
        a.x + q(208),
        a.y + q(21),
        q(11),
        LIME,
        true,
    );
    label(
        dc,
        &format!("Sayfa {} / {}", lib.page + 1, lib.pages()),
        a.x + a.w / 2 - q(32),
        a.y + a.h - q(29),
        q(12),
        MUTED,
        false,
    );
    if lib.filtered.is_empty() {
        label(
            dc,
            "Eşleşen efekt yok",
            a.x + 24,
            a.y + q(116),
            q(20),
            WHITE,
            true,
        );
    }
    drop(lib);
    rect(
        dc,
        a.x + q(16),
        a.y + q(52),
        a.w - q(244),
        q(34),
        BG,
        if GetFocus() == GetDlgItem(hwnd, SEARCH) {
            LIME
        } else {
            LINE
        },
        10,
    );
    label(dc, "⌕", a.x + q(25), a.y + q(58), q(18), MUTED, false);
    let a = l.settings;
    rect(dc, a.x, a.y, a.w, a.h, PANEL, LINE, 20);
    let saved = SaveDC(dc);
    SetMapMode(dc, MM_ANISOTROPIC);
    SetWindowExtEx(dc, 372, 316, null_mut());
    SetViewportExtEx(dc, q(372), q(316), null_mut());
    SetWindowOrgEx(dc, 764, 484, null_mut());
    SetViewportOrgEx(dc, a.x + (a.w - q(372)) / 2, a.y - l.scroll, null_mut());
    label(dc, "Efekt ayarları", 790, 502, 20, WHITE, true);
    {
        let font = CreateFontW(
            -12,
            0,
            0,
            0,
            400,
            0,
            0,
            0,
            DEFAULT_CHARSET as u32,
            0,
            0,
            CLEARTYPE_QUALITY as u32,
            0,
            wide("Segoe UI").as_ptr(),
        );
        let old = SelectObject(dc, font);
        SetTextColor(dc, MUTED);
        let mut r = RECT {
            left: 790,
            top: 534,
            right: 1106,
            bottom: 565,
        };
        let text = wide(lighting::DESCRIPTIONS[settings.effect]);
        DrawTextW(
            dc,
            text.as_ptr(),
            -1,
            &mut r,
            DT_WORDBREAK | DT_END_ELLIPSIS,
        );
        SelectObject(dc, old);
        DeleteObject(font);
    }
    label(dc, "Renk seçimi", 790, 679, 12, MUTED, false);
    if matches!(settings.effect, 10 | 57 | 58) || (settings.palette == 0 && settings.effect != 9) {
        label(
            dc,
            if settings.effect == 10 {
                "Bu modda ışıklar kapalı."
            } else if matches!(settings.effect, 57 | 58) {
                "Renkler ekranından alınır."
            } else {
                "Renkler otomatik akar."
            },
            790,
            753,
            13,
            MUTED,
            false,
        );
    }

    label(
        dc,
        if s.preview {
            "Önizleme: Windows ayarı değişmez."
        } else {
            "Oturum açıldığında RS7 açılır."
        },
        790,
        850,
        11,
        MUTED,
        false,
    );
    RestoreDC(dc, saved);
    line(
        dc,
        &[(24, l.content - 36), (l.w - 24, l.content - 36)],
        0x20201f,
        1,
    );
    SetViewportOrgEx(dc, 0, 0, null_mut());
    BitBlt(target, 0, 0, l.w, l.h, dc, 0, 0, SRCCOPY);
    SelectObject(dc, old);
    DeleteObject(bitmap);
    DeleteDC(dc);
    EndPaint(hwnd, &ps);
}
pub unsafe fn slider(d: &NMCUSTOMDRAW) -> LRESULT {
    if d.hdr.code != NM_CUSTOMDRAW {
        return 0;
    }
    if d.dwDrawStage == CDDS_PREPAINT {
        return CDRF_NOTIFYITEMDRAW as LRESULT;
    }
    if d.dwDrawStage == CDDS_ITEMPREPAINT {
        let r = d.rc;
        match d.dwItemSpec as u32 {
            2 => {
                rect(
                    d.hdc,
                    r.left,
                    r.top,
                    r.right - r.left,
                    r.bottom - r.top,
                    if IsWindowEnabled(d.hdr.hwndFrom) != 0 {
                        LIME
                    } else {
                        MUTED
                    },
                    if IsWindowEnabled(d.hdr.hwndFrom) != 0 {
                        LIME
                    } else {
                        MUTED
                    },
                    12,
                );
                return CDRF_SKIPDEFAULT as LRESULT;
            }
            3 => {
                rect(
                    d.hdc,
                    r.left,
                    r.top,
                    r.right - r.left,
                    r.bottom - r.top,
                    LINE,
                    LINE,
                    6,
                );
                return CDRF_SKIPDEFAULT as LRESULT;
            }
            _ => {}
        }
    }
    CDRF_DODEFAULT as LRESULT
}

pub unsafe fn refresh_library(hwnd: HWND, s: &Ui) {
    let mut query = [0u16; 256];
    let n = GetWindowTextW(GetDlgItem(hwnd, SEARCH), query.as_mut_ptr(), 256);
    let (visible, page, pages, category) = {
        let mut lib = s.library.borrow_mut();
        lib.update(&String::from_utf16_lossy(&query[..n as usize]));
        (lib.visible().to_vec(), lib.page, lib.pages(), lib.category)
    };
    for slot in 0..crate::library::PAGE_SIZE {
        let h = GetDlgItem(hwnd, 400 + slot as i32);
        if let Some(&i) = visible.get(slot) {
            text(h, EFFECTS[i]);
            ShowWindow(h, SW_SHOW);
        } else {
            ShowWindow(h, SW_HIDE);
        }
    }
    text(
        GetDlgItem(hwnd, CATEGORY_FILTER),
        &format!(
            "Kategori: {} ▾",
            category.map(|i| lighting::CATEGORIES[i]).unwrap_or("Tümü")
        ),
    );
    EnableWindow(GetDlgItem(hwnd, PREVIOUS), (page > 0) as i32);
    EnableWindow(GetDlgItem(hwnd, NEXT), (page + 1 < pages) as i32);
    RedrawWindow(hwnd, null(), null_mut(), RDW_INVALIDATE | RDW_ALLCHILDREN);
}

pub unsafe fn mode_controls(hwnd: HWND, s: &Ui) {
    let settings = s.worker.settings.lock().unwrap().clone();
    let effect = settings.effect;
    let speed = !matches!(effect, 9 | 10 | 48 | 53 | 54 | 57 | 58 | 59 | 60 | 61);
    EnableWindow(GetDlgItem(hwnd, SPEED), speed as i32);
    let palette = !matches!(effect, 9 | 10 | 57 | 58);
    for id in 600..603 {
        EnableWindow(GetDlgItem(hwnd, id), palette as i32);
    }
    let custom = !matches!(effect, 10 | 57 | 58) && (effect == 9 || settings.palette != 0);
    visibility(s.first, custom);
    visibility(s.second, custom && effect != 9 && settings.palette == 2);
    EnableWindow(s.first, custom as i32);
    EnableWindow(s.second, (custom && settings.palette == 2) as i32);
}
