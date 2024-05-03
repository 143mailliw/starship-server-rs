use leptos::{component, view, Children, IntoView};
use stylers::style;

#[component]
pub fn Header(children: Children) -> impl IntoView {
    let class_name = style! {
        .header {
            font-size: 10pt;
            font-weight: 800;
            margin-bottom: 0.2rem;
            text-transform: uppercase;
            color: var(--light-light-black);
        }
    };

    view! { class = class_name,
        <div class="header">
            {children()}
        </div>
    }
}
