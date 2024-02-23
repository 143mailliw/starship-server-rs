use std::cell::RefCell;
use std::rc::Rc;

use leptos::{view, IntoView};
use phosphor_leptos::{IconWeight, Square, TextT};
use toolbox_types::tree::{
    nodes::{ShapeNode, TextNode},
    ValidNode,
};

pub trait NodeInfo {
    fn get_friendly_name() -> String;
    fn get_icon(color: &str, size: &str) -> impl IntoView;
}

pub trait NodeInfoRef {
    fn get_friendly_name(&self) -> String;
    fn get_icon(&self, color: &str, size: &str) -> impl IntoView;
}

impl NodeInfo for TextNode {
    fn get_friendly_name() -> String {
        "Text".to_string()
    }

    fn get_icon(color: &str, size: &str) -> impl IntoView {
        view! {
            <TextT color=color weight=IconWeight::Bold size=size/>
        }
    }
}

impl NodeInfo for ShapeNode {
    fn get_friendly_name() -> String {
        "Shape".to_string()
    }

    fn get_icon(color: &str, size: &str) -> impl IntoView {
        view! {
            <Square color=color weight=IconWeight::Bold size=size/>
        }
    }
}

impl NodeInfoRef for ValidNode {
    fn get_friendly_name(&self) -> String {
        match self {
            ValidNode::ShapeNode(_) => ShapeNode::get_friendly_name(),
            ValidNode::TextNode(_) => TextNode::get_friendly_name(),
        }
    }

    fn get_icon(&self, color: &str, size: &str) -> impl IntoView {
        match self {
            ValidNode::ShapeNode(_) => ShapeNode::get_icon(color, size).into_view(),
            ValidNode::TextNode(_) => TextNode::get_icon(color, size).into_view(),
        }
    }
}

impl NodeInfoRef for Rc<RefCell<ValidNode>> {
    fn get_friendly_name(&self) -> String {
        let borrowed = self.borrow();
        borrowed.get_friendly_name()
    }

    fn get_icon(&self, color: &str, size: &str) -> impl IntoView {
        let borrowed = self.borrow();
        borrowed.get_icon(color, size).into_view()
    }
}
