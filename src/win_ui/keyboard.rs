//! ANSI 75% geometry: physical key sizes and logical LED coordinates are kept separate.
use super::theme::{label, rect, rgb, BG};
use super::*;
use crate::layout::{keys, Key};
const LEFT: i32 = 76;
const TOP: i32 = 184;
const UNIT: f32 = 62.;
const STEP: i32 = 42;

fn bounds(k: &Key) -> (i32, i32, i32, i32) {
    (
        LEFT + (k.u * UNIT).round() as i32,
        TOP + k.row as i32 * STEP,
        (k.width * UNIT).round() as i32 - 5,
        37,
    )
}
pub fn hit(x: i32, y: i32) -> Option<(f32, f32)> {
    keys()
        .iter()
        .find(|k| {
            let (a, b, w, h) = bounds(k);
            x >= a && x < a + w && y >= b && y < b + h
        })
        .map(|k| (k.col as f32, k.row as f32))
}
unsafe fn legend(dc: HDC, k: &Key, x: i32, y: i32, w: i32, h: i32) {
    let mut r = RECT {
        left: x + 4,
        top: y + 2,
        right: x + w - 4,
        bottom: y + h - 2,
    };
    let size = if k.legend.chars().count() > 6 {
        10
    } else if k.legend.chars().count() > 2 {
        11
    } else {
        13
    };
    let font = CreateFontW(
        -size,
        0,
        0,
        0,
        500,
        0,
        0,
        0,
        DEFAULT_CHARSET as u32,
        0,
        0,
        CLEARTYPE_QUALITY as u32,
        0,
        wide("Segoe UI").as_ptr(),
    );
    let old = SelectObject(dc, font);
    SetTextColor(dc, 0xeeeae6);
    SetBkMode(dc, TRANSPARENT as i32);
    let text = wide(k.legend);
    DrawTextW(
        dc,
        text.as_ptr(),
        -1,
        &mut r,
        DT_CENTER | DT_VCENTER | DT_SINGLELINE,
    );
    SelectObject(dc, old);
    DeleteObject(font);
    if !k.secondary.is_empty() {
        label(dc, k.secondary, x + w - 14, y + 3, 8, 0x85817c, false);
    }
    if matches!(k.legend, "F" | "J") {
        rect(dc, x + w / 2 - 6, y + h - 7, 12, 1, 0x55534e, 0x55534e, 0);
    }
}
pub unsafe fn paint(dc: HDC, frame: &lighting::Frame, heart_mode: bool) {
    rect(dc, 64, 176, 1016, 280, 0x080808, 0x080808, 20);
    rect(dc, 64, 170, 1016, 280, 0x242323, 0x403e3b, 20);
    rect(dc, 70, 176, 1004, 268, 0x191818, 0x292827, 12);
    rect(dc, 73, 181, 998, 258, BG, 0x101010, 8);
    for k in keys() {
        let (x, y, w, h) = bounds(&k);
        let i = (0..90)
            .find(|&i| lighting::coord(i) == (k.row, k.col))
            .unwrap();
        let c = frame[i];
        let strength = if heart_mode { 2 } else { 5 };
        let tint = rgb([
            18 + c[0] / strength,
            19 + c[1] / strength,
            21 + c[2] / strength,
        ]);
        let edge = rgb([48 + c[0] / 3, 48 + c[1] / 3, 47 + c[2] / 3]);
        rect(dc, x, y + 3, w, h, 0x080808, 0x080808, 5);
        rect(dc, x, y, w, h - 1, 0x1c1b1b, edge, 6);
        rect(dc, x + 1, y + 1, w - 2, h - 4, tint, tint, 5);
        legend(dc, &k, x, y, w, h - 2);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn physical_keys_map_to_unique_real_leds() {
        let keys = keys();
        assert_eq!(keys.len(), 81);
        let mut used = std::collections::HashSet::new();
        for k in keys {
            assert!(used.insert((k.row, k.col)));
            assert!((0..90).any(|i| lighting::coord(i) == (k.row, k.col)));
        }
    }
    #[test]
    fn wide_spacebar_and_shift_hit_their_actual_logical_key() {
        assert_eq!(
            hit(LEFT + (3.9 * UNIT) as i32, TOP + 5 * STEP + 15),
            Some((5., 5.))
        );
        assert_eq!(
            hit(LEFT + (9.8 * UNIT) as i32, TOP + 5 * STEP + 15),
            Some((5., 5.))
        );
        assert_eq!(
            hit(LEFT + (1.9 * UNIT) as i32, TOP + 4 * STEP + 15),
            Some((0., 4.))
        );
        assert_eq!(hit(1040, 370), None);
    }
}
