//! Opt-in, per-user Windows logon startup. No elevated rights or background service.
use std::{
    io,
    ptr::{null, null_mut},
};
use windows_sys::Win32::{Foundation::*, System::Registry::*};
const RUN: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
const NAME: &str = "RS7";
fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}
fn command(path: &std::path::Path) -> io::Result<Vec<u16>> {
    use std::os::windows::ffi::OsStrExt;
    let raw: Vec<u16> = path.as_os_str().encode_wide().collect();
    if !path.is_absolute() || raw.iter().any(|&v| v == 0 || v == 34) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Geçersiz uygulama yolu.",
        ));
    }
    let result: Vec<u16> = std::iter::once(34).chain(raw).chain([34, 0]).collect();
    if result.len() > 260 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput,"Windows başlangıç kaydı için dosya yolu çok uzun. RS7'yi daha kısa bir klasör yoluna taşıyın."));
    }
    Ok(result)
}
struct Key(HKEY);
impl Drop for Key {
    fn drop(&mut self) {
        unsafe {
            RegCloseKey(self.0);
        }
    }
}
unsafe fn open(write: bool) -> io::Result<Option<Key>> {
    let mut key = null_mut();
    let code = if write {
        RegCreateKeyExW(
            HKEY_CURRENT_USER,
            wide(RUN).as_ptr(),
            0,
            null(),
            REG_OPTION_NON_VOLATILE,
            KEY_QUERY_VALUE | KEY_SET_VALUE,
            null(),
            &mut key,
            null_mut(),
        )
    } else {
        RegOpenKeyExW(
            HKEY_CURRENT_USER,
            wide(RUN).as_ptr(),
            0,
            KEY_QUERY_VALUE,
            &mut key,
        )
    };
    if code == ERROR_FILE_NOT_FOUND {
        return Ok(None);
    }
    if code != ERROR_SUCCESS {
        return Err(io::Error::from_raw_os_error(code as i32));
    }
    Ok(Some(Key(key)))
}
pub fn enabled() -> io::Result<bool> {
    unsafe {
        let Some(key) = open(false)? else {
            return Ok(false);
        };
        let expected = command(&std::env::current_exe()?)?;
        let mut kind = 0;
        let mut bytes = 0;
        let code = RegQueryValueExW(
            key.0,
            wide(NAME).as_ptr(),
            null(),
            &mut kind,
            null_mut(),
            &mut bytes,
        );
        if code == ERROR_FILE_NOT_FOUND {
            return Ok(false);
        }
        if code != ERROR_SUCCESS {
            return Err(io::Error::from_raw_os_error(code as i32));
        }
        if kind != REG_SZ || bytes > 4096 || bytes % 2 != 0 {
            return Ok(false);
        }
        let mut data = vec![0u16; bytes as usize / 2];
        let code = RegQueryValueExW(
            key.0,
            wide(NAME).as_ptr(),
            null(),
            &mut kind,
            data.as_mut_ptr().cast(),
            &mut bytes,
        );
        if code != ERROR_SUCCESS {
            return Err(io::Error::from_raw_os_error(code as i32));
        }
        data.truncate(bytes as usize / 2);
        Ok(data == expected)
    }
}
pub fn set(enabled: bool) -> io::Result<()> {
    unsafe {
        let command = if enabled {
            Some(command(&std::env::current_exe()?)?)
        } else {
            None
        };
        let Some(key) = open(enabled)? else {
            return Ok(());
        };
        // A read-only open cannot remove the value: reopen only our per-user Run key.
        let key = if !enabled {
            drop(key);
            let mut k = null_mut();
            let code = RegOpenKeyExW(
                HKEY_CURRENT_USER,
                wide(RUN).as_ptr(),
                0,
                KEY_SET_VALUE,
                &mut k,
            );
            if code == ERROR_FILE_NOT_FOUND {
                return Ok(());
            }
            if code != ERROR_SUCCESS {
                return Err(io::Error::from_raw_os_error(code as i32));
            }
            Key(k)
        } else {
            key
        };
        let code = if let Some(value) = command {
            RegSetValueExW(
                key.0,
                wide(NAME).as_ptr(),
                0,
                REG_SZ,
                value.as_ptr().cast(),
                (value.len() * 2) as u32,
            )
        } else {
            RegDeleteValueW(key.0, wide(NAME).as_ptr())
        };
        if code != ERROR_SUCCESS && !(!enabled && code == ERROR_FILE_NOT_FOUND) {
            return Err(io::Error::from_raw_os_error(code as i32));
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn logon_command_quotes_unicode_and_spaces_without_shell() {
        let p = std::path::Path::new("C:\\Uygulamalar\\Türkçe RS7\\RS7.exe");
        let c = command(p).unwrap();
        assert_eq!(
            String::from_utf16(&c[..c.len() - 1]).unwrap(),
            "\"C:\\Uygulamalar\\Türkçe RS7\\RS7.exe\""
        );
        assert_eq!(c.last(), Some(&0));
        assert!(command(std::path::Path::new("relative.exe")).is_err());
        assert!(command(std::path::Path::new("C:\\bad\"path.exe")).is_err());
        assert!(command(std::path::Path::new(&format!(
            "C:\\{}.exe",
            "a".repeat(260)
        )))
        .is_err());
    }
}
