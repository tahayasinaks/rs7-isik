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
    for slot in 0..12 {
        btn(
            hwnd,
            s,
            400 + slot,
            "",
            40 + (slot % 3) * 232,
            550 + (slot / 3) * 54,
            220,
            48,
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
    for (i, t) in ["Çok renk", "Tek renk", "İki renk"].iter().enumerate() {
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
    refresh_library(hwnd, s);
    mode_controls(hwnd, s);
    sync_actions(hwnd, s);
    SetTimer(hwnd, 1, 50, None);
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
unsafe fn symbol(dc: HDC, x: i32, y: i32, category: usize, heart: bool, color: u32) {
    if heart {
        line(
            dc,
            &[
                (x + 2, y + 7),
                (x + 5, y + 3),
                (x + 9, y + 3),
                (x + 12, y + 7),
                (x + 15, y + 3),
                (x + 19, y + 3),
                (x + 22, y + 7),
                (x + 21, y + 11),
                (x + 12, y + 20),
                (x + 3, y + 11),
                (x + 2, y + 7),
            ],
            color,
            1,
        );
        return;
    }
    match category {
        0 => {
            let old = SelectObject(dc, GetStockObject(NULL_BRUSH));
            let p = CreatePen(PS_SOLID, 1, color);
            let op = SelectObject(dc, p);
            Ellipse(dc, x + 5, y + 5, x + 19, y + 19);
            Ellipse(dc, x + 9, y + 9, x + 15, y + 15);
            SelectObject(dc, op);
            SelectObject(dc, old);
            DeleteObject(p);
        }
        1 => {
            line(dc, &[(x + 12, y + 1), (x + 12, y + 7)], color, 1);
            line(dc, &[(x + 12, y + 17), (x + 12, y + 23)], color, 1);
            line(dc, &[(x + 1, y + 12), (x + 7, y + 12)], color, 1);
            line(dc, &[(x + 17, y + 12), (x + 23, y + 12)], color, 1);
            rect(dc, x + 9, y + 9, 6, 6, color, color, 5);
        }
        2 => {
            for row in [5, 12, 19] {
                let pts = (0..24)
                    .map(|i| (x + i, y + row + ((i as f32 * 0.35).sin() * 2.) as i32))
                    .collect::<Vec<_>>();
                line(dc, &pts, color, 1);
            }
        }
        3 => {
            line(
                dc,
                &[
                    (x + 12, y + 1),
                    (x + 15, y + 9),
                    (x + 23, y + 12),
                    (x + 15, y + 15),
                    (x + 12, y + 23),
                    (x + 9, y + 15),
                    (x + 1, y + 12),
                    (x + 9, y + 9),
                    (x + 12, y + 1),
                ],
                color,
                1,
            );
        }
        4 => {
            line(
                dc,
                &[
                    (x + 12, y + 1),
                    (x + 23, y + 12),
                    (x + 12, y + 23),
                    (x + 1, y + 12),
                    (x + 12, y + 1),
                ],
                color,
                1,
            );
            line(
                dc,
                &[
                    (x + 12, y + 6),
                    (x + 18, y + 12),
                    (x + 12, y + 18),
                    (x + 6, y + 12),
                    (x + 12, y + 6),
                ],
                color,
                1,
            );
        }
        5 => {
            for (i, h) in [7, 14, 21, 12, 5].iter().enumerate() {
                rect(
                    dc,
                    x + i as i32 * 5,
                    y + (24 - h) / 2,
                    2,
                    *h,
                    color,
                    color,
                    2,
                );
            }
        }
        _ => {
            for (i, h) in [8, 15, 21].iter().enumerate() {
                rect(dc, x + 2 + i as i32 * 7, y + 22 - h, 4, *h, color, color, 2);
            }
        }
    }
}
pub unsafe fn button(s: &Ui, d: &DRAWITEMSTRUCT) {
    let id = d.CtlID as i32;
    let dc = d.hDC;
    let w = d.rcItem.right - d.rcItem.left;
    let h = d.rcItem.bottom - d.rcItem.top;
    let settings = s.worker.settings.lock().unwrap().clone();
    let effect = (400..412).contains(&id);
    let palette = (600..603).contains(&id);
    let idx = if effect {
        s.library.borrow().slot((id - 400) as usize)
    } else {
        None
    };
    let selected =
        idx == Some(settings.effect) || (palette && settings.palette == (id - 600) as usize);
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
        if effect || palette || matches!(id, FIRST | SECOND | PREVIOUS | NEXT | CATEGORY_FILTER) {
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
        rect(
            dc,
            10,
            8,
            32,
            32,
            if selected { 0x263a2a } else { 0x232321 },
            if selected { 0x263a2a } else { 0x232321 },
            9,
        );
        symbol(
            dc,
            14,
            12,
            lighting::CATEGORY[idx],
            idx == 64,
            if selected { LIME } else { 0xb5b3ae },
        );
        label(
            dc,
            &title,
            52,
            7,
            if title.chars().count() > 21 { 12 } else { 14 },
            color,
            true,
        );
        label(
            dc,
            lighting::CATEGORIES[lighting::CATEGORY[idx]],
            52,
            28,
            10,
            if selected { 0x8fc694 } else { MUTED },
            false,
        );
        if selected {
            rect(dc, w - 13, 9, 4, 4, LIME, LIME, 4);
        }
    } else if id == FIRST || id == SECOND {
        let c = if id == FIRST {
            settings.first
        } else {
            settings.second
        };
        rect(dc, 11, 10, 16, 16, rgb(c), rgb(c), 5);
        label(dc, &title, 37, 9, 13, color, false);
    } else {
        let mut rr = RECT {
            left: 6,
            top: 0,
            right: w - 6,
            bottom: h,
        };
        SelectObject(dc, s.font);
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
    let mut ps: PAINTSTRUCT = std::mem::zeroed();
    let target = BeginPaint(hwnd, &mut ps);
    let dc = CreateCompatibleDC(target);
    let bitmap = CreateCompatibleBitmap(target, 1160, 834);
    let old = SelectObject(dc, bitmap);
    rect(dc, 0, 0, 1160, 834, BG, BG, 0);
    DrawIconEx(
        dc,
        12,
        3,
        crate::brand::icon(106),
        106,
        106,
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
    let badge = if s.preview {
        "Önizleme"
    } else if running {
        "Efekt çalışıyor"
    } else if connected {
        "USB ile bağlı"
    } else {
        "Bağlantı bekleniyor"
    };
    rect(
        dc,
        626,
        42,
        206,
        30,
        if connected && !s.preview {
            0x1b241c
        } else {
            PANEL
        },
        LINE,
        14,
    );
    rect(
        dc,
        640,
        54,
        5,
        5,
        if running { LIME } else { MUTED },
        if running { LIME } else { MUTED },
        5,
    );
    label(
        dc,
        badge,
        654,
        47,
        12,
        if connected { LIME } else { MUTED },
        false,
    );
    rect(dc, 24, 112, 1112, 352, PANEL, LINE, 22);
    label(dc, "Senin klavyen, senin ritmin.", 48, 131, 23, WHITE, true);
    label(
        dc,
        if s.preview && lighting::reactive(s.worker.settings.lock().unwrap().effect) {
            "Tepkiyi görmek için klavyeye tıklayın."
        } else {
            "Her tuşta bir hareket, her harekette senin rengin."
        },
        450,
        138,
        12,
        MUTED,
        false,
    );
    let settings = s.worker.settings.lock().unwrap().clone();
    rect(dc, 946, 136, 164, 30, 0x203024, 0x203024, 14);
    label(
        dc,
        EFFECTS[settings.effect],
        958,
        141,
        if EFFECTS[settings.effect].chars().count() > 22 {
            10
        } else {
            12
        },
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
    super::keyboard::paint(dc, &frame, settings.effect == 64);
    rect(dc, 24, 484, 724, 316, PANEL, LINE, 20);
    rect(dc, 764, 484, 372, 316, PANEL, LINE, 20);
    label(dc, "Efekt kütüphanesi", 40, 502, 20, WHITE, true);
    {
        let lib = s.library.borrow();
        rect(dc, 224, 504, 30, 22, 0x222820, 0x222820, 8);
        label(
            dc,
            &lib.filtered.len().to_string(),
            231,
            507,
            11,
            LIME,
            true,
        );
        label(
            dc,
            &format!("Sayfa {} / {}", lib.page + 1, lib.pages()),
            334,
            773,
            12,
            MUTED,
            false,
        );
        if lib.filtered.is_empty() {
            label(dc, "Eşleşen efekt yok", 250, 631, 21, WHITE, true);
            label(
                dc,
                "Başka bir ad veya kategori deneyin.",
                233,
                662,
                13,
                MUTED,
                false,
            );
        }
    }
    rect(
        dc,
        272,
        500,
        256,
        34,
        BG,
        if GetFocus() == GetDlgItem(hwnd, SEARCH) {
            LIME
        } else {
            LINE
        },
        10,
    );
    {
        let old = SelectObject(dc, GetStockObject(NULL_BRUSH));
        let p = CreatePen(PS_SOLID, 1, MUTED);
        let op = SelectObject(dc, p);
        Ellipse(dc, 282, 510, 291, 519);
        MoveToEx(dc, 289, 517, null_mut());
        LineTo(dc, 294, 522);
        SelectObject(dc, op);
        SelectObject(dc, old);
        DeleteObject(p);
    }
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
    label(dc, "Renk düzeni", 790, 679, 12, MUTED, false);
    line(dc, &[(24, 804), (1136, 804)], 0x20201f, 1);
    // Clip child control areas to avoid flicker; WS_CLIPCHILDREN handles target rendering.
    BitBlt(target, 0, 0, 1160, 834, dc, 0, 0, SRCCOPY);
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
    for slot in 0..12 {
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
    let effect = s.worker.settings.lock().unwrap().effect;
    let speed = !matches!(effect, 9 | 10 | 48 | 53 | 54 | 57 | 58 | 59 | 60 | 61);
    EnableWindow(GetDlgItem(hwnd, SPEED), speed as i32);
    let palette = !matches!(effect, 9 | 10 | 57 | 58);
    for id in 600..603 {
        EnableWindow(GetDlgItem(hwnd, id), palette as i32);
    }
    EnableWindow(s.first, (!matches!(effect, 10 | 57 | 58)) as i32);
    EnableWindow(s.second, palette as i32);
}
