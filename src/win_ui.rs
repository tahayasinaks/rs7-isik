mod category_popup;
mod color_picker;
mod keyboard;
mod responsive;
mod scrollbar;
mod theme;
mod update_notice;
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
    UI::{Controls::*, Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
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
const AUTOSTART: i32 = 213;
const UPDATE: i32 = 214;
const TBM_GETPOS: u32 = WM_USER;
struct Ui {
    worker: Worker,
    status: HWND,
    brightness: HWND,
    speed: HWND,
    first: HWND,
    second: HWND,
    font: HFONT,
    scaled_font: std::cell::Cell<HFONT>,
    font_px: std::cell::Cell<i32>,
    last_status: RefCell<String>,
    tray: RefCell<Option<tray::Tray>>,
    taskbar_created: u32,
    preview: bool,
    started: Instant,
    preview_running: std::cell::Cell<bool>,
    library: RefCell<crate::library::Library>,
    preview_presses: RefCell<Vec<lighting::Press>>,
    hovered: std::cell::Cell<i32>,
    scroll: std::cell::Cell<i32>,
    scroll_hover: std::cell::Cell<bool>,
    scroll_drag: std::cell::Cell<Option<(i32, i32)>>,
    autostart: std::cell::Cell<bool>,
    updates: crate::updates::Shared,
    update_caption: RefCell<String>,
    notified_version: RefCell<String>,
    last_update_check: std::cell::Cell<Instant>,
    saved_settings: RefCell<lighting::Settings>,
    pending_settings: RefCell<Option<(lighting::Settings, Instant)>>,
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
    if let Some(c) = color_picker::show(hwnd, value, state.font) {
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
        WM_EXITSIZEMOVE => {
            responsive::save_window(hwnd, state);
            0
        }
        WM_GETMINMAXINFO => {
            let limits = &mut *(l as *mut MINMAXINFO);
            limits.ptMinTrackSize = POINT { x: 680, y: 520 };
            0
        }
        WM_SIZE => {
            if w != SIZE_MINIMIZED as usize {
                responsive::apply(hwnd, state);
            }
            0
        }
        WM_VSCROLL => {
            let mut info: SCROLLINFO = std::mem::zeroed();
            info.cbSize = std::mem::size_of::<SCROLLINFO>() as u32;
            info.fMask = SIF_TRACKPOS;
            GetScrollInfo(hwnd, SB_VERT, &mut info);
            let current = responsive::current(hwnd, state);
            let target = match (w & 65535) as i32 {
                SB_LINEUP => current.scroll - 48,
                SB_LINEDOWN => current.scroll + 48,
                SB_PAGEUP => current.scroll - current.h + 64,
                SB_PAGEDOWN => current.scroll + current.h - 64,
                SB_THUMBTRACK | SB_THUMBPOSITION => info.nTrackPos,
                SB_TOP => 0,
                SB_BOTTOM => current.content,
                _ => current.scroll,
            };
            responsive::scroll(hwnd, state, target - current.scroll);
            0
        }
        WM_MOUSEWHEEL => {
            let delta = ((w >> 16) & 65535) as i16 as i32;
            responsive::scroll(hwnd, state, -delta * 96 / 120);
            0
        }
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
            if IsWindowVisible(hwnd) == 0 {
                let maximized = responsive::restore_window(hwnd, state);
                ShowWindow(
                    hwnd,
                    if maximized {
                        SW_SHOWMAXIMIZED
                    } else {
                        SW_RESTORE
                    },
                );
            } else {
                ShowWindow(hwnd, SW_RESTORE);
            }
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
            if id == UPDATE {
                if state.preview {
                    update_notice::show(hwnd, "1.2.2 (örnek)", true, state.font);
                    return 0;
                }
                let value = state.updates.lock().unwrap().clone();
                match value {
                    crate::updates::State::Available(r) => {
                        crate::updates::download(state.updates.clone(), r)
                    }
                    crate::updates::State::Current | crate::updates::State::Error(_) => {
                        state.last_update_check.set(Instant::now());
                        crate::updates::check(state.updates.clone())
                    }
                    _ => {}
                }
                return 0;
            }
            if id == AUTOSTART {
                let next = !state.autostart.get();
                if state.preview {
                    state.autostart.set(next);
                } else {
                    match crate::preferences::save(&state.worker.settings.lock().unwrap())
                        .and_then(|_| crate::startup::set(next))
                    {
                        Ok(()) => {
                            state.autostart.set(next);
                            if next {
                                let _ = state.worker.command.send(Command::AutoStart);
                            }
                        }
                        Err(error) => {
                            MessageBoxW(
                                hwnd,
                                wide(&format!(
                                    "Windows başlangıç tercihi değiştirilemedi.\n{error}"
                                ))
                                .as_ptr(),
                                wide("RS7 · Başlangıç ayarı").as_ptr(),
                                MB_OK | MB_ICONERROR,
                            );
                        }
                    }
                }
                text(
                    GetDlgItem(hwnd, AUTOSTART),
                    if state.autostart.get() {
                        "Windows ile başlat · Açık"
                    } else {
                        "Windows ile başlat · Kapalı"
                    },
                );
                InvalidateRect(GetDlgItem(hwnd, AUTOSTART), null(), 0);
                return 0;
            }
            if id == SEARCH && ((w >> 16) as u32) == EN_CHANGE {
                theme::refresh_library(hwnd, state);
                return 0;
            }
            if id == CATEGORY_FILTER {
                let category = state.library.borrow().category;
                let font = if state.scaled_font.get().is_null() {
                    state.font
                } else {
                    state.scaled_font.get()
                };
                if let Some(index) = category_popup::show(
                    hwnd,
                    GetDlgItem(hwnd, CATEGORY_FILTER),
                    category,
                    font,
                    responsive::current(hwnd, state).scale,
                ) {
                    state.library.borrow_mut().category =
                        if index == 0 { None } else { Some(index - 1) };
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
                theme::mode_controls(hwnd, state);
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
                theme::mode_controls(hwnd, state);
            }
            RedrawWindow(hwnd, null(), null_mut(), RDW_INVALIDATE | RDW_ALLCHILDREN);
            0
        }
        WM_LBUTTONDOWN if state.preview => {
            let x = (l as u32 & 65535) as i32;
            let y = ((l as u32 >> 16) & 65535) as i32;
            if let Some((logical_x, logical_y)) = responsive::current(hwnd, state)
                .key_point(x, y)
                .and_then(|(x, y)| keyboard::hit(x, y))
            {
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
            update_ui(hwnd, state);
            if IsWindowVisible(hwnd) != 0 && IsIconic(hwnd) == 0 {
                let mut hero = responsive::current(hwnd, state).hero.rect();
                OffsetRect(&mut hero, 0, -state.scroll.get());
                InvalidateRect(hwnd, &hero, 0);
                theme::animate_thumbnail(hwnd, state);
            }
            0
        }
        WM_TIMER => {
            persist_changes(state);
            update_ui(hwnd, state);
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
            let mut rect = responsive::current(hwnd, state).hero.rect();
            OffsetRect(&mut rect, 0, -state.scroll.get());
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
            responsive::save_window(hwnd, state);
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
            responsive::save_window(hwnd, state);
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
        let background = !preview && std::env::args().any(|a| a == "--autostart");
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
            if !existing.is_null() && !background {
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
        let saved_settings = worker.settings.lock().unwrap().clone();

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
            saved_settings: RefCell::new(saved_settings),
            pending_settings: RefCell::new(None),
            updates: std::sync::Arc::new(std::sync::Mutex::new(if preview {
                crate::updates::State::Current
            } else {
                crate::updates::State::Checking
            })),
            update_caption: RefCell::new(String::new()),
            notified_version: RefCell::new(String::new()),
            last_update_check: std::cell::Cell::new(Instant::now()),
            worker,
            preview,
            started: Instant::now(),
            preview_running: std::cell::Cell::new(true),
            library: RefCell::new(crate::library::Library::default()),
            preview_presses: RefCell::new(Vec::new()),
            hovered: std::cell::Cell::new(0),
            scroll: std::cell::Cell::new(0),
            scroll_hover: std::cell::Cell::new(false),
            scroll_drag: std::cell::Cell::new(None),
            autostart: std::cell::Cell::new(if preview {
                false
            } else {
                crate::startup::enabled().unwrap_or(false)
            }),
            status: null_mut(),
            brightness: null_mut(),
            speed: null_mut(),
            first: null_mut(),
            second: null_mut(),
            font,
            scaled_font: std::cell::Cell::new(null_mut()),
            font_px: std::cell::Cell::new(0),
            last_status: RefCell::new(String::new()),
            tray: RefCell::new(None),
            taskbar_created: RegisterWindowMessageW(wide("TaskbarCreated").as_ptr()),
        });
        let hwnd = CreateWindowExW(
            0,
            class.as_ptr(),
            wide(crate::brand::NAME).as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_CLIPCHILDREN,
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
            s.effect = 64;
            s.speed = 1.0;
            s.palette = 2;
            s.brightness = 0.8;
            s.first = [198, 244, 50];
            s.second = [255, 255, 255];
        }

        build(hwnd, &mut ui);
        if preview {
            text(GetDlgItem(hwnd, SEARCH), "kalp şekli");
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

        if !preview {
            if ui.autostart.get() {
                let _ = ui.worker.command.send(Command::AutoStart);
            }
            crate::updates::check(ui.updates.clone());
        }
        if background {
            responsive::apply(hwnd, &ui);
            if !ui.tray.borrow().as_ref().is_some_and(|t| t.available) {
                ShowWindow(hwnd, SW_SHOWMINNOACTIVE);
            }
        } else {
            let maximized = responsive::restore_window(hwnd, &ui);
            responsive::apply(hwnd, &ui);
            ShowWindow(hwnd, if maximized { SW_SHOWMAXIMIZED } else { SW_SHOW });
        }
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
        if !ui.scaled_font.get().is_null() {
            DeleteObject(ui.scaled_font.get());
        }
        DeleteObject(font);
        if !mutex.is_null() {
            CloseHandle(mutex);
        }
    }
}
fn persist_changes(s: &Ui) {
    let settings = s.worker.settings.lock().unwrap().clone();
    if settings == *s.saved_settings.borrow() {
        s.pending_settings.borrow_mut().take();
        return;
    }
    let mut pending = s.pending_settings.borrow_mut();
    match pending.as_mut() {
        Some((value, at)) if *value == settings => {
            if at.elapsed() >= std::time::Duration::from_millis(750) {
                match crate::preferences::save(&settings) {
                    Ok(()) => {
                        *s.saved_settings.borrow_mut() = settings;
                        *pending = None;
                    }
                    Err(_) => {
                        *at = Instant::now();
                    }
                }
            }
        }
        _ => *pending = Some((settings, Instant::now())),
    }
}
unsafe fn update_ui(hwnd: HWND, s: &Ui) {
    use crate::updates::State;
    if !s.preview && s.last_update_check.get().elapsed() >= std::time::Duration::from_secs(15 * 60)
    {
        let busy = matches!(
            *s.updates.lock().unwrap(),
            State::Checking | State::Downloading(_) | State::Ready(_) | State::Installing
        );
        if !busy {
            s.last_update_check.set(Instant::now());
            crate::updates::check(s.updates.clone());
        }
    }
    let value = s.updates.lock().unwrap().clone();
    if let State::Available(r) = &value {
        if !s.preview && *s.notified_version.borrow() != r.version {
            *s.notified_version.borrow_mut() = r.version.clone();
            if update_notice::show(hwnd, &r.version, false, s.font) {
                crate::updates::download(s.updates.clone(), r.clone());
            }
        }
    }
    let (caption, enabled) = match &value {
        State::Checking => ("Güncelleme kontrol ediliyor…".into(), false),
        State::Current => (
            if s.preview {
                "Güncelleme · Önizleme".into()
            } else {
                format!("v{} · Güncellemeleri kontrol et", env!("CARGO_PKG_VERSION"))
            },
            true,
        ),
        State::Available(r) => (format!("v{} · İndir ve kur", r.version), true),
        State::Downloading(p) => (format!("Güncelleme indiriliyor · %{p}"), false),
        State::Ready(_) | State::Installing => ("Kurulum başlatılıyor…".into(), false),
        State::Error(e) => {
            let _ = e;
            ("Kontrol edilemedi · Tekrar dene".into(), true)
        }
    };
    if caption != *s.update_caption.borrow() {
        text(GetDlgItem(hwnd, UPDATE), &caption);
        EnableWindow(GetDlgItem(hwnd, UPDATE), enabled as i32);
        *s.update_caption.borrow_mut() = caption;
    }
    if let State::Ready(path) = value {
        if !s.preview {
            *s.updates.lock().unwrap() = State::Installing;
            if let Err(e) = crate::preferences::save(&s.worker.settings.lock().unwrap()) {
                *s.updates.lock().unwrap() = State::Error(format!("Ayarlar kaydedilemedi: {e}"));
                return;
            }
            match std::process::Command::new(&path)
                .arg(format!("/UPDATEPID={}", std::process::id()))
                .spawn()
            {
                Ok(_) => {
                    PostMessageW(hwnd, tray::EXIT, 0, 0);
                }
                Err(e) => {
                    *s.updates.lock().unwrap() = State::Error(format!("Kurulum açılamadı: {e}"));
                    MessageBoxW(
                        hwnd,
                        wide(&format!("Kurulum açılamadı: {e}")).as_ptr(),
                        wide("RS7 Light · Güncelleme").as_ptr(),
                        MB_OK | MB_ICONERROR,
                    );
                }
            }
        }
    }
}
