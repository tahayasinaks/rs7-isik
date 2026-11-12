use std::ptr::{null, null_mut};
use windows_sys::Win32::{
    Foundation::*,
    UI::{Shell::*, WindowsAndMessaging::*},
};
pub const MESSAGE: u32 = WM_APP + 10;
pub const SHOW: u32 = WM_APP + 11;
pub const EXIT: u32 = WM_APP + 12;
pub const OPEN: u32 = 301;
pub const START: u32 = 302;
pub const STOP: u32 = 303;
pub const QUIT: u32 = 304;
fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}
pub struct Tray {
    pub available: bool,
    data: NOTIFYICONDATAW,
}
impl Tray {
    pub unsafe fn new(hwnd: HWND) -> Self {
        let mut data: NOTIFYICONDATAW = std::mem::zeroed();
        data.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
        data.hWnd = hwnd;
        data.uID = 1;
        data.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
        data.uCallbackMessage = MESSAGE;
        data.hIcon = LoadIconW(null_mut(), IDI_APPLICATION);
        let tip = wide(&format!(
            "RS7 Işık v{} — Açmak için tıklayın",
            env!("CARGO_PKG_VERSION")
        ));
        for (dest, src) in data.szTip.iter_mut().zip(tip) {
            *dest = src;
        }
        let mut tray = Self {
            available: false,
            data,
        };
        tray.recreate();
        tray
    }
    pub unsafe fn recreate(&mut self) {
        self.available = Shell_NotifyIconW(NIM_ADD, &self.data) != 0;
    }
    pub unsafe fn remove(&mut self) {
        if self.available {
            Shell_NotifyIconW(NIM_DELETE, &self.data);
            self.available = false;
        }
    }
}
pub unsafe fn menu(hwnd: HWND, connected: bool, running: bool) -> u32 {
    let menu = CreatePopupMenu();
    if menu.is_null() {
        return OPEN;
    }
    AppendMenuW(menu, MF_STRING, OPEN as usize, wide("Aç").as_ptr());
    AppendMenuW(
        menu,
        MF_STRING | if connected && !running { 0 } else { MF_GRAYED },
        START as usize,
        wide("Efekti başlat").as_ptr(),
    );
    AppendMenuW(
        menu,
        MF_STRING | if running { 0 } else { MF_GRAYED },
        STOP as usize,
        wide("Efekti durdur").as_ptr(),
    );
    AppendMenuW(menu, MF_SEPARATOR, 0, null());
    AppendMenuW(menu, MF_STRING, QUIT as usize, wide("Tamamen çık").as_ptr());
    let mut point: POINT = std::mem::zeroed();
    GetCursorPos(&mut point);
    SetForegroundWindow(hwnd);
    let selected = TrackPopupMenu(
        menu,
        TPM_RETURNCMD | TPM_RIGHTBUTTON,
        point.x,
        point.y,
        0,
        hwnd,
        null(),
    ) as u32;
    DestroyMenu(menu);
    PostMessageW(hwnd, WM_NULL, 0, 0);
    selected
}
impl Drop for Tray {
    fn drop(&mut self) {
        unsafe {
            self.remove();
        }
    }
}
