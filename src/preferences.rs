use crate::lighting::{Settings, EFFECTS, PALETTES};
use std::{fs, io, path::PathBuf};
fn path() -> Option<PathBuf> {
    std::env::var_os("APPDATA").map(|p| PathBuf::from(p).join("RS7Isik").join("settings-v1.txt"))
}
fn encode(s: &Settings) -> String {
    format!(
        "RS7ISIK1 {} {} {} {} {} {} {} {} {} {}",
        s.effect,
        s.palette,
        s.brightness,
        s.speed,
        s.first[0],
        s.first[1],
        s.first[2],
        s.second[0],
        s.second[1],
        s.second[2]
    )
}
fn decode(text: &str) -> Option<Settings> {
    let fields: Vec<_> = text.split_whitespace().collect();
    if fields.len() != 11 || fields[0] != "RS7ISIK1" {
        return None;
    }
    let effect = fields[1].parse().ok()?;
    let palette = fields[2].parse().ok()?;
    let brightness: f32 = fields[3].parse().ok()?;
    let speed: f32 = fields[4].parse().ok()?;
    if effect >= EFFECTS.len()
        || palette >= PALETTES.len()
        || !brightness.is_finite()
        || !speed.is_finite()
        || !(0.0..=1.0).contains(&brightness)
        || !(0.2..=3.0).contains(&speed)
    {
        return None;
    }
    Some(Settings {
        effect,
        palette,
        brightness,
        speed,
        first: [
            fields[5].parse().ok()?,
            fields[6].parse().ok()?,
            fields[7].parse().ok()?,
        ],
        second: [
            fields[8].parse().ok()?,
            fields[9].parse().ok()?,
            fields[10].parse().ok()?,
        ],
    })
}
pub fn load() -> Settings {
    path()
        .and_then(|p| fs::read_to_string(p).ok())
        .and_then(|s| decode(&s))
        .unwrap_or_default()
}
pub fn save(s: &Settings) -> io::Result<()> {
    let path = path().ok_or_else(|| io::Error::other("APPDATA bulunamadı"))?;
    fs::create_dir_all(path.parent().unwrap())?;
    let temp = path.with_extension("tmp");
    fs::write(&temp, encode(s))?;
    fs::rename(temp, path)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn round_trip_custom_preferences() {
        let s = Settings {
            effect: 11,
            palette: 2,
            speed: 2.67,
            brightness: 0.42,
            first: [255, 0, 80],
            second: [10, 220, 255],
        };
        assert_eq!(decode(&encode(&s)), Some(s));
    }
    #[test]
    fn rejects_corrupt_preferences() {
        for value in [
            "",
            "RS7ISIK1 11 2 NaN 1 1 2 3 4 5 6",
            "RS7ISIK1 99 0 1 1 1 2 3 4 5 6",
            "RS7ISIK1 1 0 1 0 1 2 3 4 5 6",
            "RS7ISIK1 1 0 1 1 999 2 3 4 5 6",
        ] {
            assert!(decode(value).is_none());
        }
    }
}
