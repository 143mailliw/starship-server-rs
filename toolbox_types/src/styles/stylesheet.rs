use super::types::{Border, Font, Graphic, Layout, Margin, TextAlignment, Transform};

#[derive(Clone, Debug, PartialEq)]
pub struct Stylesheet {
    // layout
    pub margin: StyleOption<Margin>,
    pub padding: StyleOption<Margin>,
    pub layout: StyleOption<Layout>,
    pub transform: StyleOption<Transform>,

    // display
    pub font: StyleOption<Font>,
    pub background: StyleOption<Graphic>,
    pub border: StyleOption<Border>,
    pub text_direction: StyleOption<TextAlignment>,
}

impl Stylesheet {
    pub fn new() -> Self {
        Stylesheet {
            margin: StyleOption::Default,
            padding: StyleOption::Default,
            layout: StyleOption::Default,
            transform: StyleOption::Default,
            font: StyleOption::Default,
            background: StyleOption::Default,
            border: StyleOption::Default,
            text_direction: StyleOption::Default,
        }
    }

    pub fn merge(self, default: Stylesheet) -> Stylesheet {
        Stylesheet {
            margin: self.margin.or(default.margin),
            padding: self.padding.or(default.padding),
            layout: self.layout.or(default.layout),
            transform: self.transform.or(default.transform),
            font: self.font.or(default.font),
            background: self.background.or(default.background),
            border: self.border.or(default.border),
            text_direction: self.text_direction.or(default.text_direction),
        }
    }
}

impl Default for Stylesheet {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StyleLayers {
    pub base: Stylesheet,
    pub hover: StyleOption<Stylesheet>,
    pub active: StyleOption<Stylesheet>,
    pub focused: StyleOption<Stylesheet>,
    pub checked: StyleOption<Stylesheet>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StyleOption<T> {
    Some(T),
    Default,
    Unsupported,
}

impl<T> StyleOption<T> {
    pub fn unwrap(self) -> T {
        match self {
            StyleOption::Some(value) => value,
            StyleOption::Default => panic!("tried to unwrap a default value"),
            StyleOption::Unsupported => unimplemented!(),
        }
    }

    pub fn is_some(&self) -> bool {
        match self {
            StyleOption::Some(_) => true,
            StyleOption::Default => false,
            StyleOption::Unsupported => false,
        }
    }

    pub fn or(self, other: Self) -> Self {
        match self {
            StyleOption::Some(_) => self,
            StyleOption::Default => other,
            StyleOption::Unsupported => StyleOption::Unsupported,
        }
    }
}
