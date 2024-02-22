use std::cell::RefCell;
use std::rc::Rc;

use leptos::{component, create_rw_signal, view, IntoView, ReadSignal, SignalGet, SignalSet};
use toolbox_types::tree::page::Page;

use crate::editor::context::{switcher::ContextSwitcher, SidebarContext};

#[component]
pub fn Sidebar(page: ReadSignal<Rc<RefCell<Page>>>) -> impl IntoView {
    let context = create_rw_signal(SidebarContext::Blocks);

    view! {
        <div>
            <ContextSwitcher context=context/>
        </div>
    }
}
