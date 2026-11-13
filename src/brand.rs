use std::ptr::null;
use windows_sys::Win32::{System::LibraryLoader::GetModuleHandleW, UI::WindowsAndMessaging::*};
pub const NAME: &str = "RS7 Light";
/// Shared, embedded multi-resolution icon; Windows owns this handle.
pub unsafe fn icon(size: i32) -> HICON {
    LoadImageW(
        GetModuleHandleW(null()),
        1usize as *const u16,
        IMAGE_ICON,
        size,
        size,
        LR_SHARED,
    ) as HICON
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows_sys::Win32::Graphics::Gdi::*;
    #[test]
    fn embedded_logo_loads_at_windows_icon_sizes() {
        unsafe {
            for size in [16, 32, 64, 256] {
                let icon = icon(size);
                assert!(!icon.is_null(), "Gömülü logo bulunamadı: {size}");
                let mut info: ICONINFO = std::mem::zeroed();
                assert_ne!(GetIconInfo(icon, &mut info), 0);
                let mut bitmap: BITMAP = std::mem::zeroed();
                let result = GetObjectW(
                    info.hbmColor,
                    std::mem::size_of::<BITMAP>() as i32,
                    &mut bitmap as *mut _ as *mut _,
                );
                DeleteObject(info.hbmColor);
                DeleteObject(info.hbmMask);
                assert_ne!(result, 0);
                assert_eq!((bitmap.bmWidth, bitmap.bmHeight), (size, size));
            }
        }
    }
}
