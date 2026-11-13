mod extended;
pub type Rgb = [u8; 3];
pub type Frame = [Rgb; 90];
pub const EFFECTS: [&str; 66] = [
    "Tuşa basınca dalga",
    "Sürekli dalga",
    "Çapraz dalga",
    "Renk döngüsü",
    "Kutup ışıkları",
    "Ateş",
    "Yağmur",
    "Yıldızlar",
    "Nefes",
    "Sabit renk",
    "Kapalı",
    "Deniz dalgası",
    "Tek tuş sönümü",
    "Tuş artısı",
    "Tuş alevi",
    "Tuş patlaması",
    "Klavye tepkisi",
    "Yazım ısı haritası",
    "Rastgele renk",
    "Yanıp sönme",
    "Çift flaş",
    "Kalp atışı",
    "Tek yön tarama",
    "Gidip gelen tarayıcı",
    "Kuyruklu yıldız",
    "Işık kovalamacası",
    "Dolup boşalma",
    "Biriken bloklar",
    "Renk takası",
    "Renk çarkı",
    "Spiral",
    "Merkezden halkalar",
    "Radar",
    "Çapraz ışınlar",
    "Çift renk çarkı",
    "Sıçrayan top",
    "Hareketli noktalar",
    "Kabarcıklar",
    "Havai fişek",
    "Şimşek",
    "Gün doğumu",
    "Mum ışığı",
    "Lav",
    "Plazma",
    "Gürültü bulutu",
    "Mozaik",
    "Hareketli paneller",
    "Fraktal akış",
    "Dama",
    "Nefes alan halka",
    "Çiçeklenme",
    "Girdap halkaları",
    "Yılan",
    "Sesle parlaklık",
    "Ses seviye çubukları",
    "Sesle dalga",
    "Sesle yıldızlar",
    "Ekran yansıtma",
    "Ekran ortam rengi",
    "İşlemci yükü",
    "Bellek doluluğu",
    "Saat",
    "Sesle parti",
    "Sesle halkalar",
    "Kalp şekli",
    "Tuşla tsunami",
];
pub const CATEGORIES: [&str; 7] = [
    "Temel",
    "Tuşa duyarlı",
    "Akış ve dalga",
    "Doğa",
    "Geometri",
    "Ses ve ekran",
    "Sistem",
];
pub const CATEGORY: [usize; 66] = [
    1, 2, 2, 0, 3, 3, 3, 3, 0, 0, 0, 3, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 4, 3, 4, 4, 4, 4, 4, 4, 4, 2, 5, 5, 5, 5, 5, 5, 6, 6, 6, 5, 5,
    4, 1,
];
pub const DESCRIPTIONS: [&str; 66] = [
    "Basılan tuştan dışarı doğru ince halkalar yayılır.",
    "Renk şeridi klavyenin bir ucundan diğerine akar.",
    "Dalga satır ve sütunlarda çapraz ilerler.",
    "Bütün tuşlar renk tayfında birlikte dolaşır.",
    "Yumuşak renk perdeleri üst üste akar.",
    "Alevler alttan yükselir ve düzensiz titreşir.",
    "Bağımsız damlalar sütunlardan aşağı süzülür.",
    "Dağınık ışık noktaları yavaşça belirip kaybolur.",
    "Parlaklık yumuşakça artar ve azalır.",
    "Seçilen birinci renk bütün tuşlarda sabit kalır.",
    "Uygulama bağlı kalır; bütün tuş ışıkları söner.",
    "Geniş su dalgası ve ince köpük tepesi ilerler.",
    "Yalnızca bastığınız tuş aydınlanır ve söner.",
    "Basılan tuşun satır ve sütununda ışık yayılır.",
    "Basılan tuştan yukarı doğru alev yükselir.",
    "Basılan tuştan büyüyen dolu bir ışık alanı yayılır.",
    "Her tuş vuruşunda bütün klavye parlar ve söner.",
    "Son vuruşlar birikir; sık basılan tuşlar daha parlak olur.",
    "Bütün klavye düzenli aralıklarla yeni bir renge geçer.",
    "Bütün tuşlar aralıklı olarak açılıp kapanır.",
    "Her döngüde iki kısa ışık darbesi oluşur.",
    "İki yumuşak darbeden sonra uzun bir bekleme gelir.",
    "Dar bir ışık bandı bir yönde tarar; arkası söner.",
    "Işık bandı iki kenar arasında gidip gelir.",
    "Parlak bir baş ve sönümlenen kuyruk satırlarda ilerler.",
    "Aralıklı ışık grupları birbirini takip eder.",
    "Tuş alanı adım adım dolar, sonra boşalır.",
    "Hareketli bloklar kenarda üst üste birikir.",
    "İki dama grubu sırayla renklerini değiştirir.",
    "Renkler klavye merkezi çevresinde döner.",
    "Dönerek dışarı açılan renk sarmalı oluşur.",
    "Merkezden dışarı doğru eş merkezli renk halkaları akar.",
    "Tek bir dar ışın merkez çevresinde tarar.",
    "İki bağımsız ışık bandı birbirinin içinden geçer.",
    "İki merkezdeki renk çarkı ters yönlerde döner.",
    "Tek bir ışık topu iki eksende sekerek hareket eder.",
    "Birden fazla ışık noktası bağımsız yörüngeler izler.",
    "Yükselen boş ışık halkaları büyür ve kaybolur.",
    "Farklı merkezlerden ışık parçacıkları patlar.",
    "Karanlık alan kısa ve düzensiz çatallanan flaşlarla parlar.",
    "Sıcak renkler aşağıdan yukarıya doğru yavaşça yükselir.",
    "Sıcak ve düşük parlaklıklı ışık yumuşakça titrer.",
    "Yavaş hareket eden parlak ve koyu sıcak lekeler oluşur.",
    "Birbiriyle girişen sinüs alanları renkli desenler üretir.",
    "Yumuşak rastlantısal bulutlar hareket eder.",
    "Küçük renk blokları bağımsız aralıklarla yenilenir.",
    "Geniş renk düzlemleri yatay ve dikey kayar.",
    "Çok ölçekli dalgalar kendini tekrar eden desenler üretir.",
    "Sabit ve dönüşümlü iki renkli kareler görünür.",
    "Tek bir ışık halkasının çapı yumuşakça büyüyüp küçülür.",
    "Merkezden açılan ışık yaprakları hareket eder.",
    "İki merkez çevresinde bükülen halkalar döner.",
    "Kuyruklu ışık izi tuşları satır satır dolaşır.",
    "Varsayılan ses çıkışının seviyesine göre bütün klavye parlar.",
    "Sol ve sağ ses seviyeleri sütunlar halinde yükselir; frekans analizi değildir.",
    "Dalganın parlaklığı çalınan sesin seviyesine tepki verir.",
    "Ses yükseldikçe daha fazla yıldız parlar.",
    "Birincil ekranın 15 × 6 renk haritası tuşlara yansır.",
    "Birincil ekranın ortalama rengi bütün tuşlara uygulanır.",
    "İşlemci kullanım oranı klavyede bir doluluk çubuğu oluşturur.",
    "Bellek kullanım oranı klavyede bir doluluk çubuğu oluşturur.",
    "Yerel saat ve dakika ışık noktalarıyla gösterilir.",
    "Ses seviyesi renkli dama alanlarını aydınlatır.",
    "Ses seviyesi merkezdeki halkanın boyutunu ve parlaklığını değiştirir.",
    "Tuş ışıkları kalp şeklinde yanar; kalp ritmi hız, renk ve parlaklığa uyar.",
    "Basılan tuştan tam boy dik dalga karşı kenara ilerler; sol tuşlar sağa, sağ tuşlar sola akar.",
];
pub fn reactive(effect: usize) -> bool {
    effect == 0 || effect == 65 || (12..=17).contains(&effect)
}
pub fn source_mode(effect: usize) -> u8 {
    match effect {
        53..=56 | 62..=63 => 1,
        57..=58 => 2,
        59..=61 => 4,
        _ => 0,
    }
}
pub const PALETTES: [&str; 3] = ["Gökkuşağı", "Tek renk", "İki renk geçişi"];
#[derive(Clone, Debug, PartialEq)]
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
pub fn hsv_for_preview(h: f32) -> Rgb {
    hsv(h, 0.8)
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
        1 => {
            // Preserve the spatial/temporal pattern when a single hue replaces a colour gradient.
            let modulation =
                if matches!(s.effect, 1 | 2 | 3 | 28 | 29 | 30 | 31 | 45 | 46 | 48 | 62) {
                    0.08 + 0.92 * (0.5 + 0.5 * (hue * std::f32::consts::TAU).cos())
                } else {
                    1.
                };
            s.first
                .map(|v| (v as f32 * amount * modulation).round() as u8)
        }
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
#[cfg(test)]
pub fn render(s: &Settings, phase: f32, now: f32, presses: &[Press]) -> Frame {
    render_live(s, phase, now, presses, &LiveInput::default())
}
#[derive(Clone)]
pub struct LiveInput {
    pub audio: [f32; 2],
    pub screen: Frame,
    pub cpu: f32,
    pub memory: f32,
    pub hour: u8,
    pub minute: u8,
}
impl Default for LiveInput {
    fn default() -> Self {
        Self {
            audio: [0.; 2],
            screen: [[0; 3]; 90],
            cpu: 0.,
            memory: 0.,
            hour: 0,
            minute: 0,
        }
    }
}
pub fn render_live(
    s: &Settings,
    phase: f32,
    now: f32,
    presses: &[Press],
    input: &LiveInput,
) -> Frame {
    if s.effect >= 12 {
        return extended::render(s, phase, now, presses, input);
    }
    std::array::from_fn(|index| {
        let (row, col) = coord(index);
        let (x, y) = (col as f32, row as f32);
        let mut rgb = [0; 3];
        if s.effect == 11 {
            // A broad moving swell and a narrow bright foam crest, rather than a rainbow strip.
            let wave = (x * 0.55 + y * 0.9 - phase * 2.).sin() * 0.5 + 0.5;
            let foam = wave.powi(10);
            let brightness = s.brightness.clamp(0., 1.);
            return if s.palette == 0 {
                let deep = [0., 12., 65.];
                let surf = [0., 155., 210.];
                let crest = [190., 245., 255.];
                std::array::from_fn(|i| {
                    let water = deep[i] * (1. - wave) + surf[i] * wave;
                    ((water * (1. - foam) + crest[i] * foam) * brightness).round() as u8
                })
            } else {
                palette(s, wave * 0.5, 0.15 + wave * 0.85)
            };
        }
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
        let frame = render(&s, 0., 0., &[]);
        assert!(frame.iter().all(|c| c[0] <= 100 && c[1] == 0 && c[2] == 0));
        assert!(frame.iter().any(|c| c[0] == 100));
        assert_ne!(frame, render(&s, 1., 1., &[]));
        for effect in 0..EFFECTS.len() {
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
    #[test]
    fn sea_moves_in_space_and_time_and_supports_custom_colors() {
        let mut s = Settings {
            effect: 11,
            ..Default::default()
        };
        let first = render(&s, 0., 0., &[]);
        assert_ne!(first[0], first[8]);
        assert_ne!(first, render(&s, 1., 0., &[]));
        assert!(first.iter().all(|c| c[2] >= c[0]));
        s.palette = 1;
        s.first = [255, 0, 0];
        assert!(render(&s, 0., 0., &[])
            .iter()
            .all(|c| c[1] == 0 && c[2] == 0));
    }
}

#[cfg(test)]
mod catalog_tests {
    use super::*;
    use std::collections::HashSet;
    fn live() -> LiveInput {
        LiveInput {
            audio: [0.37, 0.81],
            screen: std::array::from_fn(|i| [(i * 2) as u8, 80, 200]),
            cpu: 0.37,
            memory: 0.71,
            hour: 12,
            minute: 34,
        }
    }
    #[test]
    fn catalog_has_unique_names_valid_metadata_and_distinct_behaviour() {
        assert_eq!(EFFECTS.len(), CATEGORY.len());
        assert_eq!(EFFECTS.len(), DESCRIPTIONS.len());
        let mut names = HashSet::new();
        let mut fingerprints = std::collections::HashMap::new();
        let presses = [
            Press {
                x: 3.,
                y: 2.,
                time: 0.,
            },
            Press {
                x: 11.,
                y: 4.,
                time: 0.17,
            },
        ];
        for (effect, name) in EFFECTS.iter().enumerate() {
            assert!(names.insert(name), "Tekrarlanan ad: {name}");
            assert!(CATEGORY[effect] < CATEGORIES.len());
            assert!(!DESCRIPTIONS[effect].is_empty());
            let s = Settings {
                effect,
                ..Default::default()
            };
            let mut signature = Vec::new();
            for t in [0.1, 0.3, 0.6, 1.1, 1.7, 2.8, 4.2, 6.3] {
                for c in render_live(&s, t, t, &presses, &live()) {
                    signature.extend(c);
                }
            }
            if let Some(previous) = fingerprints.insert(signature, *name) {
                panic!("Aynı davranış: {previous} / {name}");
            }
        }
    }
    #[test]
    fn every_effect_obeys_blackout_at_all_palettes_and_speeds() {
        let presses = [Press {
            x: 3.,
            y: 2.,
            time: 0.,
        }];
        for effect in 0..EFFECTS.len() {
            for palette in 0..3 {
                for speed in [0.2, 1., 3.] {
                    let s = Settings {
                        effect,
                        palette,
                        speed,
                        brightness: 0.,
                        ..Default::default()
                    };
                    for t in [0.2, 1., 30.] {
                        assert_eq!(
                            render_live(&s, t, t, &presses, &live()),
                            [[0; 3]; 90],
                            "{effect}/{palette}/{speed}"
                        );
                    }
                }
            }
        }
    }
    #[test]
    fn all_key_effects_require_input_and_decay_after_ten_seconds() {
        let presses = [Press {
            x: 3.,
            y: 2.,
            time: 1.,
        }];
        for effect in (0..EFFECTS.len()).filter(|&i| reactive(i)) {
            let s = Settings {
                effect,
                ..Default::default()
            };
            assert_eq!(render(&s, 1., 1., &[]), [[0; 3]; 90]);
            assert_ne!(render(&s, 1., 1.2, &presses), [[0; 3]; 90], "{effect}");
            assert_eq!(render(&s, 1., 60., &presses), [[0; 3]; 90]);
        }
    }
    #[test]
    fn live_inputs_are_used_and_silence_is_dark() {
        for effect in [53, 54, 55, 56, 62, 63] {
            let s = Settings {
                effect,
                ..Default::default()
            };
            assert_eq!(
                render_live(&s, 1., 1., &[], &LiveInput::default()),
                [[0; 3]; 90]
            );
            assert_ne!(render_live(&s, 1., 1., &[], &live()), [[0; 3]; 90]);
        }
        let s = Settings {
            effect: 57,
            brightness: 1.,
            ..Default::default()
        };
        assert_eq!(render_live(&s, 0., 0., &[], &live()), live().screen);
        let s = Settings {
            effect: 58,
            brightness: 1.,
            ..Default::default()
        };
        let frame = render_live(&s, 0., 0., &[], &live());
        assert!(frame.iter().all(|c| *c == frame[0]));
    }
}

#[cfg(test)]
mod brightness_regression {
    use super::*;
    #[test]
    fn overlapping_keys_never_exceed_the_brightness_limit() {
        let input = LiveInput {
            audio: [1.; 2],
            screen: [[255; 3]; 90],
            cpu: 1.,
            memory: 1.,
            hour: 23,
            minute: 59,
        };
        let presses = [Press {
            x: 3.,
            y: 2.,
            time: 0.,
        }; 32];
        for effect in 0..EFFECTS.len() {
            for palette in 0..3 {
                let s = Settings {
                    effect,
                    palette,
                    brightness: 0.25,
                    first: [255; 3],
                    second: [255; 3],
                    ..Default::default()
                };
                assert!(
                    render_live(&s, 0.2, 0.2, &presses, &input)
                        .iter()
                        .flatten()
                        .all(|&v| v <= 64),
                    "Parlaklık sınırı: {effect}/{palette}"
                );
            }
        }
    }
}

#[cfg(test)]
mod monochrome_regression {
    use super::*;
    #[test]
    fn single_colour_retains_gradient_based_motion_and_checker_contrast() {
        let live = LiveInput {
            audio: [0.7; 2],
            ..Default::default()
        };
        for effect in [1, 2, 3, 28, 29, 30, 31, 45, 46, 62] {
            let s = Settings {
                effect,
                palette: 1,
                first: [200, 0, 0],
                ..Default::default()
            };
            let a = render_live(&s, 0., 0., &[], &live);
            let b = render_live(&s, 1.4, 1.4, &[], &live);
            assert_ne!(a, b, "Tek renk hareketi kayboldu: {effect}");
            assert!(b.iter().all(|c| c[1] == 0 && c[2] == 0));
        }
        let s = Settings {
            effect: 48,
            palette: 1,
            ..Default::default()
        };
        let f = render(&s, 0., 0., &[]);
        assert_ne!(f[0], f[1]);
    }
}
