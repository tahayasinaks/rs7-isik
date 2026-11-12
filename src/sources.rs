//! Live sources run away from the USB deadline. Only the selected, running mode is sampled.
use crate::lighting::{self, LiveInput};
use std::{
    ffi::c_void,
    ptr::{null, null_mut},
    sync::{
        atomic::{AtomicBool, AtomicU8, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};
use windows_sys::{
    core::GUID,
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        System::{Com::*, SystemInformation::*, Threading::GetSystemTimes},
        UI::WindowsAndMessaging::*,
    },
};
#[derive(Clone, Default)]
pub struct Snapshot {
    pub input: LiveInput,
    pub note: String,
}
pub struct Sources {
    mode: Arc<AtomicU8>,
    quit: Arc<AtomicBool>,
    snapshot: Arc<Mutex<Snapshot>>,
    join: Option<JoinHandle<()>>,
}
impl Sources {
    pub fn new() -> Self {
        let mode = Arc::new(AtomicU8::new(0));
        let quit = Arc::new(AtomicBool::new(false));
        let snapshot = Arc::new(Mutex::new(Snapshot::default()));
        let (m, q, s) = (mode.clone(), quit.clone(), snapshot.clone());
        let join = thread::spawn(move || run(m, q, s));
        Self {
            mode,
            quit,
            snapshot,
            join: Some(join),
        }
    }
    pub fn select(&self, effect: Option<usize>) {
        self.mode.store(
            effect.map(lighting::source_mode).unwrap_or(0),
            Ordering::Relaxed,
        );
    }
    pub fn read(&self) -> Snapshot {
        self.snapshot.lock().unwrap().clone()
    }
}
impl Drop for Sources {
    fn drop(&mut self) {
        self.quit.store(true, Ordering::Relaxed);
        if let Some(j) = self.join.take() {
            let _ = j.join();
        }
    }
}
struct ComPtr(*mut c_void);
impl ComPtr {
    unsafe fn slot(&self, n: usize) -> *const c_void {
        *(*(self.0 as *const *const *const c_void)).add(n)
    }
}
impl Drop for ComPtr {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                let release: unsafe extern "system" fn(*mut c_void) -> u32 =
                    std::mem::transmute(self.slot(2));
                release(self.0);
            }
        }
    }
}
// ABI and interface IDs from the Windows SDK mmdeviceapi.h / endpointvolume.h.
unsafe fn meter() -> Result<ComPtr, String> {
    let cls = GUID::from_u128(0xbcde0395_e52f_467c_8e3d_c4579291692e);
    let iid = GUID::from_u128(0xa95664d2_9614_4f35_a746_de8db63617e6);
    let meterid = GUID::from_u128(0xc02216f6_8c67_4b5b_9d00_d008e73e0064);
    let mut p = null_mut();
    let hr = CoCreateInstance(&cls, null_mut(), CLSCTX_INPROC_SERVER, &iid, &mut p);
    if hr < 0 {
        return Err(format!("Ses aygıtı açılamadı ({hr:08X})."));
    }
    let enumerator = ComPtr(p);
    let get: unsafe extern "system" fn(*mut c_void, i32, i32, *mut *mut c_void) -> i32 =
        std::mem::transmute(enumerator.slot(4));
    let mut p = null_mut();
    let hr = get(enumerator.0, 0, 0, &mut p);
    if hr < 0 {
        return Err("Varsayılan ses çıkışı bulunamadı.".into());
    }
    let device = ComPtr(p);
    let activate: unsafe extern "system" fn(
        *mut c_void,
        *const GUID,
        u32,
        *const c_void,
        *mut *mut c_void,
    ) -> i32 = std::mem::transmute(device.slot(3));
    let mut p = null_mut();
    let hr = activate(device.0, &meterid, CLSCTX_ALL, null(), &mut p);
    if hr < 0 {
        return Err(format!("Ses ölçer açılamadı ({hr:08X})."));
    }
    Ok(ComPtr(p))
}
unsafe fn peaks(m: &ComPtr) -> Result<[f32; 2], String> {
    let count: unsafe extern "system" fn(*mut c_void, *mut u32) -> i32 =
        std::mem::transmute(m.slot(4));
    let mut n = 0;
    if count(m.0, &mut n) < 0 || n == 0 || n > 32 {
        return Err("Ses kanalları okunamadı.".into());
    }
    let get: unsafe extern "system" fn(*mut c_void, u32, *mut f32) -> i32 =
        std::mem::transmute(m.slot(5));
    let mut values = [0f32; 32];
    if get(m.0, n, values.as_mut_ptr()) < 0 {
        return Err("Ses aygıtı değişti; yeniden bağlanılıyor.".into());
    }
    Ok([
        values[0].clamp(0., 1.),
        values[if n > 1 { 1 } else { 0 }].clamp(0., 1.),
    ])
}
unsafe fn screen() -> Option<lighting::Frame> {
    let src = GetDC(null_mut());
    if src.is_null() {
        return None;
    }
    let dc = CreateCompatibleDC(src);
    if dc.is_null() {
        ReleaseDC(null_mut(), src);
        return None;
    }
    let bmp = CreateCompatibleBitmap(src, 15, 6);
    if bmp.is_null() {
        DeleteDC(dc);
        ReleaseDC(null_mut(), src);
        return None;
    }
    let old = SelectObject(dc, bmp);
    SetStretchBltMode(dc, HALFTONE);
    SetBrushOrgEx(dc, 0, 0, null_mut());
    let ok = StretchBlt(
        dc,
        0,
        0,
        15,
        6,
        src,
        0,
        0,
        GetSystemMetrics(SM_CXSCREEN),
        GetSystemMetrics(SM_CYSCREEN),
        SRCCOPY,
    );
    SelectObject(dc, old);
    let mut info: BITMAPINFO = std::mem::zeroed();
    info.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
    info.bmiHeader.biWidth = 15;
    info.bmiHeader.biHeight = -6;
    info.bmiHeader.biPlanes = 1;
    info.bmiHeader.biBitCount = 32;
    info.bmiHeader.biCompression = BI_RGB;
    let mut pixels = [0u8; 360];
    let read = if ok != 0 {
        GetDIBits(
            dc,
            bmp,
            0,
            6,
            pixels.as_mut_ptr() as *mut _,
            &mut info,
            DIB_RGB_COLORS,
        )
    } else {
        0
    };
    DeleteObject(bmp);
    DeleteDC(dc);
    ReleaseDC(null_mut(), src);
    if read != 6 {
        return None;
    }
    Some(std::array::from_fn(|i| {
        let (r, c) = lighting::coord(i);
        let p = (r as usize * 15 + c as usize) * 4;
        [pixels[p + 2], pixels[p + 1], pixels[p]]
    }))
}
fn ticks(t: FILETIME) -> u64 {
    ((t.dwHighDateTime as u64) << 32) | t.dwLowDateTime as u64
}
unsafe fn stats(previous: &mut Option<(u64, u64)>) -> (f32, f32, u8, u8) {
    let (mut idle, mut kernel, mut user): (FILETIME, FILETIME, FILETIME) =
        (std::mem::zeroed(), std::mem::zeroed(), std::mem::zeroed());
    let mut cpu = 0.;
    if GetSystemTimes(&mut idle, &mut kernel, &mut user) != 0 {
        let total = ticks(kernel) + ticks(user);
        let free = ticks(idle);
        if let Some((oldtotal, oldidle)) = *previous {
            let dt = total.saturating_sub(oldtotal);
            if dt > 0 {
                cpu = (1. - free.saturating_sub(oldidle) as f32 / dt as f32).clamp(0., 1.);
            }
        }
        *previous = Some((total, free));
    }
    let mut mem: MEMORYSTATUSEX = std::mem::zeroed();
    mem.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
    let memory = if GlobalMemoryStatusEx(&mut mem) != 0 {
        mem.dwMemoryLoad as f32 / 100.
    } else {
        0.
    };
    let mut time: SYSTEMTIME = std::mem::zeroed();
    GetLocalTime(&mut time);
    (cpu, memory, time.wHour as u8, time.wMinute as u8)
}
fn run(mode: Arc<AtomicU8>, quit: Arc<AtomicBool>, shared: Arc<Mutex<Snapshot>>) {
    unsafe {
        let com = CoInitializeEx(null(), COINIT_MULTITHREADED as u32) >= 0;
        let mut audio = None;
        let mut snapshot = Snapshot::default();
        let mut previous = None;
        let mut last_mode = 0;
        let mut retry = Instant::now() - Duration::from_secs(3);
        let mut last_screen = retry;
        let mut last_stats = retry;
        while !quit.load(Ordering::Relaxed) {
            let requested = mode.load(Ordering::Relaxed);
            if requested != last_mode {
                audio = None;
                snapshot = Snapshot::default();
                previous = None;
                retry = Instant::now() - Duration::from_secs(3);
                last_mode = requested;
            }
            if requested == 1 {
                if audio.is_none() && retry.elapsed() >= Duration::from_secs(2) {
                    retry = Instant::now();
                    if com {
                        match meter() {
                            Ok(m) => {
                                audio = Some(m);
                                snapshot.note = "Ses çıkışına tepki · Mikrofon kullanılmaz.".into();
                            }
                            Err(e) => snapshot.note = e,
                        }
                    } else {
                        snapshot.note = "Windows ses sistemi başlatılamadı.".into();
                    }
                }
                if let Some(m) = &audio {
                    match peaks(m) {
                        Ok(v) => snapshot.input.audio = v,
                        Err(e) => {
                            audio = None;
                            snapshot.input.audio = [0.; 2];
                            snapshot.note = e;
                        }
                    }
                }
                // Refresh periodically so a newly selected Windows output becomes active.
                if audio.is_some() && retry.elapsed() > Duration::from_secs(3) {
                    audio = None;
                    retry = Instant::now() - Duration::from_secs(3);
                }
            } else if requested == 2 && last_screen.elapsed() >= Duration::from_millis(125) {
                last_screen = Instant::now();
                match screen() {
                    Some(frame) => {
                        snapshot.input.screen = frame;
                        snapshot.note = "Birincil ekran renkleri · Görüntü kaydedilmez.".into();
                    }
                    None => {
                        snapshot.input.screen = [[0; 3]; 90];
                        snapshot.note = "Ekran renkleri okunamadı.".into();
                    }
                }
            } else if requested == 4 && last_stats.elapsed() >= Duration::from_millis(500) {
                last_stats = Instant::now();
                let (c, m, h, min) = stats(&mut previous);
                snapshot.input.cpu = c;
                snapshot.input.memory = m;
                snapshot.input.hour = h;
                snapshot.input.minute = min;
                snapshot.note = "Windows sistem bilgileri · Sıcaklık ölçülmez.".into();
            }
            *shared.lock().unwrap() = snapshot.clone();
            thread::sleep(Duration::from_millis(25));
        }
        drop(audio);
        if com {
            CoUninitialize();
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    #[test]
    #[ignore = "Yerel Windows ses ve ekran kaynakları; CI masaüstü gerektirmez."]
    fn windows_live_sources_open_sample_and_release() {
        unsafe {
            let hr = CoInitializeEx(null(), COINIT_MULTITHREADED as u32);
            assert!(hr >= 0, "COM {hr:08X}");
            let result = meter().and_then(|m| peaks(&m));
            assert!(result.is_ok(), "{}", result.unwrap_err());
            assert!(screen().is_some(), "Ekran örneklenemedi");
            let mut previous = None;
            let (c, m, h, min) = stats(&mut previous);
            assert!((0.0..=1.).contains(&c));
            assert!((0.0..=1.).contains(&m));
            assert!(h < 24 && min < 60);
            CoUninitialize();
        }
    }
}
