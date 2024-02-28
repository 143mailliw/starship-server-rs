use std::cell::RefCell;
use std::rc::Rc;

use leptos::{create_node_ref, html, use_context, view, IntoView, NodeRef, SignalGet};
use log::info;
use stylers::style;
use toolbox_types::{
    events::Type,
    tree::{NodeBase, NodeFeature, ValidNode},
};

use crate::{
    context::render::EditorContext, hooks::node_signal::create_node,
    rendering::renderable::Renderable,
};

pub fn render(node: Rc<RefCell<ValidNode>>) -> impl IntoView {
    let (node_sig, trigger) = create_node(
        node.clone(),
        vec![NodeFeature::Properties, NodeFeature::Styles],
    );

    // we only use this to determine if we are in the editor
    let context = use_context::<EditorContext>().is_some();

    let class_name = style! {
        .text {
            display: inline-block;
            min-height: 1rem;
            min-width: 1rem;
        }
    };

    if context {
        let input_element: NodeRef<html::Span> = create_node_ref();

        view! {class = class_name,
            <span
                id={move || node_sig.get().get_render_id()}
                on:load=move |_| trigger.track()
                contenteditable
                class="text"
                node_ref=input_element
                on:input=move |e| {
                    if let Some(span) = input_element.get() {
                        let mut cell = node_sig.get();
                        let text = span.inner_text();
                        info!("setting text: {}", text);
                        cell.set_property("text", Type::String(text), false).expect("text property not found");
                    }
                }
            >
                <style>
                    {move || {
                        trigger.track();
                        let cell = node_sig.get();
                        cell.get_css()
                    }}
                </style>
                {move || {
                    trigger.track();
                    let node_raw = node_sig.get();

                    let text: String = node_raw
                        .get_property("text")
                        .expect("no text")
                        .try_into_string()
                        .unwrap();

                    text
                }}
            </span>
        }
    } else {
        view! {class = class_name,
            <span
                id={move || node_sig.get().get_render_id()}
                class="text"
            >
                <style>
                    {move || {
                        trigger.track();
                        let cell = node_sig.get();
                        cell.get_css()
                    }}
                </style>
                {move || {
                    trigger.track();
                    let node_raw = node_sig.get();

                    let text: String = node_raw
                        .get_property("text")
                        .expect("no text")
                        .try_into_string()
                        .unwrap();

                    text
                }}
            </span>
        }
    }
}
