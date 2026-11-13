mod keyboard;
mod theme;
use crate::{
    lighting::{self, EFFECTS},
    tray,
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
    System::{LibraryLoader::GetModuleHandleW, Threading::CreateMutexW},
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
const SEARCH: i32 = 207;
const CATEGORY_FILTER: i32 = 208;
const PREVIOUS: i32 = 209;
const NEXT: i32 = 210;
const TBM_GETPOS: u32 = WM_USER;
struct Ui {
    worker: Worker,
    status: HWND,
    brightness: HWND,
    speed: HWND,
    first: HWND,
    second: HWND,
    font: HFONT,
    last_status: RefCell<String>,
    tray: RefCell<Option<tray::Tray>>,
    taskbar_created: u32,
    preview: bool,
    started: Instant,
    preview_running: std::cell::Cell<bool>,
    library: RefCell<crate::library::Library>,
    preview_presses: RefCell<Vec<lighting::Press>>,
    hovered: std::cell::Cell<i32>,
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
    theme::build(hwnd, state);
}
unsafe fn choose(hwnd: HWND, state: &Ui, second: bool) {
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
            &format!("#{:02X}{:02X}{:02X}", c[0], c[1], c[2]),
        );
    }
}
unsafe fn paint(hwnd: HWND, state: &Ui) {
    theme::paint(hwnd, state);
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
    // Win32 modal dialogs and menus can re-enter this callback. Use shared state,
    // not an exclusive &mut Ui spanning calls into the Windows message pump.
    let state = &*pointer;
    if msg == state.taskbar_created && msg != 0 {
        if let Some(tray) = &mut *state.tray.borrow_mut() {
            tray.recreate();
            if !tray.available {
                ShowWindow(hwnd, SW_SHOW);
            }
        }
        return 0;
    }
    match msg {
        WM_ERASEBKGND => 1,
        WM_NOTIFY => theme::slider(&*(l as *const NMCUSTOMDRAW)),
        WM_DRAWITEM => {
            theme::button(state, &*(l as *const DRAWITEMSTRUCT));
            1
        }
        WM_SETCURSOR => {
            let child = w as HWND;
            let id = if GetParent(child) == hwnd {
                GetDlgCtrlID(child)
            } else {
                0
            };
            let old = state.hovered.replace(id);
            if id != old {
                if old > 0 {
                    InvalidateRect(GetDlgItem(hwnd, old), null(), 0);
                }
                if id > 0 {
                    InvalidateRect(child, null(), 0);
                }
            }
            DefWindowProcW(hwnd, msg, w, l)
        }
        WM_CTLCOLORSTATIC | WM_CTLCOLORBTN | WM_CTLCOLOREDIT | WM_CTLCOLORLISTBOX => {
            let value = l as HWND == state.brightness || l as HWND == state.speed;
            let background =
                if value || matches!(GetDlgCtrlID(l as HWND), BRIGHT | SPEED | 211 | 212) {
                    theme::PANEL
                } else {
                    theme::BG
                };
            SetTextColor(
                w as HDC,
                if l as HWND == state.status {
                    0x96938e
                } else if value {
                    0x32f4c6
                } else {
                    theme::WHITE
                },
            );
            SetBkColor(w as HDC, background);
            SetDCBrushColor(w as HDC, background);
            GetStockObject(DC_BRUSH) as LRESULT
        }

        tray::SHOW => {
            ShowWindow(hwnd, SW_RESTORE);
            SetForegroundWindow(hwnd);
            0
        }
        tray::MESSAGE => {
            match l as u32 {
                WM_LBUTTONUP | WM_LBUTTONDBLCLK => {
                    PostMessageW(hwnd, tray::SHOW, 0, 0);
                }
                WM_RBUTTONUP | WM_CONTEXTMENU => {
                    let (connected, running) = {
                        let s = state.worker.status.lock().unwrap();
                        (s.connected, s.running)
                    };
                    match tray::menu(hwnd, connected, running) {
                        tray::OPEN => {
                            PostMessageW(hwnd, tray::SHOW, 0, 0);
                        }
                        tray::START => {
                            let _ = state.worker.command.send(Command::Start);
                        }
                        tray::STOP => {
                            let _ = state.worker.command.send(Command::Stop);
                        }
                        tray::QUIT => {
                            PostMessageW(hwnd, tray::EXIT, 0, 0);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            0
        }
        WM_SYSCOMMAND if (w as u32 & 0xfff0) == SC_MINIMIZE => {
            hide(hwnd, state);
            0
        }
        WM_COMMAND => {
            let id = (w & 0xffff) as i32;
            if id == SEARCH && ((w >> 16) as u32) == EN_CHANGE {
                theme::refresh_library(hwnd, state);
                return 0;
            }
            if id == CATEGORY_FILTER {
                let menu = CreatePopupMenu();
                AppendMenuW(menu, MF_STRING, 800, wide("Tümü").as_ptr());
                for (i, c) in lighting::CATEGORIES.iter().enumerate() {
                    AppendMenuW(menu, MF_STRING, 801 + i, wide(c).as_ptr());
                }
                let mut r: RECT = std::mem::zeroed();
                GetWindowRect(GetDlgItem(hwnd, CATEGORY_FILTER), &mut r);
                let chosen = TrackPopupMenu(
                    menu,
                    TPM_RETURNCMD | TPM_LEFTALIGN,
                    r.left,
                    r.bottom,
                    0,
                    hwnd,
                    null(),
                );
                DestroyMenu(menu);
                if chosen >= 800 && chosen <= 807 {
                    state.library.borrow_mut().category = if chosen == 800 {
                        None
                    } else {
                        Some((chosen - 801) as usize)
                    };
                    theme::refresh_library(hwnd, state);
                }
                return 0;
            }
            if id == PREVIOUS || id == NEXT {
                {
                    let mut lib = state.library.borrow_mut();
                    if id == PREVIOUS {
                        lib.page = lib.page.saturating_sub(1);
                    } else {
                        lib.page = (lib.page + 1).min(lib.pages() - 1);
                    }
                }
                theme::refresh_library(hwnd, state);
                return 0;
            }
            if (400..400 + crate::library::PAGE_SIZE as i32).contains(&id) {
                let effect = state.library.borrow().slot((id - 400) as usize);
                if let Some(effect) = effect {
                    state.worker.settings.lock().unwrap().effect = effect;
                    state.preview_presses.borrow_mut().clear();
                }
                theme::mode_controls(hwnd, state);
                RedrawWindow(hwnd, null(), null_mut(), RDW_INVALIDATE | RDW_ALLCHILDREN);
                return 0;
            }
            if (600..603).contains(&id) {
                state.worker.settings.lock().unwrap().palette = (id - 600) as usize;
                RedrawWindow(hwnd, null(), null_mut(), RDW_INVALIDATE | RDW_ALLCHILDREN);
                return 0;
            }
            if state.preview && [CONNECT, START, STOP, DISCONNECT].contains(&id) {
                if id == START {
                    state.preview_running.set(true);
                }
                if id == STOP {
                    state.preview_running.set(false);
                }
                theme::sync_actions(hwnd, state);
                RedrawWindow(hwnd, null(), null_mut(), RDW_INVALIDATE | RDW_ALLCHILDREN);
                return 0;
            }

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
            RedrawWindow(hwnd, null(), null_mut(), RDW_INVALIDATE | RDW_ALLCHILDREN);
            0
        }
        WM_LBUTTONDOWN if state.preview => {
            let x = (l as u32 & 65535) as i32;
            let y = ((l as u32 >> 16) & 65535) as i32;
            if let Some((logical_x, logical_y)) = keyboard::hit(x, y) {
                let mut presses = state.preview_presses.borrow_mut();
                if presses.len() == 32 {
                    presses.remove(0);
                }
                presses.push(lighting::Press {
                    x: logical_x,
                    y: logical_y,
                    time: state.started.elapsed().as_secs_f32(),
                });
                SetFocus(hwnd);
            }
            0
        }
        WM_KEYDOWN if state.preview => {
            let scan = ((l as u32 >> 16) & 255) + if l as u32 & (1 << 24) != 0 { 256 } else { 0 };
            if l as u32 & (1 << 30) == 0 {
                if let Some((x, y)) = lighting::position(scan) {
                    let mut presses = state.preview_presses.borrow_mut();
                    if presses.len() == 32 {
                        presses.remove(0);
                    }
                    presses.push(lighting::Press {
                        x,
                        y,
                        time: state.started.elapsed().as_secs_f32(),
                    });
                }
            }
            0
        }
        WM_HSCROLL => {
            let mut s = state.worker.settings.lock().unwrap();
            s.brightness = SendMessageW(GetDlgItem(hwnd, BRIGHT), TBM_GETPOS, 0, 0) as f32 / 100.;
            s.speed = SendMessageW(GetDlgItem(hwnd, SPEED), TBM_GETPOS, 0, 0) as f32 / 100.;
            text(
                state.brightness,
                &format!("%{}", (s.brightness * 100.).round() as u32),
            );
            text(state.speed, &format!("{:.2}×", s.speed));
            0
        }
        WM_TIMER if state.preview => {
            if IsWindowVisible(hwnd) != 0 && IsIconic(hwnd) == 0 {
                let hero = RECT {
                    left: 24,
                    top: 120,
                    right: 1136,
                    bottom: 464,
                };
                InvalidateRect(hwnd, &hero, 0);
                theme::animate_thumbnail(hwnd, state);
            }
            0
        }
        WM_TIMER => {
            let status = state.worker.status.lock().unwrap();
            let label = format!(
                "{}  {}  {}",
                status.message,
                status.source_note,
                if status.running {
                    format!("· {} kare/sn", status.fps)
                } else {
                    String::new()
                }
            );
            if label != *state.last_status.borrow() {
                text(state.status, &label);
                *state.last_status.borrow_mut() = label;
                let header = RECT {
                    left: 0,
                    top: 0,
                    right: 1160,
                    bottom: 110,
                };
                InvalidateRect(hwnd, &header, 0);
            }
            drop(status);
            theme::sync_actions(hwnd, state);
            let rect = RECT {
                left: 24,
                top: 120,
                right: 1136,
                bottom: 464,
            };
            if IsWindowVisible(hwnd) != 0 && IsIconic(hwnd) == 0 {
                InvalidateRect(hwnd, &rect, 0);
                theme::animate_thumbnail(hwnd, state);
            }
            0
        }
        WM_PAINT => {
            paint(hwnd, state);
            0
        }
        WM_CLOSE => {
            if state.preview {
                DestroyWindow(hwnd);
            } else {
                hide(hwnd, state);
            }
            0
        }
        WM_QUERYENDSESSION => 1,
        WM_ENDSESSION if w != 0 => {
            if !state.preview {
                let _ = crate::preferences::save(&state.worker.settings.lock().unwrap());
            }
            state.worker.shutdown();
            if let Some(icon) = &mut *state.tray.borrow_mut() {
                icon.remove();
            }
            DestroyWindow(hwnd);
            0
        }
        tray::EXIT => {
            KillTimer(hwnd, 1);
            let saved = if state.preview {
                Ok(())
            } else {
                let settings = state.worker.settings.lock().unwrap().clone();
                crate::preferences::save(&settings)
            };
            if let Err(error) = saved {
                MessageBoxW(
                    hwnd,
                    wide(&format!("Ayarlar kaydedilemedi: {error}")).as_ptr(),
                    wide("Ayar kaydı").as_ptr(),
                    MB_OK | MB_ICONWARNING,
                );
            }
            state.worker.shutdown();
            if let Some(icon) = &mut *state.tray.borrow_mut() {
                icon.remove();
            }
            let close_error = state.worker.status.lock().unwrap().close_error.clone();
            if let Some(error) = close_error {
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
unsafe fn hide(hwnd: HWND, state: &Ui) {
    if state.tray.borrow().as_ref().is_some_and(|t| t.available) {
        ShowWindow(hwnd, SW_HIDE);
    } else {
        ShowWindow(hwnd, SW_MINIMIZE);
    }
}
pub fn run() {
    unsafe {
        let preview = std::env::args().any(|a| a == "--design-preview")
            || std::env::current_exe().ok().is_some_and(|p| {
                p.file_stem()
                    .is_some_and(|n| n.to_string_lossy().contains("Onizleme"))
            });
        let preview_class = format!("RS7DesignPreview.{}", env!("CARGO_PKG_VERSION"));
        let preview_mutex = format!("Local\\RS7.DesignPreview.{}", env!("CARGO_PKG_VERSION"));
        let class_name = if preview {
            preview_class.as_str()
        } else {
            "RS7IsikRustWindow"
        };
        let mutex = CreateMutexW(
            null(),
            0,
            wide(if preview {
                preview_mutex.as_str()
            } else {
                "Local\\RS7IsikRust.SingleInstance"
            })
            .as_ptr(),
        );
        if !mutex.is_null() && GetLastError() == ERROR_ALREADY_EXISTS {
            let existing = FindWindowW(wide(class_name).as_ptr(), null());
            if !existing.is_null() {
                PostMessageW(existing, tray::SHOW, 0, 0);
            }
            CloseHandle(mutex);
            return;
        }

        let instance = GetModuleHandleW(null());
        let class = wide(class_name);
        let mut wc: WNDCLASSW = std::mem::zeroed();
        wc.lpfnWndProc = Some(wndproc);
        wc.hInstance = instance;
        wc.lpszClassName = class.as_ptr();
        wc.hCursor = LoadCursorW(null_mut(), IDC_ARROW);
        wc.hbrBackground = null_mut();
        wc.hIcon = crate::brand::icon(32);
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
            -16,
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
            preview,
            started: Instant::now(),
            preview_running: std::cell::Cell::new(true),
            library: RefCell::new(crate::library::Library::default()),
            preview_presses: RefCell::new(Vec::new()),
            hovered: std::cell::Cell::new(0),
            status: null_mut(),
            brightness: null_mut(),
            speed: null_mut(),
            first: null_mut(),
            second: null_mut(),
            font,
            last_status: RefCell::new(String::new()),
            tray: RefCell::new(None),
            taskbar_created: RegisterWindowMessageW(wide("TaskbarCreated").as_ptr()),
        });
        let hwnd = CreateWindowExW(
            0,
            class.as_ptr(),
            wide(crate::brand::NAME).as_ptr(),
            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_CLIPCHILDREN,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            1176,
            872,
            null_mut(),
            null_mut(),
            instance,
            null(),
        );
        if hwnd.is_null() {
            return;
        }
        SendMessageW(
            hwnd,
            WM_SETICON,
            ICON_SMALL as usize,
            crate::brand::icon(16) as isize,
        );
        SendMessageW(
            hwnd,
            WM_SETICON,
            ICON_BIG as usize,
            crate::brand::icon(48) as isize,
        );
        if preview {
            let mut s = ui.worker.settings.lock().unwrap();
            s.effect = 65;
            s.speed = 1.0;
            s.palette = 2;
            s.brightness = 0.8;
            s.first = [198, 244, 50];
            s.second = [255, 255, 255];
        }

        build(hwnd, &mut ui);
        if preview {
            text(GetDlgItem(hwnd, SEARCH), "tsunami");
            theme::refresh_library(hwnd, &ui);
        }

        SetWindowLongPtrW(hwnd, GWLP_USERDATA, (&mut *ui) as *mut Ui as isize);

        let dark: i32 = 1;
        windows_sys::Win32::Graphics::Dwm::DwmSetWindowAttribute(
            hwnd,
            20,
            &dark as *const _ as *const _,
            4,
        );
        if !preview {
            *ui.tray.borrow_mut() = Some(tray::Tray::new(hwnd));
        }
        let keyboard_hook = if preview {
            null_mut()
        } else {
            SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook), instance, 0)
        };
        if keyboard_hook.is_null() && !preview {
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
        if !mutex.is_null() {
            CloseHandle(mutex);
        }
    }
}
