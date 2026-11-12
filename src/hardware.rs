use crate::lighting::{coord, Frame};
use hidapi::{HidApi, HidDevice};
use std::time::{Duration, Instant};
pub const IDS: [u16; 8] = [4640, 4641, 4651, 4695, 4696, 4921, 4917, 4888];
pub fn version(bytes: &[u8]) -> Option<u16> {
    let p = if bytes.first() == Some(&0) {
        &bytes[1..]
    } else {
        bytes
    };
    if p.len() < 8 || p[0] != 2 {
        return None;
    }
    let text = format!("{:x}{:02x}", p[5], p[6]);
    let code = text.parse::<u16>().ok()?;
    (100..1000).contains(&code).then_some(code)
}
pub fn packets(frame: &Frame, firmware: u16) -> Vec<Vec<u8>> {
    let limit = if firmware >= 200 { 112 } else { 41 };
    frame
        .chunks(limit)
        .enumerate()
        .map(|(chunk, colors)| {
            let (y, x) = coord(chunk * limit);
            let mut p = vec![0, 0x1d, y, x, colors.len() as u8];
            for c in colors {
                p.extend_from_slice(c);
            }
            p
        })
        .collect()
}
pub struct Keyboard {
    device: HidDevice,
    pub firmware: u16,
    pub pid: u16,
}
impl Keyboard {
    pub fn connect() -> Result<Self, String> {
        let api = HidApi::new().map_err(|e| e.to_string())?;
        let devices: Vec<_> = api
            .device_list()
            .filter(|d| {
                d.vendor_id() == 0x373b
                    && IDS.contains(&d.product_id())
                    && d.usage_page() == 0xff60
                    && d.usage() == 0x61
            })
            .collect();
        if devices.len() != 1 {
            return Err(if devices.is_empty(){"RS7 Air bulunamadı. USB bağlantısını kontrol edip ATK HUB ve eski uygulamayı kapatın."}else{"Birden çok RS7 Air bulundu. Yalnızca kontrol edeceğiniz klavyeyi bağlı bırakın."}.into());
        }
        let info = devices[0];
        let device = info.open_device(&api).map_err(|e| e.to_string())?;
        let mut request = [0; 64];
        request[1] = 2;
        request[2] = 150;
        device.write(&request).map_err(|e| e.to_string())?;
        let deadline = Instant::now() + Duration::from_secs(3);
        let mut buffer = [0; 512];
        while Instant::now() < deadline {
            let n = device
                .read_timeout(&mut buffer, 100)
                .map_err(|e| e.to_string())?;
            if let Some(firmware) = version(&buffer[..n]) {
                return Ok(Self {
                    device,
                    firmware,
                    pid: info.product_id(),
                });
            }
        }
        Err("Firmware sürümü okunamadı. ATK HUB ve eski uygulamayı kapatıp tekrar bağlanın.".into())
    }
    pub fn start(&self) -> Result<(), String> {
        if self.firmware < 200 {
            let mut p = [0; 64];
            p[..10].copy_from_slice(&[0, 7, 65, 46, 0, 127, 255, 0, 255, 100]);
            self.device.write(&p).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
    pub fn write(&self, frame: &Frame) -> Result<(), String> {
        for packet in packets(frame, self.firmware) {
            let n = self.device.write(&packet).map_err(|e| e.to_string())?;
            if n < packet.len() {
                return Err("RGB paketi eksik gönderildi.".into());
            }
        }
        Ok(())
    }
    pub fn black(&self) -> Result<(), String> {
        self.write(&[[0; 3]; 90])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn packet_split_and_coordinates() {
        let f = std::array::from_fn(|i| [i as u8, 1, 2]);
        let p = packets(&f, 110);
        assert_eq!(p.iter().map(|v| v[4]).collect::<Vec<_>>(), [41, 41, 8]);
        assert_eq!(&p[1][..8], &[0, 29, 2, 11, 41, 41, 1, 2]);
        assert_eq!(packets(&f, 202).len(), 1);
        assert_eq!(packets(&f, 202)[0].len(), 275);
    }
    #[test]
    fn accepts_native_and_prefixed_versions() {
        let p = [2, 150, 0, 0, 0, 2, 2, 0];
        assert_eq!(version(&p), Some(202));
        let mut b = vec![0];
        b.extend(p);
        assert_eq!(version(&b), Some(202));
        assert_eq!(version(&[2, 3]), None);
    }
}
