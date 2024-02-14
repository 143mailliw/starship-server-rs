use super::types::{Border, CardinalDirection, Font, Graphic, Layout, Margin, Transform};

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
    pub text_direction: StyleOption<CardinalDirection>,
}

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
}
