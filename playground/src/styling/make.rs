use super::to_css::ToCSS;
use toolbox_types::styles::stylesheet::{StyleLayers, StyleOption};

pub fn css(layers: StyleLayers, prefix: &str) -> String {
    let mut styles: Vec<String> = vec![];
    styles.push(format!("#{prefix} {{ {} }} ", layers.base.to_css()));

    if let StyleOption::Some(sheet) = layers.hover {
        styles.push(format!("#{prefix}:hover {{ {} }} ", sheet.to_css()));
    }

    if let StyleOption::Some(sheet) = layers.active {
        styles.push(format!("#{prefix}:active {{ {} }} ", sheet.to_css()));
    }

    if let StyleOption::Some(sheet) = layers.focused {
        styles.push(format!("#{prefix}:focus {{ {} }} ", sheet.to_css()));
    }

    if let StyleOption::Some(sheet) = layers.checked {
        styles.push(format!("#{prefix}:checked {{ {} }} ", sheet.to_css()));
    }

    styles.join("\n")
}
