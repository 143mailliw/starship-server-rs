use enum_iterator::Sequence;
use leptos::{component, create_rw_signal, view, IntoView, Show, SignalGet};

use crate::{
    components::switcher::{Switchable, Switcher},
    editor::sidebar::styles::StyleEditor,
};

#[derive(Debug, PartialEq, Clone, Copy, Sequence)]
pub enum RightSidebarContext {
    Styles,
}

impl Switchable for RightSidebarContext {
    fn name(&self) -> &str {
        match self {
            RightSidebarContext::Styles => "Styles",
        }
    }
}

#[component]
pub fn Right() -> impl IntoView {
    let context = create_rw_signal(RightSidebarContext::Styles);

    view! {
        <div>
            <Switcher context=context/>
            <Show when=move || context.get() == RightSidebarContext::Styles>
                <StyleEditor/>
            </Show>
        </div>
    }
}
