use std::fmt::format;

use toolbox_types::styles::{
    color::light_color_from_themed,
    stylesheet::{StyleOption, Stylesheet},
    types,
};

trait ToCSS {
    /// Transform this object into a CSS string that can be used for HTML rendering.
    fn to_css(&self) -> String;
}

//
// Values
//

impl ToCSS for types::Color {
    fn to_css(&self) -> String {
        match self {
            Self::Hsla { h, s, v, a } => format!("hsl({h}deg {s}% {v}% / {a}"),
            Self::Rgba { r, g, b, a } => format!("rgb({r}, {g}, {b} / {a})"),
            Self::Themed { color, alpha } => light_color_from_themed(types::Color::Themed {
                color: *color,
                alpha: *alpha,
            })
            .to_css(),
        }
    }
}

impl ToCSS for types::Scale {
    fn to_css(&self) -> String {
        match self {
            Self::Em(em) => format!("{em}em"),
            Self::Percent(percent) => format!("{percent}%"),
            Self::Pixels(px) => format!("{px}px"),
            Self::Points(pt) => format!("{pt}pt"),
            Self::Auto => "auto".to_string(),
        }
    }
}

impl ToCSS for types::FontWeight {
    fn to_css(&self) -> String {
        match self {
            Self::ExtraLight => 200,
            Self::Light => 300,
            Self::Normal => 400,
            Self::SemiBold => 600,
            Self::Bold => 700,
            Self::ExtraBold => 800,
            Self::Black => 900,
            Self::Custom(weight) => *weight,
        }
        .to_string()
    }
}

impl ToCSS for types::Margin {
    fn to_css(&self) -> String {
        match self.locked {
            types::Locked::None => format!(
                "{} {} {} {}",
                self.top.to_css(),
                self.right.to_css(),
                self.bottom.to_css(),
                self.left.to_css(),
            ),
            types::Locked::LeftRight => format!(
                "{} {} {} {}",
                self.top.to_css(),
                self.left.to_css(),
                self.bottom.to_css(),
                self.left.to_css(),
            ),
            types::Locked::UpDown => format!(
                "{} {} {} {}",
                self.top.to_css(),
                self.right.to_css(),
                self.top.to_css(),
                self.left.to_css(),
            ),
            types::Locked::Both => format!("{} {}", self.top.to_css(), self.left.to_css()),
            types::Locked::All => self.top.to_css(),
        }
    }
}

impl ToCSS for types::Gradient {
    fn to_css(&self) -> String {
        match self.gradient_type {
            types::GradientType::Linear(angle) => format!(
                "linear-gradient({}deg, {}, {})",
                angle,
                self.from.to_css(),
                self.to.to_css()
            ),
            types::GradientType::Radial => format!(
                "radial-gradient({}, {})",
                self.from.to_css(),
                self.to.to_css()
            ),
            types::GradientType::Conic => format!(
                "conic-gradient({}, {})",
                self.from.to_css(),
                self.to.to_css()
            ),
        }
    }
}

impl ToCSS for types::Graphic {
    fn to_css(&self) -> String {
        match self {
            Self::Image { url, size, repeat } => format!(
                "background-image: url({}); background-size: {}; background-repeat: {};",
                url.to_string(),
                size.to_css(),
                if *repeat { "repeat" } else { "no-repeat" }
            ),
            Self::Gradient(gradient) => format!("background: {};", gradient.to_css()),
            Self::Color(color) => format!("background-color: {};", color.to_css()),
            Self::None => "background: none;".to_string(),
        }
    }
}

//
// Property-specific
//

impl ToCSS for types::Layout {
    fn to_css(&self) -> String {
        match self {
            Self::None => "display: block;".to_string(),
            Self::Flex { direction } => format!(
                "display: flex; flex-direction: {};",
                match direction {
                    types::FlexDirection::RowLeft => "row",
                    types::FlexDirection::RowRight => "row-reverse",
                    types::FlexDirection::ColumnDown => "column",
                    types::FlexDirection::ColumnUp => "column-reverse",
                }
            ),
            Self::GridCount { columns } => format!(
                "display: grid; grid-template-columns: repeat({}, 1fr);",
                columns
            ),
            Self::GridScale { width, height } => {
                let mut properties: Vec<String> = vec![];

                properties.push("display: grid;".to_string());
                properties.push(format!(
                    "grid-template-columns: repeat(auto-fill, minmax({}, 1fr));",
                    width.to_css()
                ));

                if let Some(h) = height {
                    properties.push(format!(
                        "grid-template-rows: repeat(auto-fill, minmax({}, 1fr));",
                        h.to_css()
                    ));
                }

                properties.join(" ")
            }
        }
    }
}

