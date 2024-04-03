use enum_iterator::all;
use enum_iterator::Sequence;
use leptos::{component, view, For, IntoView, RwSignal, SignalGet, SignalSet};
use stylers::style;

pub trait Switchable: Sequence + Copy + Clone + PartialEq {
    fn name(&self) -> &str;
}

#[component]
pub fn Switcher<T>(context: RwSignal<T>) -> impl IntoView
where
    T: Switchable + 'static,
{
    let class_name = style! {
        .switcher {
            margin: 0.25rem;
            display: flex;
            gap: 0.15rem;
        }

        .current {
            background: var(--light-dark-white);
            color: var(--light-dark-blue);
        }

        button {
            padding: 0.15rem 0.35rem;
            font-size: 10pt;
            font-weight: 600;
            transition: all 0.2s;
            border-radius: var(--border-radius-small);
        }

        button:hover {
            background: var(--light-dark-white);
        }
    };

    view! { class = class_name,
        <div class="switcher">
            <For
                each=move || {all::<T>()}
                key=|value| value.name().to_string()
                let:value
            >
                <button
                    on:click=move |_| context.set(value)
                    class:current=move || context.get() == value
                >
                    {value.name().to_string()}
                </button>
            </For>
        </div>
    }
}
