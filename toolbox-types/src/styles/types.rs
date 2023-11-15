pub enum ThemedColor {
    LightWhite,
    White,
    DarkWhite,

    LightBlack,
    Black,
    DarkBlack,

    LightRed,
    Red,
    DarkRed,

    LightBlue,
    Blue,
    DarkBlue,

    LightGreen,
    Green,
    DarkGreen,

    LightYellow,
    Yellow,
    DarkYellow,

    LightPurple,
    Purple,
    DarkPurple,

    LightPink,
    Pink,
    DarkPink,

    LightOrange,
    Orange,
    DarkOrange,
}

pub enum Color {
    Themed { color: ThemedColor, alpha: u8 },
    Rgba { r: u8, g: u8, b: u8, a: u8 },
    Hsva { h: u8, s: u8, v: u8, a: u8 },
}

pub enum GradientType {
    Linear(Direction),
    Radial,
    Conic,
}

pub struct Gradient {
    pub from: Color,
    pub to: Color,
    pub gradient_type: GradientType,
}

pub enum Graphic {
    Color(Color),
    Image(String),
    Gradient(Gradient),
    None,
}

pub enum Scale {
    Percent(f64),
    Em(f64),
    Points(f64),
    Pixels(f64),
    Auto,
}

pub enum Direction {
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

pub enum CardinalDirection {
    Top,
    Left,
    Right,
    Bottom,
}

pub enum FlexDirection {
    RowLeft,
    RowRight,
    ColumnUp,
    ColumnDown,
}

pub enum Layout {
    Absolute {
        x: Scale,
        y: Scale,
    },
    Flex {
        direction: FlexDirection,
    },
    GridCount {
        row_scale: u64,
    },
    GridScale {
        width: Option<Scale>,
        height: Option<Scale>,
    },
}

pub struct Margin {
    pub top: Scale,
    pub bottom: Scale,
    pub left: Scale,
    pub right: Scale,
}

pub enum FontWeight {
    ExtraLight,
    Light,
    Normal,
    Bold,
    ExtraBold,
    Black,
    Custom(u16),
}

pub struct Font {
    pub name: StyleString,
    pub weight: FontWeight,
    pub size: Scale,
    pub color: Color,
}

pub struct Transform {
    pub size_x: Scale,
    pub size_y: Scale,
    pub degrees: f64,
}

pub struct Corners {
    pub top_left: Scale,
    pub top_right: Option<Scale>,
    pub bottom_left: Option<Scale>,
    pub bottom_right: Option<Scale>,
    pub locked: bool,
}

pub enum BorderStyle {
    Dotted,
    Dashed(Scale),
    Straight,
}

pub struct BorderSide {
    pub color: Color,
    pub size: Scale,
    pub style: BorderStyle,
}

pub struct Border {
    pub left: Option<BorderSide>,
    pub right: Option<BorderSide>,
    pub top: Option<BorderSide>,
    pub bottom: Option<BorderSide>,
    pub corners: Corners,
}

pub enum StyleString {
    Static(&'static str),
    Dynamic(String),
}

pub enum StyleVec {}
