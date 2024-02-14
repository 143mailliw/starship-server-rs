pub mod hooks;
pub mod styling;

use log::Level;
use std::cell::RefCell;
use std::rc::Rc;

use leptos::{component, event_target_value, mount_to_body, view, IntoView, SignalGet};
use toolbox_types::events::Type;
use toolbox_types::project;
use toolbox_types::tree::nodes;
use toolbox_types::tree::{page, CreatableNode, NodeBase, NodeFeature, ValidNode};

use crate::hooks::node_signal::create_node;

fn main() {
    console_log::init_with_level(Level::Info);

    let project = project::Project::create("test".to_string(), project::Type::Component);
    let page = page::Page::create("test page".to_string(), Rc::downgrade(&project));
    let node = nodes::TextNode::create();

    let mut page_ref = page.borrow_mut();
    page_ref.add_child(node.clone(), None);

    mount_to_body(|| view! { <App node={node}/> })
}

#[component]
fn App(node: Rc<RefCell<ValidNode>>) -> impl IntoView {
    let (node_sig, trigger) = create_node(node.clone(), vec![NodeFeature::Properties]);

    view! {
        <div>
            {move || {
                trigger.track();
                let node_raw = node_sig.get();

                let text: String = node_raw.get_property("text").expect("no text").try_into_string().unwrap();
                format!("text: {text} ")
            }}

            <input
                type="text"
                on:input=move |ev| {
                    let mut node_raw = node_sig.get();
                    node_raw.set_property("text", Type::String(event_target_value(&ev)), true);
                }
            />
        </div>
    }
}
