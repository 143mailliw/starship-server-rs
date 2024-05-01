use leptos::{component, view, IntoView};

#[component]
pub fn TextField() -> impl IntoView {
    view! {
        <input type="text"/>
    }
}
