use leptos::{component, event_target_value, view, IntoView, RwSignal, SignalGet, SignalSet};

#[component]
pub fn TextField<T: ToString, F: Fn(String) + 'static>(
    value: T,
    input: F,
    description: Option<String>,
) -> impl IntoView {
    view! {
        <div>
            <span>{description}</span>
            <input type="text" value=value.to_string() on:input=move |ev| input(event_target_value(&ev)) />
        </div>
    }
}
