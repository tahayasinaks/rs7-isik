//! GitHub stable-release updates over Windows TLS; never installs without a click.
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    ptr::{null, null_mut},
    sync::{Arc, Mutex},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use windows_sys::Win32::Networking::WinHttp::*;
const API: &str = "https://api.github.com/repos/tahayasinaks/rs7-isik/releases/latest";
const PREFIX: &str = "https://github.com/tahayasinaks/rs7-isik/releases/download/";
#[derive(Clone, Debug)]
pub struct Release {
    pub version: String,
    pub url: String,
    pub size: u64,
    pub digest: String,
    pub sums: String,
}
#[derive(Clone, Debug)]
pub enum State {
    Checking,
    Current,
    Available(Release),
    Downloading(u8),
    Ready(PathBuf),
    Installing,
    Error(String),
}
pub type Shared = Arc<Mutex<State>>;
fn version(s: &str) -> Option<[u32; 3]> {
    let mut v = [0; 3];
    let p: Vec<_> = s.strip_prefix('v').unwrap_or(s).split('.').collect();
    if p.len() != 3 {
        return None;
    }
    for (i, n) in p.iter().enumerate() {
        if n.is_empty()
            || !n.bytes().all(|c| c.is_ascii_digit())
            || (n.len() > 1 && n.starts_with('0'))
        {
            return None;
        }
        v[i] = n.parse().ok()?;
    }
    Some(v)
}
fn newer(remote: &str, current: &str) -> bool {
    let Some(r) = version(remote) else {
        return false;
    };
    let base = current.split('-').next().unwrap_or(current);
    version(base).is_some_and(|c| r > c || (r == c && current.contains('-')))
}
fn digest(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit())
}
fn manifest(raw: &[u8], current: &str) -> Result<Option<Release>, String> {
    let d: serde_json::Value =
        serde_json::from_slice(raw).map_err(|_| "Güncelleme yanıtı okunamadı.")?;
    if d["draft"].as_bool() != Some(false) || d["prerelease"].as_bool() != Some(false) {
        return Ok(None);
    }
    let tag = d["tag_name"].as_str().ok_or("Sürüm etiketi eksik.")?;
    if !newer(tag, current) {
        return Ok(None);
    }
    let v = tag.strip_prefix('v').ok_or("Sürüm etiketi geçersiz.")?;
    let name = format!("RS7-Light-v{v}-Setup-x64.exe");
    let assets = d["assets"].as_array().ok_or("Kurulum dosyaları eksik.")?;
    let pick = |name: &str| assets.iter().find(|a| a["name"].as_str() == Some(name));
    let a = pick(&name).ok_or("Yeni sürümün kurulum paketi henüz hazır değil.")?;
    let url = a["browser_download_url"]
        .as_str()
        .ok_or("İndirme adresi eksik.")?;
    if url != format!("{PREFIX}{tag}/{name}") {
        return Err("Güncelleme adresi doğrulanamadı.".into());
    }
    let size = a["size"]
        .as_u64()
        .filter(|&n| n > 0 && n <= 100 * 1024 * 1024)
        .ok_or("Kurulum boyutu geçersiz.")?;
    let hash = a["digest"]
        .as_str()
        .and_then(|s| s.strip_prefix("sha256:"))
        .filter(|s| digest(s))
        .ok_or("GitHub SHA-256 bilgisi eksik; kurulum başlatılmadı.")?;
    let sums = pick("SHA256SUMS.txt")
        .and_then(|a| a["browser_download_url"].as_str())
        .ok_or("Doğrulama dosyası eksik.")?;
    if sums != format!("{PREFIX}{tag}/SHA256SUMS.txt") {
        return Err("Doğrulama adresi geçersiz.".into());
    }
    Ok(Some(Release {
        version: v.into(),
        url: url.into(),
        size,
        digest: hash.to_lowercase(),
        sums: sums.into(),
    }))
}
struct Internet(*mut std::ffi::c_void);
impl Drop for Internet {
    fn drop(&mut self) {
        unsafe {
            WinHttpCloseHandle(self.0);
        }
    }
}
fn error() -> String {
    format!("Ağ bağlantısı kurulamadı: {}", io::Error::last_os_error())
}
fn get(
    url: &str,
    limit: u64,
    mut consume: impl FnMut(&[u8], u64) -> Result<(), String>,
) -> Result<(), String> {
    unsafe {
        let address = url
            .strip_prefix("https://")
            .ok_or("HTTPS adresi gerekli.")?;
        let (host, path) = address.split_once('/').ok_or("Adres geçersiz.")?;
        if !matches!(host, "api.github.com" | "github.com") {
            return Err("Güncelleme sunucusu geçersiz.".into());
        }
        let wide = |s: &str| s.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
        let session = Internet(WinHttpOpen(
            wide("RS7-Light-Updater/1").as_ptr(),
            WINHTTP_ACCESS_TYPE_AUTOMATIC_PROXY,
            null(),
            null(),
            0,
        ));
        if session.0.is_null() {
            return Err(error());
        }
        WinHttpSetTimeouts(session.0, 10000, 10000, 15000, 15000);
        let connection = Internet(WinHttpConnect(session.0, wide(host).as_ptr(), 443, 0));
        if connection.0.is_null() {
            return Err(error());
        }
        let request = Internet(WinHttpOpenRequest(
            connection.0,
            wide("GET").as_ptr(),
            wide(&format!("/{path}")).as_ptr(),
            null(),
            null(),
            null(),
            WINHTTP_FLAG_SECURE,
        ));
        if request.0.is_null() {
            return Err(error());
        }
        let redirect = WINHTTP_OPTION_REDIRECT_POLICY_DISALLOW_HTTPS_TO_HTTP;
        if WinHttpSetOption(
            request.0,
            WINHTTP_OPTION_REDIRECT_POLICY,
            (&redirect as *const u32).cast(),
            4,
        ) == 0
        {
            return Err(error());
        }
        let headers = wide("Accept: application/vnd.github+json\r\n");
        if WinHttpSendRequest(request.0, headers.as_ptr(), u32::MAX, null(), 0, 0, 0) == 0
            || WinHttpReceiveResponse(request.0, null_mut()) == 0
        {
            return Err(error());
        }
        let (mut status, mut size) = (0u32, 4u32);
        if WinHttpQueryHeaders(
            request.0,
            WINHTTP_QUERY_STATUS_CODE | WINHTTP_QUERY_FLAG_NUMBER,
            null(),
            (&mut status as *mut u32).cast(),
            &mut size,
            null_mut(),
        ) == 0
        {
            return Err(error());
        }
        if status != 200 {
            return Err(format!(
                "Güncelleme sunucusu HTTP {status} döndürdü. Daha sonra tekrar deneyin."
            ));
        }
        let start = Instant::now();
        let mut total = 0;
        let mut buf = [0u8; 65536];
        loop {
            if start.elapsed() > Duration::from_secs(180) {
                return Err("İndirme zaman aşımına uğradı.".into());
            }
            let mut n = 0;
            if WinHttpReadData(request.0, buf.as_mut_ptr().cast(), buf.len() as u32, &mut n) == 0 {
                return Err(error());
            }
            if n == 0 {
                break;
            }
            total += n as u64;
            if total > limit {
                return Err("İndirme boyut sınırını aştı.".into());
            }
            consume(&buf[..n as usize], total)?;
        }
        Ok(())
    }
}
fn bytes(url: &str, limit: u64) -> Result<Vec<u8>, String> {
    let mut data = Vec::new();
    get(url, limit, |b, _| {
        data.extend_from_slice(b);
        Ok(())
    })?;
    Ok(data)
}
pub fn check(state: Shared) {
    *state.lock().unwrap() = State::Checking;
    std::thread::spawn(move || {
        let result =
            bytes(API, 2 * 1024 * 1024).and_then(|b| manifest(&b, env!("CARGO_PKG_VERSION")));
        *state.lock().unwrap() = match result {
            Ok(Some(r)) => State::Available(r),
            Ok(None) => State::Current,
            Err(e) => State::Error(e),
        };
    });
}
fn verify_sums(raw: &[u8], name: &str, expected: &str) -> bool {
    String::from_utf8_lossy(raw).lines().any(|l| {
        let mut p = l.split_whitespace();
        match (p.next(), p.next(), p.next()) {
            (Some(h), Some(n), None) => {
                h.eq_ignore_ascii_case(expected) && n.trim_start_matches('*') == name
            }
            _ => false,
        }
    })
}
pub fn download(state: Shared, r: Release) {
    *state.lock().unwrap() = State::Downloading(0);
    std::thread::spawn(move || {
        let result = (|| {
            let name = format!("RS7-Light-v{}-Setup-x64.exe", r.version);
            let sums = bytes(&r.sums, 65536)?;
            if !verify_sums(&sums, &name, &r.digest) {
                return Err("Yayın doğrulama bilgileri eşleşmedi; indirme durduruldu.".into());
            }
            let base = std::env::var_os("LOCALAPPDATA")
                .map(PathBuf::from)
                .ok_or("LOCALAPPDATA bulunamadı.")?
                .join("RS7Light")
                .join("Updates");
            fs::create_dir_all(&base).map_err(|e| e.to_string())?;
            let dir = base.join(format!(
                "v{}-{}-{}",
                r.version,
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map_err(|e| e.to_string())?
                    .as_millis()
            ));
            fs::create_dir(&dir).map_err(|e| e.to_string())?;
            let part = dir.join("download.part");
            let path = dir.join(&name);
            let transfer = (|| {
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&part)
                    .map_err(|e| e.to_string())?;
                let mut hash = Sha256::new();
                let mut count = 0;
                get(&r.url, r.size, |b, n| {
                    file.write_all(b).map_err(|e| e.to_string())?;
                    hash.update(b);
                    count = n;
                    *state.lock().unwrap() = State::Downloading(((n * 100 / r.size).min(99)) as u8);
                    Ok(())
                })?;
                file.sync_all().map_err(|e| e.to_string())?;
                drop(file);
                if count != r.size || format!("{:x}", hash.finalize()) != r.digest {
                    return Err("İndirilen kurulum dosyası doğrulanamadı; çalıştırılmadı.".into());
                }
                fs::rename(&part, &path).map_err(|e| e.to_string())?;
                Ok(path)
            })();
            if transfer.is_err() {
                let _ = fs::remove_file(&part);
                let _ = fs::remove_dir(&dir);
            }
            transfer
        })();
        *state.lock().unwrap() = match result {
            Ok(p) => State::Ready(p),
            Err(e) => State::Error(e),
        };
    });
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn version_ordering() {
        assert!(newer("v1.2.10", "1.2.9"));
        assert!(!newer("v1.1.3", "1.2.0"));
        assert!(!newer("v1.2.0", "1.2.0"));
        assert!(newer("v1.2.0", "1.2.0-preview.8"));
        for v in ["v1.2.0-beta", "v1.02.0", "1.2", "1.2.0.1", "1.2.x"] {
            assert!(!newer(v, "1.0.0"));
        }
    }
    fn sample() -> serde_json::Value {
        let name = "RS7-Light-v2.0.0-Setup-x64.exe";
        serde_json::json!({"draft":false,"prerelease":false,"tag_name":"v2.0.0","assets":[{"name":name,"browser_download_url":format!("{PREFIX}v2.0.0/{name}"),"size":1024,"digest":format!("sha256:{}","a".repeat(64))},{"name":"SHA256SUMS.txt","browser_download_url":format!("{PREFIX}v2.0.0/SHA256SUMS.txt")}]})
    }
    #[test]
    fn release_requires_expected_asset_and_integrity() {
        let d = sample();
        assert!(manifest(d.to_string().as_bytes(), "1.0.0")
            .unwrap()
            .is_some());
        for field in ["browser_download_url", "digest", "size"] {
            let mut x = d.clone();
            x["assets"][0][field] = serde_json::json!("bad");
            assert!(manifest(x.to_string().as_bytes(), "1.0.0").is_err());
        }
        let mut x = d.clone();
        x["prerelease"] = true.into();
        assert!(manifest(x.to_string().as_bytes(), "1.0.0")
            .unwrap()
            .is_none());
    }
    #[test]
    fn sums_match_only_exact_filename_and_hash() {
        let hash = "a".repeat(64);
        assert!(verify_sums(
            format!("{hash}  setup.exe\n").as_bytes(),
            "setup.exe",
            &hash
        ));
        assert!(!verify_sums(
            format!("{hash}  other.exe\n").as_bytes(),
            "setup.exe",
            &hash
        ));
        assert!(!verify_sums(b"bad setup.exe", "setup.exe", &hash));
        assert_eq!(
            format!("{:x}", Sha256::digest(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
