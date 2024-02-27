use std::rc::Rc;
use std::{cell::RefCell, fmt::format};

use log::info;
use toolbox_types::tree::{NodeBase, ValidNode};

use crate::styling::make::css;

pub trait Renderable {
    fn get_render_id(&self) -> String;
    fn get_css(&self) -> String;
}

impl Renderable for Rc<RefCell<ValidNode>> {
    fn get_render_id(&self) -> String {
        let node_borrowed = self.borrow();
        format!("elem-{}", node_borrowed.id())
    }

    fn get_css(&self) -> String {
        let node_borrowed = self.borrow();
        let styles = node_borrowed.get_styles();

        let css_res = css(&styles, format!("elem-{}", node_borrowed.id()).as_str());

        css_res
    }
}
