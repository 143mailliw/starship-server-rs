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

pub enum StyleOption<T> {
    Some(T),
    Default,
    Unsupported,
}
