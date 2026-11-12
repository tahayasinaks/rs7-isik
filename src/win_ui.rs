use crate::{
    lighting::{self, EFFECTS, PALETTES},
    worker::{Command, Worker, CAPTURE},
};
use std::sync::atomic::Ordering;
use std::{
    cell::RefCell,
    ptr::{null, null_mut},
    sync::{mpsc::SyncSender, OnceLock},
    time::Instant,
};
use windows_sys::Win32::{
    Foundation::*,
    Graphics::Gdi::*,
    System::LibraryLoader::GetModuleHandleW,
    UI::{Controls::Dialogs::*, Controls::*, Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
};
static KEYS: OnceLock<SyncSender<(u32, Instant)>> = OnceLock::new();
thread_local! {static HELD:RefCell<[bool;512]>=const{RefCell::new([false;512])};}
const CONNECT: i32 = 101;
const START: i32 = 102;
const STOP: i32 = 103;
const DISCONNECT: i32 = 104;
const EFFECT: i32 = 201;
const PALETTE: i32 = 202;
const BRIGHT: i32 = 203;
const SPEED: i32 = 204;
const FIRST: i32 = 205;
const SECOND: i32 = 206;
const TBM_GETPOS: u32 = WM_USER;
struct Ui {
    worker: Worker,
    status: HWND,
    brightness: HWND,
    speed: HWND,
    first: HWND,
    second: HWND,
    font: HFONT,
    last_status: String,
}
fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().chain(Some(0)).collect()
}
unsafe fn text(hwnd: HWND, value: &str) {
    SetWindowTextW(hwnd, wide(value).as_ptr());
}
unsafe fn child(
    parent: HWND,
    class: &str,
    value: &str,
    id: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    style: u32,
    font: HFONT,
) -> HWND {
    let window = CreateWindowExW(
        0,
        wide(class).as_ptr(),
        wide(value).as_ptr(),
        WS_CHILD | WS_VISIBLE | style,
        x,
        y,
        w,
        h,
        parent,
        id as usize as HMENU,
        GetModuleHandleW(null()),
        null(),
    );
    SendMessageW(window, WM_SETFONT, font as usize, 1);
    window
}
unsafe fn build(hwnd: HWND, state: &mut Ui) {
    let f = state.font;
    child(hwnd, "STATIC", "RS7 Işık · Rust", 0, 24, 20, 750, 36, 0, f);
    child(
        hwnd,
        "STATIC",
        "ATK RS7 Air — USB ile bağlayın; ATK HUB ve eski Electron uygulamasını kapatın.",
        0,
        24,
        60,
        980,
        28,
        0,
        f,
    );
    for (id, label, x, w) in [
        (CONNECT, "Klavyeyi bağla", 24, 180),
        (START, "Başlat", 216, 130),
        (STOP, "Efekti durdur", 358, 230),
        (DISCONNECT, "Bağlantıyı kes", 600, 180),
    ] {
        child(
            hwnd,
            "BUTTON",
            label,
            id,
            x,
            104,
            w,
            40,
            BS_PUSHBUTTON as u32 | WS_TABSTOP,
            f,
        );
    }
    child(hwnd, "STATIC", "Işık modu", 0, 24, 166, 120, 25, 0, f);
    let combo = child(
        hwnd,
        "COMBOBOX",
        "",
        EFFECT,
        148,
        160,
        300,
        400,
        CBS_DROPDOWNLIST as u32 | WS_TABSTOP | WS_VSCROLL,
        f,
    );
    for label in EFFECTS {
        SendMessageW(combo, CB_ADDSTRING, 0, wide(label).as_ptr() as isize);
    }
    SendMessageW(combo, CB_SETCURSEL, 0, 0);
    child(hwnd, "STATIC", "Renk düzeni", 0, 480, 166, 145, 25, 0, f);
    let palette = child(
        hwnd,
        "COMBOBOX",
        "",
        PALETTE,
        626,
        160,
        350,
        200,
        CBS_DROPDOWNLIST as u32 | WS_TABSTOP | WS_VSCROLL,
        f,
    );
    for label in PALETTES {
        SendMessageW(palette, CB_ADDSTRING, 0, wide(label).as_ptr() as isize);
    }
    SendMessageW(palette, CB_SETCURSEL, 0, 0);
    state.brightness = child(hwnd, "STATIC", "Parlaklık: %60", 0, 24, 224, 180, 28, 0, f);
    let brightness = child(
        hwnd,
        "msctls_trackbar32",
        "",
        BRIGHT,
        205,
        215,
        240,
        45,
        WS_TABSTOP,
        f,
    );
    SendMessageW(brightness, TBM_SETRANGE, 1, (100 << 16) as isize);
    SendMessageW(brightness, TBM_SETPOS, 1, 60);
    state.speed = child(hwnd, "STATIC", "Hız: 1.00×", 0, 480, 224, 160, 28, 0, f);
    let speed = child(
        hwnd,
        "msctls_trackbar32",
        "",
        SPEED,
        636,
        215,
        340,
        45,
        WS_TABSTOP,
        f,
    );
    SendMessageW(speed, TBM_SETRANGE, 1, ((300 << 16) | 20) as isize);
    SendMessageW(speed, TBM_SETPOS, 1, 100);
    state.first = child(
        hwnd,
        "BUTTON",
        "1. renk: #1EB4FF",
        FIRST,
        24,
        278,
        250,
        40,
        WS_TABSTOP,
        f,
    );
    state.second = child(
        hwnd,
        "BUTTON",
        "2. renk: #C814FF",
        SECOND,
        290,
        278,
        250,
        40,
        WS_TABSTOP,
        f,
    );
    child(
        hwnd,
        "STATIC",
        "Renk seçince özel renk düzeni etkinleşir. İkinci renk, iki renk geçişinde kullanılır.",
        0,
        24,
        330,
        980,
        25,
        0,
        f,
    );
    state.status = child(
        hwnd,
        "STATIC",
        "Klavye bağlı değil.",
        0,
        24,
        378,
        980,
        50,
        0,
        f,
    );
    child(hwnd,"STATIC","Tuşa basınca dalga: bastığınız tuştan halka yayılır. Tuşlar kaydedilmez veya internete gönderilmez.",0,24,638,990,24,0,f);
    child(hwnd,"STATIC","Küçültme: efekt sürer. X ile çıkış: özel efekt sonlandırılır. Fn algılanmaz; diğer klavyeler de tetikleyebilir.",0,24,666,990,24,0,f);
    SetTimer(hwnd, 1, 50, None);
}
unsafe fn choose(hwnd: HWND, state: &mut Ui, second: bool) {
    let mut settings = state.worker.settings.lock().unwrap().clone();
    let value = if second {
        settings.second
    } else {
        settings.first
    };
    let mut custom = [0u32; 16];
    let mut dialog: CHOOSECOLORW = std::mem::zeroed();
    dialog.lStructSize = std::mem::size_of::<CHOOSECOLORW>() as u32;
    dialog.hwndOwner = hwnd;
    dialog.rgbResult = value[0] as u32 | ((value[1] as u32) << 8) | ((value[2] as u32) << 16);
    dialog.lpCustColors = custom.as_mut_ptr();
    dialog.Flags = CC_FULLOPEN | CC_RGBINIT;
    if ChooseColorW(&mut dialog) != 0 {
        let c = [
            dialog.rgbResult as u8,
            (dialog.rgbResult >> 8) as u8,
            (dialog.rgbResult >> 16) as u8,
        ];
        if second {
            settings.second = c;
            settings.palette = 2;
        } else {
            settings.first = c;
            if settings.palette == 0 {
                settings.palette = 1;
            }
        }
        SendMessageW(GetDlgItem(hwnd, PALETTE), CB_SETCURSEL, settings.palette, 0);
        *state.worker.settings.lock().unwrap() = settings;
        text(
            if second { state.second } else { state.first },
            &format!(
                "{}. renk: #{:02X}{:02X}{:02X}",
                if second { 2 } else { 1 },
                c[0],
                c[1],
                c[2]
            ),
        );
    }
}
unsafe fn paint(hwnd: HWND, state: &Ui) {
    let mut ps: PAINTSTRUCT = std::mem::zeroed();
    let dc = BeginPaint(hwnd, &mut ps);
    let frame = state.worker.status.lock().unwrap().frame;
    let old = SelectObject(dc, state.font);
    SetBkMode(dc, TRANSPARENT as i32);
    const LABELS: [[&str; 15]; 6] = [
        [
            "Esc", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12",
            "Ins", "Del",
        ],
        [
            "`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "+", "Sil", "End",
        ],
        [
            "Tab", "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "[", "]", "\\", "PgUp",
        ],
        [
            "Caps", "A", "S", "D", "F", "G", "H", "J", "K", "L", ";", "'", "", "Enter", "PgDn",
        ],
        [
            "Shift", "Z", "X", "C", "V", "B", "N", "M", ",", ".", "/", "Shift", "", "↑", "",
        ],
        [
            "Ctrl", "Win", "Alt", "", "", "Space", "", "", "", "Alt", "Fn", "←", "", "↓", "→",
        ],
    ];
    for (i, rgb) in frame.iter().enumerate() {
        let (row, col) = lighting::coord(i);
        let x = 24 + col as i32 * 64;
        let y = 438 + row as i32 * 31;
        let rect = RECT {
            left: x,
            top: y,
            right: x + 60,
            bottom: y + 27,
        };
        let color = rgb[0] as u32 | ((rgb[1] as u32) << 8) | ((rgb[2] as u32) << 16);
        let brush = CreateSolidBrush(color);
        FillRect(dc, &rect, brush);
        DeleteObject(brush);
        let bright = rgb.iter().map(|v| *v as u32).sum::<u32>() > 420;
        SetTextColor(dc, if bright { 0 } else { 0xffffff });
        let label = wide(LABELS[row as usize][col as usize]);
        TextOutW(dc, x + 5, y + 3, label.as_ptr(), (label.len() - 1) as i32);
    }
    SelectObject(dc, old);
    EndPaint(hwnd, &ps);
}
unsafe extern "system" fn hook(code: i32, w: WPARAM, l: LPARAM) -> LRESULT {
    if code >= 0 {
        let key = &*(l as *const KBDLLHOOKSTRUCT);
        let scan = key.scanCode
            + if key.flags & LLKHF_EXTENDED != 0 {
                256
            } else {
                0
            };
        if scan < 512 && key.flags & LLKHF_INJECTED == 0 {
            HELD.with(|held| {
                let mut held = held.borrow_mut();
                if !CAPTURE.load(Ordering::Relaxed) {
                    held.fill(false);
                    return;
                }
                if w == WM_KEYUP as usize || w == WM_SYSKEYUP as usize {
                    held[scan as usize] = false;
                }
                if (w == WM_KEYDOWN as usize || w == WM_SYSKEYDOWN as usize) && !held[scan as usize]
                {
                    held[scan as usize] = true;
                    if let Some(tx) = KEYS.get() {
                        let _ = tx.try_send((scan, Instant::now()));
                    }
                }
            });
        }
    }
    CallNextHookEx(null_mut(), code, w, l)
}
unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, w: WPARAM, l: LPARAM) -> LRESULT {
    let pointer = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Ui;
    if pointer.is_null() {
        return DefWindowProcW(hwnd, msg, w, l);
    }
    let state = &mut *pointer;
    match msg {
        WM_COMMAND => {
            let id = (w & 0xffff) as i32;
            let command = match id {
                CONNECT => Some(Command::Connect),
                START => Some(Command::Start),
                STOP => Some(Command::Stop),
                DISCONNECT => Some(Command::Disconnect),
                _ => None,
            };
            if let Some(c) = command {
                let _ = state.worker.command.send(c);
            }
            if id == EFFECT || id == PALETTE {
                let mut s = state.worker.settings.lock().unwrap();
                let index = SendMessageW(GetDlgItem(hwnd, id), CB_GETCURSEL, 0, 0);
                if index >= 0 {
                    if id == EFFECT {
                        s.effect = index as usize;
                    } else {
                        s.palette = index as usize;
                    }
                }
            }
            if id == FIRST || id == SECOND {
                choose(hwnd, state, id == SECOND);
            }
            0
        }
        WM_HSCROLL => {
            let mut s = state.worker.settings.lock().unwrap();
            s.brightness = SendMessageW(GetDlgItem(hwnd, BRIGHT), TBM_GETPOS, 0, 0) as f32 / 100.;
            s.speed = SendMessageW(GetDlgItem(hwnd, SPEED), TBM_GETPOS, 0, 0) as f32 / 100.;
            text(
                state.brightness,
                &format!("Parlaklık: %{}", (s.brightness * 100.).round() as u32),
            );
            text(state.speed, &format!("Hız: {:.2}×", s.speed));
            0
        }
        WM_TIMER => {
            let status = state.worker.status.lock().unwrap();
            let label = format!(
                "{}  {}",
                status.message,
                if status.running {
                    format!("· {} kare/sn", status.fps)
                } else {
                    String::new()
                }
            );
            if label != state.last_status {
                text(state.status, &label);
                state.last_status = label;
            }
            EnableWindow(GetDlgItem(hwnd, CONNECT), (!status.connected) as i32);
            EnableWindow(
                GetDlgItem(hwnd, START),
                (status.connected && !status.running) as i32,
            );
            EnableWindow(GetDlgItem(hwnd, STOP), status.running as i32);
            EnableWindow(GetDlgItem(hwnd, DISCONNECT), status.connected as i32);
            drop(status);
            let rect = RECT {
                left: 20,
                top: 435,
                right: 1000,
                bottom: 630,
            };
            InvalidateRect(hwnd, &rect, 0);
            0
        }
        WM_PAINT => {
            paint(hwnd, state);
            0
        }
        WM_CLOSE => {
            KillTimer(hwnd, 1);
            state.worker.shutdown();
            if let Some(error) = &state.worker.status.lock().unwrap().close_error {
                MessageBoxW(hwnd,wide(&format!("Klavyeye kapatma komutu gönderilemedi: {error}\nIşıklar sürerse USB bağlantısını çıkarıp takın.")).as_ptr(),wide("USB bağlantısı").as_ptr(),MB_OK|MB_ICONWARNING);
            }
            DestroyWindow(hwnd);
            0
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, w, l),
    }
}
pub fn run() {
    unsafe {
        let instance = GetModuleHandleW(null());
        let class = wide("RS7IsikRustWindow");
        let mut wc: WNDCLASSW = std::mem::zeroed();
        wc.lpfnWndProc = Some(wndproc);
        wc.hInstance = instance;
        wc.lpszClassName = class.as_ptr();
        wc.hCursor = LoadCursorW(null_mut(), IDC_ARROW);
        wc.hbrBackground = (COLOR_WINDOW + 1) as usize as HBRUSH;
        if RegisterClassW(&wc) == 0 {
            return;
        }
        let controls = INITCOMMONCONTROLSEX {
            dwSize: std::mem::size_of::<INITCOMMONCONTROLSEX>() as u32,
            dwICC: ICC_BAR_CLASSES,
        };
        InitCommonControlsEx(&controls);
        let worker = Worker::new();
        let _ = KEYS.set(worker.keys.clone());
        let font = CreateFontW(
            -18,
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
            0,
            0,
            wide("Segoe UI").as_ptr(),
        );
        let mut ui = Box::new(Ui {
            worker,
            status: null_mut(),
            brightness: null_mut(),
            speed: null_mut(),
            first: null_mut(),
            second: null_mut(),
            font,
            last_status: String::new(),
        });
        let hwnd = CreateWindowExW(
            0,
            class.as_ptr(),
            wide("RS7 Işık — Rust 0.3.0").as_ptr(),
            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            1050,
            755,
            null_mut(),
            null_mut(),
            instance,
            null(),
        );
        if hwnd.is_null() {
            return;
        }
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, (&mut *ui) as *mut Ui as isize);
        build(hwnd, &mut ui);
        let keyboard_hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook), instance, 0);
        if keyboard_hook.is_null() {
            MessageBoxW(
                hwnd,
                wide("Tuşa tepki dinleyicisi başlatılamadı. Diğer efektler kullanılabilir.")
                    .as_ptr(),
                wide("Tuş dinleyicisi").as_ptr(),
                MB_OK | MB_ICONWARNING,
            );
        }
        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            if IsDialogMessageW(hwnd, &msg) == 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
        CAPTURE.store(false, Ordering::Relaxed);
        if !keyboard_hook.is_null() {
            UnhookWindowsHookEx(keyboard_hook);
        }
        ui.worker.shutdown();
        DeleteObject(font);
    }
}
