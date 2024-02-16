use std::cell::RefCell;
use std::rc::Rc;

use toolbox_types::tree::{NodeBase, ValidNode};

use crate::styling::make::css;

pub trait Renderable {
    fn get_styles(&self) -> String;
}

impl Renderable for Rc<RefCell<ValidNode>> {
    fn get_styles(&self) -> String {
        let mut node_borrowed = self.borrow_mut();
        let styles = node_borrowed.styles().clone();

        css(&styles, node_borrowed.id())
    }
}
