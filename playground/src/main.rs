pub mod hooks;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

use leptos::*;
use toolbox_types::project;
use toolbox_types::tree::nodes;
use toolbox_types::tree::{page, CreatableNode, NodeBase, NodeFeature, ValidNode};

use crate::hooks::node_signal::create_node_signal;

fn main() {
    let project = project::Project::create("test".to_string(), project::Type::Component);
    let page = page::Page::create("test page".to_string(), Rc::downgrade(&project));
    let node = nodes::TextNode::create();

    let mut page_ref = page.borrow_mut();
    page_ref.add_child(node.clone(), None);

    mount_to_body(|| view! { <App node={node}/> })
}

#[component]
fn App(node: Rc<RefCell<ValidNode>>) -> impl IntoView {
    let node_sig = create_node_signal(node, vec![NodeFeature::Properties]);

    view! {
        <div>
            {move || {
                let node_raw = node_sig.get();
                let node_ref = node_raw.borrow();

                let text: String = node_ref.get_property("text").try_into().unwrap();
                text
            }}
            <button
                on:click=move |_| {
                    // doesnt do anything
                }
            >
                "Set"
            </button>
        </div>
    }
}
