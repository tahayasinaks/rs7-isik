//! Original procedural implementations; no vendor effect code or paid assets are embedded.
use super::*;
use std::f32::consts::TAU;
fn band(d: f32, width: f32) -> f32 {
    (1. - d.abs() / width).clamp(0., 1.)
}
fn smooth(n: f32) -> f32 {
    let i = n.floor();
    let f = n - i;
    let a = f * f * (3. - 2. * f);
    noise(i) * (1. - a) + noise(i + 1.) * a
}
fn field(x: f32, y: f32, t: f32) -> f32 {
    (smooth(x * 0.43 + t * 0.27) + smooth(y * 0.71 - t * 0.19) + smooth((x + y) * 0.31 + t * 0.13))
        / 3.
}
fn distance(x: f32, y: f32, cx: f32, cy: f32) -> f32 {
    ((x - cx).powi(2) + ((y - cy) * 1.3).powi(2)).sqrt()
}
fn heartbeat(t: f32) -> f32 {
    let u = t.rem_euclid(2.5);
    (-((u - 0.3) * 9.).powi(2)).exp() + 0.65 * (-((u - 0.73) * 11.).powi(2)).exp()
}
fn heart_shape(x: f32, y: f32) -> bool {
    let a = x * x + y * y - 1.;
    a * a * a - x * x * y * y * y <= 0.
}
fn heart_coverage(row: u8, col: u8) -> f32 {
    let Some((left, top, width, height)) = crate::layout::led_rect(row, col) else {
        return 0.;
    };
    // Anchor the cusp on N, not on a rectangular matrix column. Horizontal area
    // sampling softens the contour without filling the notch from a neighbouring row.
    let (tip_left, _, tip_width, _) = crate::layout::led_rect(4, 6).unwrap();
    let centre_x = tip_left + tip_width * 0.5;
    let y = (1.91 - (top + height * 0.5)) / 1.35;
    let mut inside = 0.;
    for n in 0..17 {
        let px = left + width * (0.06 + 0.88 * (n as f32 + 0.5) / 17.);
        if heart_shape((px - centre_x) / 2.9, y) {
            inside += 1.;
        }
    }
    inside / 17.
}
fn clock_pixel(x: usize, y: usize, h: u8, m: u8) -> bool {
    const DIGITS: [[u8; 5]; 10] = [
        [7, 5, 5, 5, 7],
        [2, 6, 2, 2, 7],
        [7, 1, 7, 4, 7],
        [7, 1, 7, 1, 7],
        [5, 5, 7, 1, 1],
        [7, 4, 7, 1, 7],
        [7, 4, 7, 5, 7],
        [7, 1, 1, 1, 1],
        [7, 5, 7, 5, 7],
        [7, 5, 7, 1, 7],
    ];
    if y >= 5 {
        return false;
    }
    let digits = [h / 10, h % 10, m / 10, m % 10];
    let block = x / 4;
    let col = x % 4;
    block < 4 && col < 3 && (DIGITS[digits[block] as usize % 10][y] & (1 << (2 - col))) != 0
}
pub(super) fn render(
    s: &Settings,
    t: f32,
    now: f32,
    presses: &[Press],
    input: &LiveInput,
) -> Frame {
    std::array::from_fn(|i| {
        let (row, col) = coord(i);
        let (x, y) = (col as f32, row as f32);
        let (dx, dy) = (x - 7., (y - 2.5) * 1.3);
        let r = (dx * dx + dy * dy).sqrt();
        let angle = dy.atan2(dx) / TAU;
        let key = row as f32 * 15. + x;
        if s.effect == 17 {
            let heat = presses
                .iter()
                .filter_map(|p| {
                    let age = (now - p.time) * s.speed.clamp(0.2, 3.);
                    if (0.0..10.).contains(&age) && distance(x, y, p.x, p.y) < 0.25 {
                        Some((1. - age / 10.) * 0.3)
                    } else {
                        None
                    }
                })
                .sum::<f32>()
                .clamp(0., 1.);
            return palette(s, 0.65 - heat * 0.65, heat);
        }
        if reactive(s.effect) {
            let mut out = [0u8; 3];
            for p in presses {
                let age = (now - p.time) * s.speed.clamp(0.2, 3.);
                if !(0.0..10.).contains(&age) {
                    continue;
                }
                let d = distance(x, y, p.x, p.y);
                let fade = (1. - age / 2.2).clamp(0., 1.);
                let value = match s.effect {
                    12 => {
                        if d < 0.25 {
                            fade.powi(2)
                        } else {
                            0.
                        }
                    }
                    13 => {
                        let cross = band(x - p.x, 0.6).max(band(y - p.y, 0.6));
                        cross * band(d - age * 9., 1.4) * fade
                    }
                    14 => {
                        let rise = p.y - y;
                        if rise >= 0. {
                            band(x - p.x + (rise * 2. + age * 6.).sin() * 0.3, 1.5)
                                * band(rise - age * 5., 1.5)
                                * fade
                        } else {
                            0.
                        }
                    }
                    15 => {
                        if d < age * 10. + 0.6 {
                            fade * (1. - d / 20.).max(0.)
                        } else {
                            0.
                        }
                    }
                    16 => fade.powi(3),
                    17 => {
                        if d < 0.25 {
                            (1. - age / 10.).max(0.) * 0.3
                        } else {
                            0.
                        }
                    }
                    _ => 0.,
                };
                let c = palette(s, p.x / 15. + p.y / 6. + age * 0.15, value);
                for k in 0..3 {
                    out[k] = if s.effect == 17 {
                        out[k].saturating_add(c[k])
                    } else {
                        out[k].max(c[k])
                    };
                }
            }
            return out;
        }
        if (57..=58).contains(&s.effect) {
            let c = if s.effect == 57 {
                input.screen[i]
            } else {
                std::array::from_fn(|k| {
                    (input.screen.iter().map(|c| c[k] as u32).sum::<u32>() / 90) as u8
                })
            };
            return c.map(|n| (n as f32 * s.brightness.clamp(0., 1.)).round() as u8);
        }
        let audio = input
            .audio
            .iter()
            .copied()
            .fold(0., f32::max)
            .clamp(0., 1.)
            .sqrt();
        let (h, v) = match s.effect {
            18 => {
                let n = noise(t.floor());
                (n, 0.4 + 0.6 * n)
            }
            19 => (0.1, (t.rem_euclid(2.) < 0.8) as u8 as f32),
            20 => {
                let u = t.rem_euclid(2.);
                (0.1, ((u < 0.16) || (0.32..0.48).contains(&u)) as u8 as f32)
            }
            21 => (0.98, heartbeat(t)),
            22 => (x / 16., band(x - (t * 5.).rem_euclid(19.) + 2., 1.6)),
            23 => (x / 16., band(x - (0.5 + 0.5 * (t * 0.9).sin()) * 14., 2.1)),
            24 => {
                let head = (t * 8.).rem_euclid(21.);
                let d = head - x - y * 0.5;
                (
                    0.56,
                    if (0.0..6.).contains(&d) {
                        (1. - d / 6.).powi(2)
                    } else {
                        0.
                    },
                )
            }
            25 => (key / 90., ((key - t * 9.).rem_euclid(7.) < 2.) as u8 as f32),
            26 => (
                x / 16.,
                (x < ((t * 0.6).sin() * 0.5 + 0.5) * 16.) as u8 as f32,
            ),
            27 => {
                let step = (t * 4.).floor() as i32;
                let stack = step.rem_euclid(16);
                let moving = (t * 14.).rem_euclid((15 - stack).max(1) as f32);
                (
                    y / 6.,
                    ((x >= 15. - stack as f32) || (x - moving).abs() < 0.7) as u8 as f32,
                )
            }
            28 => (
                ((col + row) % 2) as f32 * 0.5 + (t * 0.75).floor() * 0.5,
                1.,
            ),
            29 => (angle - t * 0.13, 1.),
            30 => (angle + r * 0.09 - t * 0.16, 1.),
            31 => (r * 0.13 - t * 0.17, 1.),
            32 => {
                let d = (angle - t * 0.12).rem_euclid(1.);
                (0.32, (1. - d / 0.16).max(0.))
            }
            33 => {
                let a = band(dx - (t * 1.3).sin() * 6., 0.9);
                let b = band(dy - (t * 0.8).cos() * 3., 0.8);
                (if a > b { 0.58 } else { 0.88 }, a.max(b))
            }
            34 => {
                let a = ((y - 2.5) * 1.3).atan2(x - 3.) / TAU;
                let b = ((y - 2.5) * 1.3).atan2(x - 11.) / TAU;
                (
                    if x < 7. { a - t * 0.17 } else { b + t * 0.17 },
                    0.65 + 0.35 * (r - t).cos(),
                )
            }
            35 => {
                let bx = 7. + 7. * (t * 0.7).sin();
                let by = 5. - 5. * (t * 1.4).sin().abs();
                (t * 0.1, band(distance(x, y, bx, by), 1.5))
            }
            36 => {
                let mut v: f32 = 0.;
                let mut h = 0.;
                for n in 0..4 {
                    let n = n as f32;
                    let b = band(
                        distance(
                            x,
                            y,
                            7. + 6. * (t * 0.5 + n * 1.7).sin(),
                            2.5 + 2.5 * (t * 0.7 + n).cos(),
                        ),
                        1.3,
                    );
                    if b > v {
                        v = b;
                        h = n / 4.;
                    }
                }
                (h, v)
            }
            37 => {
                let mut v: f32 = 0.;
                for n in 0..3 {
                    let n = n as f32;
                    let age = (t * 0.6 + n * 0.7).rem_euclid(3.);
                    let d = distance(x, y, 2. + n * 5., 6. - age * 3.);
                    v = v.max(band(d - 0.5 - age * 0.3, 0.55) * (1. - age / 3.));
                }
                (0.52, v)
            }
            38 => {
                let burst = (t * 0.6).floor();
                let age = (t * 0.6).fract() * 2.;
                let cx = 2. + noise(burst + 3.) * 10.;
                let cy = 1. + noise(burst + 13.) * 3.;
                let d = distance(x, y, cx, cy);
                let spokes = ((y - cy).atan2(x - cx) * 5.).cos().abs().powi(4);
                (
                    noise(burst) + d * 0.04,
                    band(d - age * 5., 1.2) * spokes * (1. - age / 2.),
                )
            }
            39 => {
                let event = (t * 2.).floor();
                let flash = noise(event) > 0.73;
                let bolt =
                    (x - (noise(event + 19.) * 10. + (y * 2. + event).sin() * 1.5)).abs() < 0.8;
                (
                    0.62,
                    if flash {
                        if bolt {
                            1.
                        } else {
                            0.12
                        }
                    } else {
                        0.
                    },
                )
            }
            40 => {
                let sunrise = (t * 0.04).rem_euclid(1.);
                (0.01 + y * 0.02, (sunrise * 8. - (5. - y)).clamp(0., 1.))
            }
            41 => (0.065, 0.4 + 0.3 * smooth(t * 6.) + 0.15 * field(x, y, t)),
            42 => {
                let n = field(x * 0.7, y * 0.8, t * 0.6);
                (0.005 + n * 0.12, (n * 1.7 - 0.2).clamp(0., 1.))
            }
            43 => {
                let n =
                    ((x * 0.7 + t).sin() + (y * 1.1 - t * 0.8).sin() + ((r * 0.6) - t * 0.5).sin())
                        / 3.;
                (n * 0.5 + t * 0.05, 0.8)
            }
            44 => {
                let n = field(x, y, t);
                (0.5 + n * 0.3, n.powi(2))
            }
            45 => (
                noise((col / 2) as f32 + (row / 2) as f32 * 8. + (t * 0.8).floor() * 35.),
                1.,
            ),
            46 => (
                ((x - t * 2.) / 5.).floor() * 0.19 + ((y + t * 0.5) / 3.).floor() * 0.33,
                0.9,
            ),
            47 => {
                let mut n = 0.;
                for octave in 0..4 {
                    let k = 2f32.powi(octave);
                    n += ((x * k * 0.23 + t / k).sin() * (y * k * 0.41 - t / k).cos()) / k;
                }
                (n * 0.3 + t * 0.03, 0.5 + 0.25 * n)
            }
            48 => (((col + row) % 2) as f32 * 0.5, 1.),
            49 => (r * 0.1, band(r - (3.5 + 3. * (t * 0.8).sin()), 0.8)),
            50 => {
                let petals = 2.5 + 1.8 * (angle * TAU * 5. + t).cos();
                (angle + t * 0.08, band(r - petals, 1.4))
            }
            51 => {
                let a = distance(x, y, 4., 2.);
                let b = distance(x, y, 10., 3.);
                ((a - b) * 0.15 + t * 0.1, 0.5 + 0.5 * (a + b - t * 2.).sin())
            }
            52 => {
                let path = if row % 2 == 0 {
                    key
                } else {
                    row as f32 * 15. + 14. - x
                };
                let d = ((t * 12.) - path).rem_euclid(100.);
                (path / 90., (1. - d / 12.).max(0.))
            }
            53 => (t * 0.1, audio),
            54 => {
                let channel = if col < 7 { 0 } else { 1 };
                let a = input.audio[channel].clamp(0., 1.).sqrt();
                (y * 0.1, (6. - y <= a * 6. + 0.05) as u8 as f32)
            }
            55 => (
                x / 18. - t * 0.14,
                audio * (0.5 + 0.5 * (x * 0.6 - t * 3.).sin()),
            ),
            56 => {
                let seed = noise(key);
                (
                    seed,
                    if seed < audio {
                        (t * 2. + seed * 40.).sin().max(0.)
                    } else {
                        0.
                    },
                )
            }
            59 => (
                0.33 * (1. - input.cpu.clamp(0., 1.)),
                (key < 90. * input.cpu.clamp(0., 1.)) as u8 as f32,
            ),
            60 => (
                0.55 + input.memory.clamp(0., 1.) * 0.3,
                (key < 90. * input.memory.clamp(0., 1.)) as u8 as f32,
            ),
            61 => (
                t * 0.02,
                clock_pixel(
                    col as usize,
                    row as usize,
                    input.hour.min(23),
                    input.minute.min(59),
                ) as u8 as f32,
            ),
            62 => (
                noise(((x / 3.).floor() + (y / 2.).floor() * 5.) + (t * 2.).floor() * 14.),
                audio,
            ),
            63 => (r * 0.07 + t * 0.1, band(r - audio * 8., 1.) * audio),
            64 => (
                0.98,
                heart_coverage(row, col) * (0.35 + 0.65 * heartbeat(t)),
            ),
            _ => (0., 0.),
        };
        palette(s, h, v)
    })
}

