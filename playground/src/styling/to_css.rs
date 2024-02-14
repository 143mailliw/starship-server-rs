use toolbox_types::styles::{color::light_color_from_themed, types};

trait ToCSS {
    /// Transform this object into a CSS string that can be used for HTML rendering.
    fn to_css(&self) -> String;
}

impl ToCSS for types::Color {
    fn to_css(&self) -> String {
        match self {
            types::Color::Hsla { h, s, v, a } => format!("hsl({h}deg {s}% {v}% / {a}"),
            types::Color::Rgba { r, g, b, a } => format!("rgb({r}, {g}, {b} / a)"),
            types::Color::Themed { color, alpha } => {
                light_color_from_themed(types::Color::Themed {
                    color: *color,
                    alpha: *alpha,
                })
                .to_css()
            }
        }
    }
}
