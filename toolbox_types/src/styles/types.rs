#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Locked {
    None,
    LeftRight,
    UpDown,
    Both,
    All,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Themed { color: ThemedColor, alpha: f32 },
    Rgba { r: u8, g: u8, b: u8, a: f32 },
    Hsla { h: u16, s: u8, v: u8, a: f32 },
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GradientType {
    Linear(u16),
    Radial,
    Conic,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Gradient {
    pub from: Color,
    pub to: Color,
    pub gradient_type: GradientType,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Graphic {
    Color(Color),
    Image {
        url: StyleString,
        size: Scale,
        repeat: bool,
    },
    Gradient(Gradient),
    None,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Scale {
    Percent(f64),
    Em(f64),
    Points(f64),
    Pixels(f64),
    Auto,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CardinalDirection {
    Top,
    Left,
    Right,
    Bottom,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlexDirection {
    RowLeft,
    RowRight,
    ColumnUp,
    ColumnDown,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Layout {
    None,
    Flex { direction: FlexDirection },
    GridCount { columns: u64 },
    GridScale { width: Scale, height: Option<Scale> },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Margin {
    pub top: Scale,
    pub bottom: Scale,
    pub left: Scale,
    pub right: Scale,
    pub locked: Locked,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FontWeight {
    ExtraLight,
    Light,
    Normal,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
    Custom(u16),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Font {
    pub name: StyleString,
    pub weight: FontWeight,
    pub size: Scale,
    pub color: Color,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub size_x: Scale,
    pub size_y: Scale,
    pub pos_x: Scale,
    pub pos_y: Scale,
    pub anchor: Direction,
    pub degrees: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Corners {
    pub top_left: Scale,
    pub top_right: Option<Scale>,
    pub bottom_left: Option<Scale>,
    pub bottom_right: Option<Scale>,
    pub locked: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BorderStyle {
    Dotted,
    Dashed(Scale),
    Straight,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BorderSide {
    pub color: Color,
    pub size: Scale,
    pub style: BorderStyle,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Border {
    pub left: Option<BorderSide>,
    pub right: Option<BorderSide>,
    pub top: Option<BorderSide>,
    pub bottom: Option<BorderSide>,
    pub corners: Corners,
    pub locked: Locked,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StyleString {
    Static(&'static str),
    Dynamic(String),
}

impl ToString for StyleString {
    fn to_string(&self) -> String {
        match self {
            StyleString::Static(s) => s.to_string(),
            StyleString::Dynamic(s) => s.clone(),
        }
    }
}

pub enum StyleVec {}
