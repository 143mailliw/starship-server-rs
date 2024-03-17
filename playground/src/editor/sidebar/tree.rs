use std::{cell::RefCell, rc::Rc};

use leptos::{
    component, create_memo, create_signal, use_context, view, For, IntoView, RwSignal, SignalGet,
    SignalSet,
};
use phosphor_leptos::{IconWeight, Minus, Plus};
use stylers::style;
use toolbox_types::{
    project::Project,
    tree::{node_rc::NodeRc, page::Page, NodeBase, NodeFeature, RegularNode, ValidNode},
};
use web_sys::DragEvent;

use crate::{
    context::render::{DragState, EditorContext},
    editor::{nodes::nodeinfo::NodeInfoRef, selection::Selection},
    hooks::node_signal,
    rendering::page::create_page,
};

#[derive(Clone)]
enum TreeType {
    Page(Rc<RefCell<Page>>),
    Node(Rc<RefCell<ValidNode>>),
}

fn move_node(
    e: DragEvent,
    node: TreeType,
    project_sig: RwSignal<Rc<RefCell<Project>>>,
    index: Option<usize>,
) {
    e.stop_propagation();
    e.prevent_default();
    let data = e.data_transfer().unwrap().get_data("text/plain");

    if let Ok(data) = data {
        let split = data.split(':').collect::<Vec<_>>();
        let page_id = split.first().unwrap();
        let rest = split.last().unwrap();

        if page_id == rest {
            return;
        }

        let project = project_sig.get();
        let project_ref = project.borrow();

        if let TreeType::Node(node) = node.clone() {
            if !node.features().contains(&NodeFeature::Children) {
                return;
            }
        }

        let pages = project_ref.pages().unwrap();
        let page = pages.iter().find(|p| &p.get_id() == page_id);

        if let Some(page) = page {
            let target = page.borrow().find_node_by_path(rest.to_string());
            if let Some(target_node) = target {
                // ignore the error for now
                // TODO: if the error is not something we expect (like ChildrenUnsupported), we should handle it
                let _ = match node {
                    TreeType::Node(mut node) => node.move_into(target_node, index),
                    TreeType::Page(mut page) => page.move_into(target_node, index),
                };
            }
        };
    }
}

#[component]
fn DropZone(node: TreeType, index: Option<usize>) -> impl IntoView {
    let (showing, set_showing) = create_signal(false);

    let class_name = style! {
        .dropzone {
            display: none;
            position: relative;
            width: 100%;
            height: 0.5rem;
            z-index: 1;
            margin-top: -0.25rem;
            margin-bottom: -0.25rem;
        }
        .marker {
            display: none;
            height: 2px;
            width: 100%;
            background-color: var(--light-light-blue);
            margin: auto;
        }
        .visible {
            display: flex;
        }
    };

    let context = use_context::<EditorContext>().expect("there should be a context");
    let project_sig = context.project;
    let drag_sig = context.dragging;

    view! { class = class_name,
        <div class="dropzone" class:visible=move || {drag_sig.get() == DragState::TreeNode}
            on:dragover=move |e| {
                e.prevent_default();
                e.stop_propagation();
                set_showing.set(true);
            }
            on:dragleave=move |e| {
                e.prevent_default();
                e.stop_propagation();
                set_showing.set(false);
            }
            on:drop=move |e| {
                e.prevent_default();
                e.stop_propagation();
                drag_sig.set(DragState::None);
                move_node(e, node.clone(), project_sig, index);
            }
        >
            <div class="marker" class:visible=move || showing.get()/>
        </div>
    }
}

#[component]
fn TreeItem(node: Rc<RefCell<ValidNode>>) -> impl IntoView {
    let (node_sig, trigger) = node_signal::create_node(
        node.clone(),
        vec![NodeFeature::Metadata, NodeFeature::Children],
    );
    let (show_children, set_show) = create_signal(true);

    let class_name = style! {
        .item {
            padding: 0.25rem 0.6rem;
            color: var(--light-dark-black);
            display: flex;
            cursor: pointer;
            position: relative;
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
        .selected {
            background-color: var(--light-selection);
        }
        .selected:hover {
            background-color: var(--light-selection-hover);
        }
    };

    let children_class_name = style! {
        .children {
            margin-left: 1.15rem;
            position: relative;
        }
    };

    let has_children = create_memo(move |_| {
        trigger.track();
        let node = node_sig.get();
        let children = node.get_children();

        children.is_some() && !children.as_ref().unwrap().is_empty()
    });

    let context = use_context::<EditorContext>().expect("there should be a context");
    let project_sig = context.project;
    let drag_sig = context.dragging;
    let selection_sig = context.selection;

    view! { class = class_name,
        <div class="container">
            <div
                class="item"
                class:selected=move || selection_sig.get().has(&node_sig.get())
                on:click=move |e| {
                    if e.ctrl_key() || e.meta_key() {
                        let mut selection = selection_sig.get();
                        selection.toggle(node_sig.get());
                        selection_sig.set(selection);
                    } else {
                        selection_sig.set(Selection::single(node_sig.get()));
                    }
                }
                draggable="true"
                on:dragstart=move |e| {
                    e.stop_propagation();
                    //e.prevent_default();
                    let node = node_sig.get();
                    let path = node.get_path().expect("bad node, can't be dragged"); // FIXME: we should handle this
                    e.data_transfer().unwrap().set_data("text/plain", &path);

                    drag_sig.set(DragState::TreeNode);
                }
                on:dragend=move |_| {
                    drag_sig.set(DragState::None);
                }
                on:dragover=move |e| {
                    e.prevent_default();
                }
                on:drop=move |e| {
                    drag_sig.set(DragState::None);
                    move_node(e, TreeType::Node(node_sig.get().clone()), project_sig, None)
                }
            >
                <div class="icon">
                    {move || node_sig.get().get_icon("var(--light-dark-black)", "0.75rem").into_view()}
                </div>
                <div class="text">{move || node_sig.get().get_friendly_name()}</div>
                <div class="showicon"
                    on:click=move |e| {
                        e.stop_propagation();

                        if has_children.get() {
                            set_show.set(!show_children.get());
                        }
                    }
                >
                    {move || {
                        if has_children.get() {
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
                trigger.track(); // i'm gonna be honest I have no clue why this is necessary

                if show_children.get() {
                    view! { class = children_class_name,
                        <div class="children">
                            <For
                                each=move || {
                                    trigger.track();
                                    let children = node_sig.get().get_children().unwrap_or(vec![]);
                                    children.iter().cloned().enumerate().collect::<Vec<_>>()
                                }
                                key=move |node| node.1.get_id()
                                let:value
                            >
                                <DropZone
                                    node={TreeType::Node(node_sig.get().clone())}
                                    index={Some(value.0)}
                                />
                                <TreeItem node={value.1} />
                            </For>
                        </div>
                    }.into_view()
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
    let page_sig_global = context.current_page;
    let (page_sig, trigger) = create_page(page_sig_global.get(), vec![NodeFeature::Children]);

    view! { class = class_name,
        <div class="tree">
            <For
                each=move || {
                    trigger.track();

                    let page = page_sig.get();
                    let children = page.get_children().expect("page should have children");
                    children.iter().cloned().enumerate().collect::<Vec<_>>()
                }
                key=move |node| node.1.get_id()
                let:value
            >
                <DropZone
                    node={TreeType::Page(page_sig.get().clone())}
                    index={Some(value.0)}
                />
                <TreeItem node={value.1.clone()} />
            </For>
            <DropZone
                node={TreeType::Page(page_sig.get().clone())}
                index={None}
            />
        </div>
    }
}
