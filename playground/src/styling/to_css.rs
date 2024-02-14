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

        properties.join(" ")
    }
}
