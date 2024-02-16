pub mod hooks;
pub mod rendering;
pub mod styling;

use log::Level;
use std::cell::RefCell;
use std::rc::Rc;

use leptos::{component, mount_to_body, view, IntoView, SignalGet};
use toolbox_types::events::Type;
use toolbox_types::project;
use toolbox_types::tree::nodes;
use toolbox_types::tree::page::Page;
use toolbox_types::tree::{CreatableNode, NodeBase, NodeFeature};

use crate::rendering::page::{create_page, render};

fn main() {
    console_log::init_with_level(Level::Info);
    console_error_panic_hook::set_once();

    let project = project::Project::create("test".to_string(), project::Type::Component);
    let page = Page::create("test page".to_string(), Rc::downgrade(&project));
    let mut node = nodes::TextNode::create();
    node.set_property("text", Type::String("bruh".to_string()), false);

    let mut page_ref = page.borrow_mut();
    page_ref.add_child(node.clone(), None);

    drop(page_ref);

    mount_to_body(|| view! { <App page={page}/> })
}

#[component]
fn App(page: Rc<RefCell<Page>>) -> impl IntoView {
    let (page_sig, trigger) = create_page(
        page.clone(),
        vec![NodeFeature::Properties, NodeFeature::Children],
    );

    view! {
        <div on:load=move |_| trigger.track()>
            {move || render(page_sig.get().clone())}
        </div>
    }
}
