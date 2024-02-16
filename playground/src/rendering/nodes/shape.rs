use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    hooks::node_signal::create_node,
    rendering::{nodes::render_children, renderable::Renderable},
};
use leptos::{view, IntoView, SignalGet};
use toolbox_types::tree::{NodeBase, NodeFeature, ValidNode};

pub fn render(node: Rc<RefCell<ValidNode>>) -> impl IntoView {
    let (node_sig, trigger) = create_node(
        node,
        vec![
            NodeFeature::Properties,
            NodeFeature::Styles,
            NodeFeature::Children,
        ],
    );

    view!(
        <div class={move || node_sig.get().id().clone()} on:load={move |_| trigger.track()}>
            // HACK: Style in body is invalid HTML but efficient
            // TODO: Move to head
            <style>
                {move || node_sig.get().get_styles()}
            </style>
            {move || {
                render_children(node_sig.get().get_children())
            }}
        </div>
    )
}
