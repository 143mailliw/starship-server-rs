use enum_iterator::Sequence;
use leptos::{component, create_rw_signal, view, IntoView, Show, SignalGet};

use crate::{
    components::switcher::{Switchable, Switcher},
    editor::sidebar::{blocks::Blocks, tree::Tree},
};

#[derive(Debug, PartialEq, Clone, Copy, Sequence)]
pub enum SidebarContext {
    Pages,
    Blocks,
    Tree,
}

impl Switchable for SidebarContext {
    fn name(&self) -> &str {
        match self {
            SidebarContext::Pages => "Pages",
            SidebarContext::Blocks => "Blocks",
            SidebarContext::Tree => "Tree",
        }
    }
}

#[component]
pub fn Left() -> impl IntoView {
    let context = create_rw_signal(SidebarContext::Blocks);

    view! {
        <div>
            <Switcher context=context/>
            <Show when=move || context.get() == SidebarContext::Pages>
                <Blocks/>
            </Show>
            <Show when=move || context.get() == SidebarContext::Blocks>
                <Blocks/>
            </Show>
            <Show when=move || context.get() == SidebarContext::Tree>
                <Tree/>
            </Show>
        </div>
    }
}
