use std::{cell::RefCell, rc::Rc};

use leptos::{
    component, create_signal, expect_context, use_context, view, For, IntoView, SignalGet,
    SignalSet,
};
use phosphor_leptos::{IconWeight, Minus, Plus};
use stylers::style;
use toolbox_types::tree::{node_rc::NodeRc, NodeBase, NodeFeature, ValidNode};

use crate::{
    context::render::EditorContext, editor::nodes::nodeinfo::NodeInfoRef, hooks::node_signal,
};

#[component]
fn TreeItem(node: Rc<RefCell<ValidNode>>) -> impl IntoView {
    let (node_sig, _) = node_signal::create_node(node.clone(), vec![NodeFeature::Metadata]);
    let (show_children, set_show) = create_signal(true);

    let class_name = style! {
        .item {
            padding: 0.25rem 0.6rem;
            color: var(--light-dark-black);
            display: flex;
            cursor: pointer;
        }
        .item:hover {
            background-color: var(--light-dark-white);
        }
        .text {
            margin-right: auto;
            font-size: 10.5pt;
            font-weight: 600;
        }
        .font {
            margin-left: 0.5rem;
        }
        .icon {
            margin-right: 0.35rem;
            margin-top: -0.05rem;
        }
    };

    let children_class_name = style! {
        .children {
            margin-left: 1.15rem;
        }
    };

    let node = node_sig.get();
    let node_ref = node.borrow();
    let children = node_ref.get_children();
    let has_children = children.is_some() && !children.as_ref().unwrap().is_empty();

    view! { class = class_name,
        <div class="container">
            <div class="item" on:click=move |_| {
                if has_children {
                    set_show.set(!show_children.get());
                }
            }>
                <div class="icon">
                    {move || node_sig.get().get_icon("var(--light-dark-black)", "0.75rem").into_view()}
                </div>
                <div class="text">{move || node_sig.get().get_friendly_name()}</div>
                <div class="showicon">
                    {move || {
                        if has_children {
                            if show_children.get() {
                                view! {<Minus weight={IconWeight::Bold} size="0.75rem"/>}
                            } else {
                                view! {<Plus weight={IconWeight::Bold} size="0.75rem"/>}
                            }
                        } else {
                            ().into_view()
                        }
                    }}
                </div>
            </div>
            {move || {
                let m_children = children.clone();
                if show_children.get() {
                    if let Some(children) = m_children {
                        view! { class = children_class_name,
                            <div class="children">
                                <For
                                    each=move || children.clone()
                                    key=move |node| node.get_id()
                                    let:value
                                >
                                    <TreeItem node={value} />
                                </For>
                            </div>
                        }.into_view()
                    } else {
                        ().into_view()
                    }
                } else {
                    ().into_view()
                }
            }}
        </div>
    }
}

#[component]
pub fn Tree() -> impl IntoView {
    let class_name = style! {
        .tree {
        }
    };

    let context = use_context::<EditorContext>().expect("there should be a context");
    let page_sig = context.current_page;

    let page = page_sig.get();
    let page_ref = page.borrow();
    let children = page_ref.get_children().expect("page should have children");
    drop(page_ref);

    view! { class = class_name,
        <div class="tree">
            <For
                each=move || children.clone()
                key=move |node| node.get_id()
                let:value
            >
                <TreeItem node={value} />
            </For>
        </div>
    }
}
