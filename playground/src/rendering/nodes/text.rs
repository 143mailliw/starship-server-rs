use std::cell::RefCell;
use std::rc::Rc;

use leptos::{view, IntoView, SignalGet};
use toolbox_types::tree::{NodeBase, NodeFeature, ValidNode};

use crate::{hooks::node_signal::create_node, rendering::renderable::Renderable};

pub fn render(node: Rc<RefCell<ValidNode>>) -> impl IntoView {
    let (node_sig, trigger) = create_node(
        node.clone(),
        vec![NodeFeature::Properties, NodeFeature::Styles],
    );

    view!(
        <span class={move || node_sig.get().id().clone()} on:load=move |_| trigger.track()>
            <style>
                {move || node_sig.get().get_styles()}
            </style>
            {move || {
                let node_raw = node_sig.get();

                let text: String = node_raw
                    .get_property("text")
                    .expect("no text")
                    .try_into_string()
                    .unwrap();
                text
            }}
        </span>
    )
}
