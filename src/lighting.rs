pub type Rgb = [u8; 3];
pub type Frame = [Rgb; 90];
pub const EFFECTS: [&str; 11] = [
    "Tuşa basınca dalga",
    "Sürekli dalga",
    "Çapraz dalga",
    "Gökkuşağı",
    "Aurora",
    "Ateş",
    "Yağmur",
    "Yıldızlar",
    "Nefes",
    "Sabit renk",
    "Kapalı",
];
pub const PALETTES: [&str; 3] = ["Gökkuşağı", "Tek renk", "İki renk geçişi"];
#[derive(Clone, Debug)]
pub struct Settings {
    pub effect: usize,
    pub brightness: f32,
    pub speed: f32,
    pub palette: usize,
    pub first: Rgb,
    pub second: Rgb,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            effect: 0,
            brightness: 0.6,
            speed: 1.0,
            palette: 0,
            first: [30, 180, 255],
            second: [200, 20, 255],
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Press {
    pub x: f32,
    pub y: f32,
    pub time: f32,
}
pub fn coord(index: usize) -> (u8, u8) {
    let (row, col) = (index / 15, index % 15);
    let col = if row == 5 && col == 11 {
        12
    } else if row == 5 && col == 12 {
        11
    } else {
        col
    };
    (row as u8, col as u8)
}
pub fn position(scan: u32) -> Option<(f32, f32)> {
    if scan == 57 {
        return Some((5., 5.));
    }
    const ROWS: [[u32; 15]; 6] = [
        [1, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 87, 88, 338, 339],
        [41, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 335],
        [15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 43, 329],
        [58, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 0, 28, 337],
        [42, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 0, 328, 0],
        [29, 347, 56, 0, 0, 57, 0, 0, 0, 312, 0, 331, 0, 336, 333],
    ];
    if scan == 0 {
        return None;
    }
    for (y, row) in ROWS.iter().enumerate() {
        if let Some(x) = row.iter().position(|v| *v == scan) {
            return Some((x as f32, y as f32));
        }
    }
    None
}
fn hsv(hue: f32, value: f32) -> Rgb {
    let h = hue.rem_euclid(1.) * 6.;
    let f = |n: f32| {
        let k = (n + h).rem_euclid(6.);
        (255. * value * (1. - k.min(4. - k).clamp(0., 1.))).round() as u8
    };
    [f(5.), f(3.), f(1.)]
}
fn palette(s: &Settings, hue: f32, value: f32) -> Rgb {
    let amount = value.clamp(0., 1.) * s.brightness.clamp(0., 1.);
    match s.palette {
        1 => s.first.map(|v| (v as f32 * amount).round() as u8),
        2 => {
            let blend = 0.5 + 0.5 * (hue * std::f32::consts::TAU).sin();
            std::array::from_fn(|i| {
                ((s.first[i] as f32 * (1. - blend) + s.second[i] as f32 * blend) * amount).round()
                    as u8
            })
        }
        _ => hsv(hue, amount),
    }
}
fn noise(n: f32) -> f32 {
    (n.mul_add(127.1, 11.7).sin() * 43758.547).fract().abs()
}
pub fn render(s: &Settings, phase: f32, now: f32, presses: &[Press]) -> Frame {
    std::array::from_fn(|index| {
        let (row, col) = coord(index);
        let (x, y) = (col as f32, row as f32);
        let mut rgb = [0; 3];
        if s.effect == 0 {
            for press in presses {
                let age = (now - press.time) * s.speed.clamp(0.2, 3.);
                if !(0.0..2.2).contains(&age) {
                    continue;
                }
                let distance = ((x - press.x).powi(2) + ((y - press.y) * 1.25).powi(2)).sqrt();
                let value = (1. - (distance - age * 8.).abs() / 1.5).max(0.) * (1. - age / 2.2);
                let ring = palette(s, distance / 20. - age * 0.3, value);
                for i in 0..3 {
                    rgb[i] = rgb[i].max(ring[i]);
                }
            }
            return rgb;
        }
        let (h, v) = match s.effect {
            1 => (x / 18. - phase * 0.14, 1.),
            2 => ((x + y) / 18. - phase * 0.14, 1.),
            3 => (phase * 0.1, 1.),
            4 => (
                0.42 + 0.18 * (x * 0.4 + phase + y * 0.5).sin(),
                0.4 + 0.3 * ((x * 0.5 - phase).sin() + 1.),
            ),
            5 => (
                0.02 + 0.09 * noise(x + (phase * 8.).floor() * 15. + y),
                (1. - y / 9.).max(0.05) * (0.5 + 0.5 * noise(x * 7. + y + (phase * 8.).floor())),
            ),
            6 => {
                let head = (phase * (2. + noise(x)) + noise(x) * 8.).rem_euclid(9.);
                (0.55, (1. - (head - y).abs() / 2.).max(0.))
            }
            7 => (
                noise(x + y * 15.),
                (phase * 1.7 + noise(x + y * 15.) * 40.)
                    .sin()
                    .max(0.)
                    .powi(12),
            ),
            8 => (phase * 0.1, 0.5 - 0.5 * (phase * 2.).cos()),
            9 => {
                return s
                    .first
                    .map(|v| (v as f32 * s.brightness.clamp(0., 1.)).round() as u8);
            }
            _ => (0., 0.),
        };
        palette(s, h, v)
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ripple_expands_and_expires() {
        let s = Settings::default();
        let p = [Press {
            x: 5.,
            y: 2.,
            time: 10.,
        }];
        assert_eq!(render(&s, 0., 10., &[]), [[0; 3]; 90]);
        assert_ne!(render(&s, 0., 10., &p)[35], [0; 3]);
        let later = render(&s, 0., 10.5, &p);
        assert_eq!(later[35], [0; 3]);
        assert_ne!(later[39], [0; 3]);
        assert_eq!(render(&s, 0., 13., &p), [[0; 3]; 90]);
    }
    #[test]
    fn speed_controls_radius() {
        let mut s = Settings::default();
        let p = [Press {
            x: 5.,
            y: 2.,
            time: 0.,
        }];
        s.speed = 2.;
        assert_eq!(
            render(&s, 0., 0.25, &p),
            render(&Settings::default(), 0., 0.5, &p)
        );
    }
    #[test]
    fn custom_colors_and_brightness_apply() {
        let mut s = Settings {
            palette: 1,
            first: [200, 0, 0],
            brightness: 0.5,
            effect: 1,
            ..Default::default()
        };
        assert!(render(&s, 0., 0., &[]).iter().all(|c| *c == [100, 0, 0]));
        for effect in 0..11 {
            s.effect = effect;
            s.brightness = 0.;
            assert_eq!(
                render(
                    &s,
                    1.,
                    1.,
                    &[Press {
                        x: 1.,
                        y: 1.,
                        time: 1.
                    }]
                ),
                [[0; 3]; 90]
            );
        }
    }
    #[test]
    fn physical_coordinates_preserve_order() {
        assert_eq!(coord(86), (5, 12));
        assert_eq!(coord(87), (5, 11));
        assert_eq!(position(30), Some((1., 3.)));
        assert_eq!(position(328), Some((13., 4.)));
        assert_eq!(position(0), None);
    }
}
