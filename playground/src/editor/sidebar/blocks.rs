use std::{cell::RefCell, rc::Rc};

use leptos::{component, use_context, view, For, IntoView, SignalGet, View};
use stylers::style;
use toolbox_types::{
    observers::Observable,
    tree::{
        nodes::{ShapeNode, TextNode},
        CreatableNode, NodeBase, NodeFeature, ValidNode,
    },
};

use crate::{
    context::render::EditorContext,
    editor::{nodes::nodeinfo::NodeInfo, sidebar::components::Header},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockType {
    Shape = 0,
    Text = 1,
}

type BlockCategory = (&'static str, &'static [BlockType]);

const CATEGORIES: &[BlockCategory] = &[("Basic", &[BlockType::Shape, BlockType::Text])];

impl BlockType {
    fn name_from_type(&self) -> String {
        match self {
            BlockType::Shape => ShapeNode::get_friendly_name(),
            BlockType::Text => TextNode::get_friendly_name(),
        }
    }

    fn icon_from_type(&self, color: &str, size: &str) -> View {
        match self {
            BlockType::Shape => ShapeNode::get_icon(color, size).into_view(),
            BlockType::Text => TextNode::get_icon(color, size).into_view(),
        }
    }

    pub fn create(&self) -> Rc<RefCell<ValidNode>> {
        match self {
            BlockType::Shape => ShapeNode::create(),
            BlockType::Text => TextNode::create(),
        }
    }
}

#[component]
fn Block(block_type: &'static BlockType) -> impl IntoView {
    let context = use_context::<EditorContext>().expect("there should be a context");
    let page_sig = context.current_page;

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
        .icon {
            margin-right: 0.35rem;
            margin-top: -0.05rem;
        }
    };

    view! { class = class_name,
        <div
            class="item"
            on:click=move |_| {
                let node = block_type.create();
                let mut page = page_sig.get();

                page.add_child(node, None).expect("page should support children");
                page.commit_changes(NodeFeature::Children);
            }
        >
            <div class="icon">{move || block_type.icon_from_type("var(--light-dark-black)", "0.75rem")}</div>
            <div class="text">{move || block_type.name_from_type()}</div>
        </div>
    }
}

#[component]
fn Category(category: &'static BlockCategory) -> impl IntoView {
    let class_name = style! {
        .title {
            margin-top: 0.5rem;
            margin-left: 0.5rem;
        }
    };

    view! { class = class_name,
        <div>
            <div class="title"><Header>{move || category.0}</Header></div>
            <For
                each=move || category.1
                key=move |v| (*v)
                let:block_type
            >
                <Block block_type={block_type} />
            </For>
        </div>
    }
}

#[component]
pub fn Blocks() -> impl IntoView {
    view! {
        <div>
            <For
                each=move || CATEGORIES.iter()
                key=move |v| v.0
                let:category
            >
                <Category category={category} />
            </For>
        </div>
    }
}