#[cfg(test)]
mod heart_tests {
    use super::*;
    #[test]
    fn analytic_heart_is_mirrored_with_a_notch_and_single_cusp() {
        for y in [-1.2, -0.8, 0., 0.7, 1.2] {
            for x in [-1.2, -0.6, -0.2, 0., 0.4, 0.9] {
                assert_eq!(heart_shape(x, y), heart_shape(-x, y));
            }
        }
        assert!(!heart_shape(0., 1.2));
        assert!(heart_shape(-0.6, 1.2));
        assert!(heart_shape(0.6, 1.2));
        assert!(heart_shape(0., -0.95));
        assert!(!heart_shape(0.6, -0.95));
    }
    #[test]
    fn physical_cusp_is_on_n_and_unused_slots_are_dark() {
        assert!(heart_coverage(4, 6) > 0.8);
        assert!(heart_coverage(4, 6) > heart_coverage(4, 5));
        assert!(heart_coverage(4, 6) > heart_coverage(4, 7));
        assert_eq!(heart_coverage(0, 7), 0.);
        assert_eq!(heart_coverage(5, 5), 0.);
        for r in 0..6 {
            for c in 0..15 {
                let v = heart_coverage(r, c);
                assert!((0.0..=1.).contains(&v));
                if crate::layout::led_rect(r, c).is_none() {
                    assert_eq!(v, 0.);
                }
            }
        }
    }
    #[test]
    fn heart_pulses_and_keeps_background_dark() {
        let s = Settings {
            effect: 64,
            brightness: 1.,
            ..Default::default()
        };
        let a = render(&s, 0.3, 0.3, &[], &LiveInput::default());
        let b = render(&s, 1.4, 1.4, &[], &LiveInput::default());
        assert_ne!(a, b);
        assert_eq!(a[0], [0; 3]);
        assert_eq!(a[7], [0; 3]);
        assert_ne!(a[66], [0; 3]);
    }
}
