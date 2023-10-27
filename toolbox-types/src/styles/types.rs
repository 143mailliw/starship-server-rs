enum Color {
    RGBA { r: u8, g: u8, b: u8, a: u8 },
    HSVA { h: u8, s: u8, v: u8, a: u8 },
}

enum Scale {
    Percent(f64),
    Em(f64),
    Points(f64),
    Pixels(f64),
    Auto,
}

enum Direction {
    TopLeft,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

enum FlexDirection {
    RowLeft,
    RowRight,
    ColumnUp,
    ColumnDown,
}

enum Layout {
    Absolute {
        x: Scale,
        y: Scale,
    },
    Flex {
        direction: FlexDirection,
    },
    GridCount {
        rowScale: u64,
    },
    GridScale {
        width: Option<Scale>,
        height: Option<Scale>,
    },
}

struct Margin {
    top: Scale,
    bottom: Scale,
    left: Scale,
    right: Scale,
}

enum FontWeight {
    ExtraLight,
    Light,
    Normal,
    Bold,
    ExtraBold,
    Black,
    Custom(u16),
}

struct Font {
    name: String,
    weight: FontWeight,
    size: Scale,
    color: Color,
}

struct Transform {
    size_x: Scale,
    size_y: Scale,
    degrees: f64,
}
