pub mod shape;
pub mod text;

use std::cell::RefCell;
use std::rc::Rc;

use leptos::{view, IntoView};
use toolbox_types::tree::ValidNode;

pub fn render_valid(node: Rc<RefCell<ValidNode>>) -> impl IntoView {
    view!({
        move || match *(node.borrow()) {
            ValidNode::ShapeNode(_) => shape::render(node.clone()).into_view(),
            ValidNode::TextNode(_) => text::render(node.clone()).into_view(),
        }
    })
}

pub fn render_children(nodes: Option<Vec<Rc<RefCell<ValidNode>>>>) -> impl IntoView {
    view!({
        move || match nodes.clone() {
            Some(nodes) => {
                let mut views = vec![];
                for node in nodes {
                    views.push(render_valid(node));
                }
                views.into_view()
            }
            None => view! {}.into_view(),
        }
    })
}
