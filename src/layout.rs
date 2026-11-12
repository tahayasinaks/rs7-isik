//! Physical ANSI geometry shared by RGB projection and the keyboard preview.
#[derive(Clone, Debug)]
pub struct Key {
    pub legend: &'static str,
    pub secondary: &'static str,
    pub row: u8,
    pub col: u8,
    pub u: f32,
    pub width: f32,
}
pub const ROW_PITCH: f32 = 42. / 62.;
fn build_keys() -> Vec<Key> {
    let mut out = Vec::with_capacity(81);
    let mut push = |row, col, u, width, legend, secondary| {
        out.push(Key {
            row,
            col,
            u,
            width,
            legend,
            secondary,
        })
    };
    let f = [
        "Esc", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12", "Insert",
        "Delete",
    ];
    let x = [
        0., 1.25, 2.25, 3.25, 4.25, 5.5, 6.5, 7.5, 8.5, 9.75, 10.75, 11.75, 12.75, 14., 15.,
    ];
    for c in 0..15 {
        push(0, c as u8, x[c], 1., f[c], "");
    }
    let numbers = [
        ("`", "~"),
        ("1", "!"),
        ("2", "@"),
        ("3", "#"),
        ("4", "$"),
        ("5", "%"),
        ("6", "^"),
        ("7", "&"),
        ("8", "*"),
        ("9", "("),
        ("0", ")"),
        ("−", "_"),
        ("=", "+"),
    ];
    for (c, (l, s)) in numbers.iter().enumerate() {
        push(1, c as u8, c as f32, 1., l, s);
    }
    push(1, 13, 13., 2., "Backspace", "");
    push(1, 14, 15., 1., "End", "");
    push(2, 0, 0., 1.5, "Tab", "");
    for (c, l) in [
        "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "[", "]", "\\",
    ]
    .iter()
    .enumerate()
    {
        push(
            2,
            c as u8 + 1,
            1.5 + c as f32,
            1.,
            l,
            match *l {
                "[" => "{",
                "]" => "}",
                "\\" => "|",
                _ => "",
            },
        );
    }
    push(2, 14, 15., 1., "Page Up", "");
    push(3, 0, 0., 1.75, "Caps Lock", "");
    for (c, l) in ["A", "S", "D", "F", "G", "H", "J", "K", "L", ";", "'"]
        .iter()
        .enumerate()
    {
        push(
            3,
            c as u8 + 1,
            1.75 + c as f32,
            1.,
            l,
            match *l {
                ";" => ":",
                "'" => "\"",
                _ => "",
            },
        );
    }
    push(3, 13, 12.75, 2.25, "Enter", "");
    push(3, 14, 15., 1., "Page Down", "");
    push(4, 0, 0., 2.25, "Left Shift", "");
    for (c, l) in ["Z", "X", "C", "V", "B", "N", "M", ",", ".", "/"]
        .iter()
        .enumerate()
    {
        push(
            4,
            c as u8 + 1,
            2.25 + c as f32,
            1.,
            l,
            match *l {
                "," => "<",
                "." => ">",
                "/" => "?",
                _ => "",
            },
        );
    }
    push(4, 11, 12.25, 1.75, "Right Shift", "");
    push(4, 13, 14., 1., "↑", "");
    for (col, u, w, l) in [
        (0, 0., 1.25, "Ctrl"),
        (1, 1.25, 1.25, "Win"),
        (2, 2.5, 1.25, "Alt"),
        (5, 3.75, 6.25, "Space"),
        (9, 10., 1.25, "Alt"),
        (10, 11.25, 1.25, "Win Fn"),
        (11, 13., 1., "←"),
        (13, 14., 1., "↓"),
        (14, 15., 1., "→"),
    ] {
        push(5, col, u, w, l, "");
    }
    out
}
pub fn keys() -> &'static [Key] {
    static KEYS: std::sync::OnceLock<Vec<Key>> = std::sync::OnceLock::new();
    KEYS.get_or_init(build_keys)
}
/// Physical keycap bounds in horizontal key-pitch units; non-key LED slots stay absent.
pub fn led_rect(row: u8, col: u8) -> Option<(f32, f32, f32, f32)> {
    static RECTS: std::sync::OnceLock<[Option<(f32, f32, f32, f32)>; 90]> =
        std::sync::OnceLock::new();
    let rects = RECTS.get_or_init(|| {
        let mut r = [None; 90];
        for k in keys() {
            r[k.row as usize * 15 + k.col as usize] =
                Some((k.u, k.row as f32 * ROW_PITCH, k.width - 5. / 62., 37. / 62.));
        }
        r
    });
    if row >= 6 || col >= 15 {
        None
    } else {
        rects[row as usize * 15 + col as usize]
    }
}
