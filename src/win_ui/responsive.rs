use super::*;
#[derive(Clone, Copy, Debug)]
pub struct Area {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}
impl Area {
    pub fn rect(self) -> RECT {
        RECT {
            left: self.x,
            top: self.y,
            right: self.x + self.w,
            bottom: self.y + self.h,
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Layout {
    pub w: i32,
    pub h: i32,
    pub narrow: bool,
    pub hero: Area,
    pub keyboard: Area,
    pub library: Area,
    pub settings: Area,
    pub cols: i32,
    pub scale: f32,
    pub card_height: i32,
    pub content: i32,
    pub scroll: i32,
}
impl Layout {
    pub fn new(w: i32, h: i32, scroll: i32) -> Self {
        let w = w.max(640);
        let narrow = w < 1060;
        let top = if narrow { 170 } else { 112 };
        let kw = (w - 112).min(1500);
        let kh = kw * 280 / 1016;
        let hero = Area {
            x: 24,
            y: top,
            w: w - 48,
            h: kh + 76,
        };
        let keyboard = Area {
            x: (w - kw) / 2,
            y: top + 58,
            w: kw,
            h: kh,
        };
        let cols = if w < 820 { 2 } else { 3 };
        let rows = (6 + cols - 1) / cols;
        let scale = (w as f32 / 1160.).clamp(1., 1.8);
        let sw = (372. * scale).round() as i32;
        let lw = if narrow { w - 48 } else { w - 68 - sw };
        let cw = (lw - 32 - 12 * (cols - 1)) / cols;
        let card_height = ((cw - 24) as f32 * 48. / 200. + 62. * scale).ceil() as i32;
        let lh = ((100. * scale) as i32 + rows * (card_height + 12) + (42. * scale) as i32)
            .max((480. * scale) as i32);
        let library = Area {
            x: 24,
            y: top + hero.h + 20,
            w: lw,
            h: lh,
        };
        let settings = Area {
            x: if narrow { 24 } else { w - 24 - sw },
            y: if narrow {
                library.y + lh + 20
            } else {
                library.y
            },
            w: if narrow { w - 48 } else { sw },
            h: if narrow { (480. * scale) as i32 } else { lh },
        };
        let content = (library.y + library.h).max(settings.y + settings.h) + 42;
        Self {
            w,
            h,
            narrow,
            hero,
            keyboard,
            library,
            settings,
            cols,
            scale,
            card_height,
            content,
            scroll: scroll.clamp(0, (content - h).max(0)),
        }
    }
    pub fn card(self, slot: i32) -> Area {
        let gap = 12;
        let cw = (self.library.w - 32 - gap * (self.cols - 1)) / self.cols;
        Area {
            x: self.library.x + 16 + (slot % self.cols) * (cw + gap),
            y: self.library.y
                + (100. * self.scale) as i32
                + (slot / self.cols) * (self.card_height + 12),
            w: cw,
            h: self.card_height,
        }
    }
    pub fn key_point(self, x: i32, y: i32) -> Option<(i32, i32)> {
        let y = y + self.scroll;
        let k = self.keyboard;
        if x < k.x || x >= k.x + k.w || y < k.y || y >= k.y + k.h {
            return None;
        }
        Some((64 + (x - k.x) * 1016 / k.w, 170 + (y - k.y) * 280 / k.h))
    }
}
pub unsafe fn current(hwnd: HWND, s: &Ui) -> Layout {
    let mut r: RECT = std::mem::zeroed();
    GetClientRect(hwnd, &mut r);
    Layout::new(r.right, r.bottom, s.scroll.get())
}
pub unsafe fn apply(hwnd: HWND, s: &Ui) {
    let l = current(hwnd, s);
    s.scroll.set(l.scroll);
    let mov = |h: HWND, x, y, w, hgt| {
        MoveWindow(h, x, y - l.scroll, w, hgt, 0);
    };
    let by = if l.narrow { 112 } else { 38 };
    for id in [CONNECT, DISCONNECT] {
        mov(GetDlgItem(hwnd, id), l.w - 304, by, 140, 42);
    }
    for id in [START, STOP] {
        mov(GetDlgItem(hwnd, id), l.w - 152, by, 124, 42);
    }
    let q = |v: i32| (v as f32 * l.scale).round() as i32;
    let size = q(16);
    if size != s.font_px.get() {
        let font = CreateFontW(
            -size,
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
        if !font.is_null() {
            for id in [
                SEARCH,
                CATEGORY_FILTER,
                PREVIOUS,
                NEXT,
                211,
                212,
                600,
                601,
                602,
                FIRST,
                SECOND,
                AUTOSTART,
                UPDATE,
            ] {
                SendMessageW(GetDlgItem(hwnd, id), WM_SETFONT, font as usize, 0);
            }
            for h in [s.brightness, s.speed] {
                SendMessageW(h, WM_SETFONT, font as usize, 0);
            }
            let previous = s.scaled_font.replace(font);
            s.font_px.set(size);
            if !previous.is_null() {
                DeleteObject(previous);
            }
        }
    }
    let a = l.library;
    mov(
        GetDlgItem(hwnd, SEARCH),
        a.x + q(44),
        a.y + q(57),
        a.w - q(284),
        q(25),
    );
    mov(
        GetDlgItem(hwnd, CATEGORY_FILTER),
        a.x + a.w - q(204),
        a.y + q(52),
        q(188),
        q(34),
    );
    for slot in 0..6 {
        let c = l.card(slot);
        mov(GetDlgItem(hwnd, 400 + slot), c.x, c.y, c.w, c.h);
    }
    mov(
        GetDlgItem(hwnd, PREVIOUS),
        a.x + 16,
        a.y + a.h - q(36),
        q(104),
        q(28),
    );
    mov(
        GetDlgItem(hwnd, NEXT),
        a.x + a.w - q(128),
        a.y + a.h - q(36),
        q(112),
        q(28),
    );
    let x = l.settings.x + (l.settings.w - q(372)) / 2;
    let y = l.settings.y;
    for (id, dy) in [(211, 90), (212, 148)] {
        mov(GetDlgItem(hwnd, id), x + q(26), y + q(dy), q(200), q(22));
    }
    for (id, dy) in [(BRIGHT, 112), (SPEED, 170)] {
        mov(GetDlgItem(hwnd, id), x + q(22), y + q(dy), q(318), q(28));
    }
    mov(s.brightness, x + q(268), y + q(90), q(72), q(24));
    mov(s.speed, x + q(268), y + q(148), q(72), q(24));
    for i in 0..3 {
        mov(
            GetDlgItem(hwnd, 600 + i),
            x + q(26 + i * 104),
            y + q(214),
            q(98),
            q(32),
        );
    }
    mov(s.first, x + q(26), y + q(260), q(150), q(36));
    mov(s.second, x + q(186), y + q(260), q(150), q(36));
    mov(
        GetDlgItem(hwnd, AUTOSTART),
        x + q(26),
        y + q(320),
        q(310),
        q(40),
    );
    mov(
        GetDlgItem(hwnd, UPDATE),
        x + q(26),
        y + q(400),
        q(310),
        q(40),
    );
    mov(s.status, 40, l.content - 30, l.w - 80, 22);
    super::scrollbar::update(hwnd, s);
    RedrawWindow(hwnd, null(), null_mut(), RDW_INVALIDATE | RDW_ALLCHILDREN);
}
pub unsafe fn scroll(hwnd: HWND, s: &Ui, delta: i32) {
    let l = current(hwnd, s);
    s.scroll
        .set((l.scroll + delta).clamp(0, (l.content - l.h).max(0)));
    apply(hwnd, s);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layouts_fit_and_reflow() {
        for w in [640, 768, 1024, 1060, 1366, 1920, 2560] {
            let l = Layout::new(w, 700, 0);
            for a in [l.hero, l.keyboard, l.library, l.settings] {
                assert!(a.x >= 0 && a.x + a.w <= w);
                assert!(a.y + a.h <= l.content);
            }
            for i in 0..6 {
                let c = l.card(i);
                assert!(c.x >= l.library.x && c.x + c.w <= l.library.x + l.library.w);
                assert!(c.y + c.h <= l.library.y + l.library.h - 40);
            }
            if l.narrow {
                assert!(l.settings.y >= l.library.y + l.library.h);
            } else {
                assert!(l.settings.x >= l.library.x + l.library.w);
            }
        }
    }
    #[test]
    fn scaled_keyboard_hit_and_scroll_agree() {
        for w in [640, 1160, 1920] {
            let l = Layout::new(w, 600, 80);
            let (x, y) = (
                l.keyboard.x + l.keyboard.w / 2,
                l.keyboard.y + l.keyboard.h / 2 - l.scroll,
            );
            let p = l.key_point(x, y).unwrap();
            assert!((p.0 - 572).abs() < 3);
            assert!((p.1 - 310).abs() < 3);
            assert!(l.key_point(0, 0).is_none());
            let end = Layout::new(w, 600, i32::MAX);
            assert_eq!(end.scroll, end.content - 600);
        }
    }
}
fn window_path() -> Option<std::path::PathBuf> {
    std::env::var_os("APPDATA").map(|p| {
        std::path::PathBuf::from(p)
            .join("RS7Isik")
            .join("window-v1.txt")
    })
}
pub unsafe fn save_window(hwnd: HWND, s: &Ui) {
    if s.preview || IsWindowVisible(hwnd) == 0 {
        return;
    }
    let Some(path) = window_path() else {
        return;
    };
    let mut p: WINDOWPLACEMENT = std::mem::zeroed();
    p.length = std::mem::size_of::<WINDOWPLACEMENT>() as u32;
    if GetWindowPlacement(hwnd, &mut p) == 0 {
        return;
    }
    let r = p.rcNormalPosition;
    let value = format!(
        "RS7WINDOW1 {} {} {} {} {}",
        r.left,
        r.top,
        r.right - r.left,
        r.bottom - r.top,
        (p.showCmd == SW_SHOWMAXIMIZED as u32 || p.flags & WPF_RESTORETOMAXIMIZED != 0) as u8
    );
    if let Some(dir) = path.parent() {
        if std::fs::create_dir_all(dir).is_ok() {
            let _ = std::fs::write(path, value);
        }
    }
}
pub unsafe fn restore_window(hwnd: HWND, s: &Ui) -> bool {
    if s.preview {
        return false;
    }
    let Some(path) = window_path() else {
        return false;
    };
    let Ok(raw) = std::fs::read_to_string(path) else {
        return false;
    };
    let f: Vec<_> = raw.split_whitespace().collect();
    if f.len() != 6 || f[0] != "RS7WINDOW1" {
        return false;
    }
    let Some(v) = f[1..]
        .iter()
        .map(|v| v.parse::<i32>().ok())
        .collect::<Option<Vec<_>>>()
    else {
        return false;
    };
    if !(680..=10000).contains(&v[2])
        || !(520..=10000).contains(&v[3])
        || v[0].abs() > 30000
        || v[1].abs() > 30000
    {
        return false;
    }
    let r = RECT {
        left: v[0],
        top: v[1],
        right: v[0] + v[2],
        bottom: v[1] + v[3],
    };
    let monitor = MonitorFromRect(&r, MONITOR_DEFAULTTONEAREST);
    let mut info: MONITORINFO = std::mem::zeroed();
    info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
    if GetMonitorInfoW(monitor, &mut info) == 0 {
        return false;
    }
    let mut p: WINDOWPLACEMENT = std::mem::zeroed();
    p.length = std::mem::size_of::<WINDOWPLACEMENT>() as u32;
    p.showCmd = SW_SHOWNORMAL as u32;
    let work = info.rcWork;
    let width = v[2].min(work.right - work.left);
    let height = v[3].min(work.bottom - work.top);
    let x = v[0].clamp(work.left, (work.right - width).max(work.left));
    let y = v[1].clamp(work.top, (work.bottom - height).max(work.top));
    p.rcNormalPosition = RECT {
        left: x,
        top: y,
        right: x + width,
        bottom: y + height,
    };
    SetWindowPlacement(hwnd, &p);
    v[4] == 1
}
