use std::{cell::RefCell, rc::Rc};

use leptos::{
    component, create_memo, create_signal, expect_context, use_context, view, For, IntoView,
    ReadSignal, RwSignal, SignalGet, SignalSet,
};
use log::info;
use phosphor_leptos::{IconWeight, Minus, Plus};
use stylers::style;
use toolbox_types::{
    observers::Observable,
    project::Project,
    tree::{node_rc::NodeRc, page::Page, NodeBase, NodeFeature, RegularNode, ValidNode},
};
use web_sys::DragEvent;

use crate::{
    context::render::EditorContext, editor::nodes::nodeinfo::NodeInfoRef, hooks::node_signal,
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
        let page = pages.iter().find(|p| {
            let borrowed = p.borrow();
            let this_id = borrowed.id();
            this_id == page_id
        });

        if let Some(page) = page {
            let page_ref = page.borrow();

            let target = page_ref.find_node_by_path(rest.to_string());
            drop(page_ref);
            if let Some(target_node) = target {
                let previous_parent = target_node.parent();

                target_node.detach();
                match node {
                    TreeType::Node(node) => {
                        let mut node_ref = node.borrow_mut();
                        node_ref.add_child(target_node.clone(), index);
                        drop(node_ref);

                        node.commit_changes(NodeFeature::Children);
                    }
                    TreeType::Page(page) => {
                        let mut page_ref = page.borrow_mut();
                        page_ref.add_child(target_node.clone(), index);
                        drop(page_ref);

                        let unmut_ref = page.borrow();
                        unmut_ref.commit_changes(NodeFeature::Children);
                    }
                }

                if let Some(previous_parent) = previous_parent {
                    let upgraded = previous_parent.upgrade();
                    if let Some(previous_parent) = upgraded {
                        previous_parent.commit_changes(NodeFeature::Children);
                    }
                } else {
                    let page_ref = page.borrow();
                    page_ref.commit_changes(NodeFeature::Children);
                }

                target_node.commit_changes(NodeFeature::Position);
            }
        };
    }
}

#[component]
fn DropZone(
    node: TreeType,
    project_sig: RwSignal<Rc<RefCell<Project>>>,
    index: Option<usize>,
) -> impl IntoView {
    let class_name = style! {
        .dropzone {
            position: relative;
            width: 100%;
            height: 0.5rem;
            background-color: var(--light-dark-black);
            top: -0.25rem;
            left: 0;
            display: flex;
            z-index: 1;
            margin-bottom: -0.25rem;
        }
        .marker {
            height: 2px;
            width: 100%;
            background-color: var(--light-light-blue);
            margin: auto;
        }
    };

    let context = use_context::<EditorContext>().expect("there should be a context");
    let project_sig = context.project;

    view! { class = class_name,
        <div class="dropzone"
            on:dragover=move |e| {
                e.prevent_default();
                e.stop_propagation();
                info!("hello!")
            }
            on:drop=move |e| {
                e.prevent_default();
                e.stop_propagation();
                info!("goodbye!");
                move_node(e, node.clone(), project_sig, index);
            }
        >
            <div class="marker" />
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
        let node_ref = node.borrow();
        let children = node_ref.get_children();

        children.is_some() && !children.as_ref().unwrap().is_empty()
    });

    let context = use_context::<EditorContext>().expect("there should be a context");
    let project_sig = context.project;

    view! { class = class_name,
        <div class="container">
            <div
                class="item"
                on:click=move |_| {
                    if has_children.get() {
                        set_show.set(!show_children.get());
                    }
                }
                draggable="true"
                on:dragstart=move |e| {
                    e.stop_propagation();
                    //e.prevent_default();
                    let node = node_sig.get();
                    let path = node.get_path().expect("bad node, can't be dragged"); // FIXME: we should handle this
                    e.data_transfer().unwrap().set_data("text/plain", &path);
                }
                on:dragover=move |e| {
                    e.prevent_default();
                }
                on:drop=move |e| move_node(e, TreeType::Node(node_sig.get().clone()), project_sig, None)
            >
                <div class="icon">
                    {move || node_sig.get().get_icon("var(--light-dark-black)", "0.75rem").into_view()}
                </div>
                <div class="text">{move || node_sig.get().get_friendly_name()}</div>
                <div class="showicon">
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
                let children = node_sig.get().borrow().get_children().unwrap_or(vec![]);
                let c_clone = children.clone(); // FIXME: i don't like this
                let children_len = c_clone.len();
                trigger.track(); // i'm gonna be honest I have no clue why this is necessary

                if show_children.get() {
                    view! { class = children_class_name,
                        <div class="children">
                            <For
                                each=move || {
                                    trigger.track();
                                    0..children_len
                                }
                                key=move |index| {
                                    c_clone[*index].get_id()
                                }
                                let:index
                            >
                                <DropZone
                                    node={TreeType::Node(node_sig.get().clone())}
                                    project_sig={project_sig}
                                    index={Some(index)}
                                />
                                <TreeItem node={children[index].clone()} />
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
                    let page_ref = page.borrow();
                    page_ref.get_children().expect("page should have children")
                }
                key=move |node| node.get_id()
                let:value
            >
                <TreeItem node={value} />
            </For>
        </div>
    }
}
