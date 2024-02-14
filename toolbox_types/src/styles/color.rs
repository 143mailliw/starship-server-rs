use super::types::{Color, ThemedColor};

pub fn light_color_from_themed(color: Color) -> Color {
    match color {
        Color::Themed { color, alpha } => match color {
            ThemedColor::LightWhite => Color::Rgba {
                r: 249,
                g: 250,
                b: 251,
                a: alpha,
            },
            ThemedColor::White => Color::Rgba {
                r: 241,
                g: 245,
                b: 249,
                a: alpha,
            },
            ThemedColor::DarkWhite => Color::Rgba {
                r: 226,
                g: 232,
                b: 240,
                a: alpha,
            },
            ThemedColor::LightBlack => Color::Rgba {
                r: 51,
                g: 65,
                b: 85,
                a: alpha,
            },
            ThemedColor::Black => Color::Rgba {
                r: 30,
                g: 41,
                b: 59,
                a: alpha,
            },
            ThemedColor::DarkBlack => Color::Rgba {
                r: 17,
                g: 24,
                b: 39,
                a: alpha,
            },
            ThemedColor::LightRed => Color::Rgba {
                r: 248,
                g: 113,
                b: 113,
                a: alpha,
            },
            ThemedColor::Red => Color::Rgba {
                r: 239,
                g: 68,
                b: 68,
                a: alpha,
            },
            ThemedColor::DarkRed => Color::Rgba {
                r: 185,
                g: 28,
                b: 28,
                a: alpha,
            },
            ThemedColor::LightBlue => Color::Rgba {
                r: 96,
                g: 165,
                b: 250,
                a: alpha,
            },
            ThemedColor::Blue => Color::Rgba {
                r: 59,
                g: 130,
                b: 246,
                a: alpha,
            },
            ThemedColor::DarkBlue => Color::Rgba {
                r: 29,
                g: 78,
                b: 216,
                a: alpha,
            },
            ThemedColor::LightGreen => Color::Rgba {
                r: 134,
                g: 239,
                b: 172,
                a: alpha,
            },
            ThemedColor::Green => Color::Rgba {
                r: 34,
                g: 197,
                b: 94,
                a: alpha,
            },
            ThemedColor::DarkGreen => Color::Rgba {
                r: 21,
                g: 128,
                b: 61,
                a: alpha,
            },
            ThemedColor::LightYellow => Color::Rgba {
                r: 254,
                g: 240,
                b: 138,
                a: alpha,
            },
            ThemedColor::Yellow => Color::Rgba {
                r: 253,
                g: 224,
                b: 71,
                a: alpha,
            },
            ThemedColor::DarkYellow => Color::Rgba {
                r: 234,
                g: 179,
                b: 8,
                a: alpha,
            },
            ThemedColor::LightPurple => Color::Rgba {
                r: 216,
                g: 180,
                b: 254,
                a: alpha,
            },
            ThemedColor::Purple => Color::Rgba {
                r: 192,
                g: 132,
                b: 252,
                a: alpha,
            },
            ThemedColor::DarkPurple => Color::Rgba {
                r: 147,
                g: 51,
                b: 234,
                a: alpha,
            },
            ThemedColor::LightPink => Color::Rgba {
                r: 251,
                g: 207,
                b: 232,
                a: alpha,
            },
            ThemedColor::Pink => Color::Rgba {
                r: 249,
                g: 168,
                b: 212,
                a: alpha,
            },
            ThemedColor::DarkPink => Color::Rgba {
                r: 244,
                g: 114,
                b: 182,
                a: alpha,
            },
            ThemedColor::LightOrange => Color::Rgba {
                r: 253,
                g: 186,
                b: 116,
                a: alpha,
            },
            ThemedColor::Orange => Color::Rgba {
                r: 249,
                g: 115,
                b: 22,
                a: alpha,
            },
            ThemedColor::DarkOrange => Color::Rgba {
                r: 234,
                g: 88,
                b: 12,
                a: alpha,
            },
        },
        _ => panic!("theme color"),
    }
}
