use std::{cell::RefCell, rc::Rc};

use leptos::{component, view, IntoView, SignalGet};
use log::{debug, info};
use toolbox_types::tree::{NodeFeature, ValidNode};

use crate::{components::input::TextField, hooks::node_signal::create_node};

#[component]
pub(super) fn TransformEditor(node: Rc<RefCell<ValidNode>>) -> impl IntoView {
    let (node_sig, trigger) = create_node(node, vec![NodeFeature::Metadata, NodeFeature::Styles]);

    view! {
        <div>
            {move || {
                trigger.track();
                let node = node_sig.get();

                view! {
                    <TextField description={Some("X".into())} value="bruh" input=move |t| {
                        info!("!!!");
                    }/>
                    <TextField description={Some("Y".into())} value="bruh" input=move |t| {
                        info!("!!!");
                    }/>
                    <TextField description={Some("W".into())} value="bruh" input=move |t| {
                        info!("!!!");
                    }/>
                    <TextField description={Some("H".into())} value="bruh" input=move |t| {
                        info!("!!!");
                    }/>
                }
            }}
        </div>
    }
}