// this is not the CSS transform property, this is a layout transform that describes the
// size and position of an element
impl ToCSS for types::Transform {
    fn to_css(&self) -> String {
        let mut properties: Vec<String> = vec![];

        properties.push(format!(
            "width: {}, height: {};",
            self.size_x.to_css(),
            self.size_y.to_css()
        ));

        match self.anchor {
            types::Direction::TopLeft => {
                properties.push(format!(
                    "top: {}; left: {};",
                    self.pos_y.to_css(),
                    self.pos_x.to_css()
                ));
            }
            types::Direction::Top => {
                properties.push(format!(
                    "top: {}; left: calc(50% + {}); transform: translateX(-50%);",
                    self.pos_y.to_css(),
                    self.pos_x.to_css()
                ));
            }
            types::Direction::TopRight => {
                properties.push(format!(
                    "top: {}; right: {};",
                    self.pos_y.to_css(),
                    self.pos_x.to_css()
                ));
            }
            types::Direction::Left => {
                properties.push(format!(
                    "top: calc(50% + {}); left: {}; transform: translateY(-50%);",
                    self.pos_y.to_css(),
                    self.pos_x.to_css()
                ));
            }
            types::Direction::Center => {
                properties.push(format!(
                    "top: calc(50% + {}); left: calc(50% + {}); transform: translate(-50%, -50%);",
                    self.pos_y.to_css(),
                    self.pos_x.to_css()
                ));
            }
            types::Direction::Right => {
                properties.push(format!(
                    "top: calc(50% + {}); right: {}; transform: translateY(-50%);",
                    self.pos_y.to_css(),
                    self.pos_x.to_css()
                ));
            }
            types::Direction::BottomLeft => {
                properties.push(format!(
                    "bottom: {}; left: {};",
                    self.pos_y.to_css(),
                    self.pos_x.to_css()
                ));
            }
            types::Direction::Bottom => {
                properties.push(format!(
                    "bottom: {}; left: calc(50% + {}); transform: translateX(-50%);",
                    self.pos_y.to_css(),
                    self.pos_x.to_css()
                ));
            }
            types::Direction::BottomRight => {
                properties.push(format!(
                    "bottom: {}; right: {};",
                    self.pos_y.to_css(),
                    self.pos_x.to_css()
                ));
            }
        }

        properties.join(" ")
    }
}

impl ToCSS for types::Font {
    fn to_css(&self) -> String {
        format!(
            "font-family: {}; font-size: {}; font-weight: {}; color: {};",
            self.name.to_string(),
            self.size.to_css(),
            self.weight.to_css(),
            self.color.to_css()
        )
    }
}

impl ToCSS for types::BorderSide {
    fn to_css(&self) -> String {
        format!(
            "{} {} {}",
            self.size.to_css(),
            match self.style {
                types::BorderStyle::Straight => "solid",
                types::BorderStyle::Dashed => "dashed",
                types::BorderStyle::Dotted => "dotted",
            },
            self.color.to_css()
        )
    }
}

impl ToCSS for types::Border {
    fn to_css(&self) -> String {
        match self.locked {
            types::Locked::None => format!(
                "border-left: {}; border-right: {}; border-top: {}; border-bottom: {};",
                self.left
                    .expect("Locked::None should result in all sides having values")
                    .to_css(),
                self.right
                    .expect("Locked::None should result in all sides having values")
                    .to_css(),
                self.top
                    .expect("Locked::None should result in all sides having values")
                    .to_css(),
                self.bottom
                    .expect("Locked::None should result in all sides having values")
                    .to_css(),
            ),
            types::Locked::LeftRight => format!(
                "border-left: {}; border-right: {}; border-top: {}; border-bottom: {};",
                self.left
                    .expect("Locked::LeftRight should result in the left, top and bottom sides having values")
                    .to_css(),
                self.left
                    .expect("Locked::LeftRight should result in the left, top and bottom sides having values")
                    .to_css(),
                self.top
                    .expect("Locked::LeftRight should result in the left, top and bottom sides having values")
                    .to_css(),
                self.bottom
                    .expect("Locked::LeftRight should result in the left, top and bottom sides having values")
                    .to_css(),
            ),
            types::Locked::UpDown => format!(
                "border-left: {}; border-right: {}; border-top: {}; border-bottom: {};",
                self.left
                    .expect("Locked::UpDown should result in the left, right and top sides having values")
                    .to_css(),
                self.right
                    .expect("Locked::UpDown should result in the left, right and top sides having values")
                    .to_css(),
                self.top
                    .expect("Locked::UpDown should result in the left, right and top sides having values")
                    .to_css(),
                self.top
                    .expect("Locked::UpDown should result in the left, right and top sides having values")
                    .to_css(),
            ),
            types::Locked::Both => format!(
                "border-left: {}; border-right: {}; border-top: {}; border-bottom: {};",
                self.left
                    .expect("Locked::Both should result in the left and top sides having values")
                    .to_css(),
                self.left
                    .expect("Locked::Both should result in the left and top sides having values")
                    .to_css(),
                self.top
                    .expect("Locked::Both should result in the left and top sides having values")
                    .to_css(),
                self.top
                    .expect("Locked::Both should result in the left and top sides having values")
                    .to_css(),
            ),
            types::Locked::All => format!(
                "border: {};",
                self.top.expect("Locked::All stores its value in the top side").to_css()
            ),
        }
    }
}

impl ToCSS for types::TextAlignment {
    fn to_css(&self) -> String {
        match self {
            types::TextAlignment::Left => "text-align: left;",
            types::TextAlignment::Center => "text-align: center;",
            types::TextAlignment::Right => "text-align: right;",
        }
        .to_string()
    }
}

//
// Stylesheet
//

impl ToCSS for Stylesheet {
    fn to_css(&self) -> String {
        let mut properties: Vec<String> = vec![];

        if let StyleOption::Some(t) = &self.padding {
            properties.push(format!("padding: {};", t.to_css()));
        }

        if let StyleOption::Some(t) = &self.margin {
            properties.push(format!("margin: {};", t.to_css()));
        }

        if let StyleOption::Some(t) = &self.layout {
            properties.push(t.to_css());
        }

        if let StyleOption::Some(t) = &self.transform {
            properties.push(t.to_css());
        }

        if let StyleOption::Some(t) = &self.font {
            properties.push(t.to_css());
        }

        if let StyleOption::Some(t) = &self.background {
            properties.push(t.to_css());
        }

        if let StyleOption::Some(t) = &self.border {
            properties.push(t.to_css());
        }

        if let StyleOption::Some(t) = &self.text_direction {
            properties.push(t.to_css());
        }

        properties.join(" ")
    }
}
