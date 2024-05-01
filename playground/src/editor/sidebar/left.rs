use enum_iterator::Sequence;
use leptos::{component, create_rw_signal, view, IntoView, Show, SignalGet};

use crate::{
    components::switcher::{Switchable, Switcher},
    editor::sidebar::{blocks::Blocks, tree::Tree},
};

#[derive(Debug, PartialEq, Clone, Copy, Sequence)]
pub enum LeftSidebarContext {
    Pages,
    Blocks,
    Tree,
}

impl Switchable for LeftSidebarContext {
    fn name(&self) -> &str {
        match self {
            LeftSidebarContext::Pages => "Pages",
            LeftSidebarContext::Blocks => "Blocks",
            LeftSidebarContext::Tree => "Tree",
        }
    }
}

#[component]
pub fn Left() -> impl IntoView {
    let context = create_rw_signal(LeftSidebarContext::Blocks);

    view! {
        <div>
            <Switcher context=context/>
            <Show when=move || context.get() == LeftSidebarContext::Pages>
                <Blocks/>
            </Show>
            <Show when=move || context.get() == LeftSidebarContext::Blocks>
                <Blocks/>
            </Show>
            <Show when=move || context.get() == LeftSidebarContext::Tree>
                <Tree/>
            </Show>
        </div>
    }
}
