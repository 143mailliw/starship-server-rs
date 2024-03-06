use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    hooks::node_signal::create_node,
    rendering::{nodes::Children, renderable::Renderable},
};
use leptos::{component, view, IntoView, SignalGet};
use log::info;
use toolbox_types::tree::{NodeBase, NodeFeature, ValidNode};

#[component]
pub fn Shape(node: Rc<RefCell<ValidNode>>) -> impl IntoView {
    let (node_sig, trigger) = create_node(
        node,
        vec![
            NodeFeature::Properties,
            NodeFeature::Styles,
            NodeFeature::Children,
        ],
    );

    view!(
        <div id={move || node_sig.get().get_render_id()}>
            // HACK: Style in body is invalid HTML but efficient
            // TODO: Move to head
            <style>
                {move || {
                    trigger.track();
                    node_sig.get().get_css()
                }}
            </style>
            {move || {
                info!("rendering shape children");
                trigger.track();
                let children = node_sig.get().get_children();
                view! {<Children nodes={children}/>}
            }}
        </div>
    )
}
