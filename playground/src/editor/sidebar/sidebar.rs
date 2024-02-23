use std::cell::RefCell;
use std::rc::Rc;

use leptos::{component, create_rw_signal, view, IntoView, ReadSignal, Show, SignalGet, SignalSet};
use toolbox_types::tree::page::Page;

use crate::editor::{
    context::{switcher::ContextSwitcher, SidebarContext},
    sidebar::tree::Tree,
};

#[component]
pub fn Sidebar() -> impl IntoView {
    let context = create_rw_signal(SidebarContext::Tree);

    view! {
        <div>
            <ContextSwitcher context=context/>
            <Show when=move || context.get() == SidebarContext::Pages>
                <div>
                    <h1>Page</h1>
                </div>
            </Show>
            <Show when=move || context.get() == SidebarContext::Blocks>
                <div>
                    <h1>Blocks</h1>
                </div>
            </Show>
            <Show when=move || context.get() == SidebarContext::Tree>
                <Tree/>
            </Show>
        </div>
    }
}
