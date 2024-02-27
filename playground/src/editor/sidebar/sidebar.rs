use leptos::{component, create_rw_signal, view, IntoView, Show, SignalGet};

use crate::editor::{
    context::{switcher::ContextSwitcher, SidebarContext},
    sidebar::{blocks::Blocks, tree::Tree},
};

#[component]
pub fn Sidebar() -> impl IntoView {
    let context = create_rw_signal(SidebarContext::Tree);

    view! {
        <div>
            <ContextSwitcher context=context/>
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
