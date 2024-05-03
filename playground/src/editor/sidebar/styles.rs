use leptos::{component, use_context, view, Children, CollectView, IntoView, SignalGet, View};
use stylers::style;
use toolbox_types::{styles::stylesheet::StyleOption, tree::NodeBase};

use crate::{
    context::render::EditorContext,
    editor::selection::{self, Selection},
};

use super::components::Header;

#[component]
fn StyleSegment<T>(
    option: StyleOption<T>,
    title: &'static str,
    children: Children,
) -> impl IntoView {
    let class_name = style! {
      .segment {
          padding: 0.5rem;
          border-bottom: 1px solid var(--light-dark-white);
      }
    };

    match option {
        StyleOption::Default | StyleOption::Some(_) => view! { class = class_name,
            <div class="segment">
                <Header>{title}</Header>
                {children().nodes.iter().collect_view()}
            </div>
        }
        .into_view(),
        _ => ().into_view(),
    }
}

#[component]
pub fn StyleEditor() -> impl IntoView {
    let context = use_context::<EditorContext>().expect("there should be a context");
    let selection_sig = context.selection;

    let notice_name = style! {
        .notice {
            color: var(--light-light-black);
            font-size: 11pt;
            text-align: center;
            margin-top: 0.75rem;
        }
    };

    let container_name = style! {
        .container {

        }
    };

    view! { class = container_name,
        <div>
            {move || match selection_sig.get() {
                Selection::None => view! { class = notice_name, <div class="notice">"Select a component to edit it's styles."</div> }.into_view(),
                Selection::Single(node) => {
                    let style_sheet = node.get_styles().base;

                    view! {
                        <div>
                            <StyleSegment option={style_sheet.transform} title={"Transform"}>
                                content
                            </StyleSegment>
                            <StyleSegment option={style_sheet.layout} title={"Layout"}>
                                content
                            </StyleSegment>
                            <StyleSegment option={style_sheet.font} title={"Text"}>
                                content
                            </StyleSegment>
                            <StyleSegment option={style_sheet.background} title={"Background"}>
                                content
                            </StyleSegment>
                            <StyleSegment option={style_sheet.border} title={"Border"}>
                                content
                            </StyleSegment>
                        </div>
                    }
                    .into_view()
                },
                Selection::Multiple(_) => view! { class = notice_name, <div class="notice">Select one component</div>}.into_view(),
            }}
        </div>
    }
}
