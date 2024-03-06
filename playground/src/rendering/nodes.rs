pub mod shape;
pub mod text;

use std::cell::RefCell;
use std::rc::Rc;

use leptos::{component, view, IntoView};
use log::info;
use toolbox_types::tree::ValidNode;

#[component]
pub fn Valid(node: Rc<RefCell<ValidNode>>) -> impl IntoView {
    view!({
        move || {
            let value = node.borrow();
            match *(value) {
                ValidNode::ShapeNode(_) => {
                    drop(value);
                    view! {<shape::Shape node=node.clone() />}
                }
                ValidNode::TextNode(_) => {
                    drop(value);
                    view! {<text::Text node=node.clone() />}
                }
            }
        }
    })
}

#[component]
pub fn Children(nodes: Option<Vec<Rc<RefCell<ValidNode>>>>) -> impl IntoView {
    view!({
        move || match nodes.clone() {
            Some(nodes) => {
                let mut views = vec![];
                for node in nodes {
                    views.push(view! {<Valid node=node.clone() />});
                }
                views.into_view()
            }
            None => view! {}.into_view(),
        }
    })
}
