pub mod shape;
pub mod text;

use std::cell::RefCell;
use std::rc::Rc;

use leptos::{component, view, For, IntoView};
use toolbox_types::tree::{node_rc::NodeRc, ValidNode};

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
                view! {
                    <For
                        each=move || {nodes.clone()}
                        key=|node| {node.get_id()}
                        let:node
                    >
                        <Valid node=node.clone() />
                    </For>
                }
            }
            None => view! {}.into_view(),
        }
    })
}
