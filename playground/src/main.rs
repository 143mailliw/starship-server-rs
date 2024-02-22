pub mod hooks;
pub mod rendering;
pub mod styling;

use log::Level;
use std::cell::RefCell;
use std::rc::Rc;
use toolbox_types::styles::stylesheet::StyleOption;
use toolbox_types::styles::types::{
    Border, BorderSide, BorderStyle, Color, Corners, Locked, Scale, ThemedColor,
};

use leptos::{component, mount_to_body, view, IntoView, SignalGet};
use toolbox_types::events::Type;
use toolbox_types::project;
use toolbox_types::tree::nodes;
use toolbox_types::tree::page::Page;
use toolbox_types::tree::{CreatableNode, NodeBase, NodeFeature};

use crate::rendering::page::{create_page, render};

fn main() {
    console_log::init_with_level(Level::Info);
    console_error_panic_hook::set_once();

    let project = project::Project::create("test".to_string(), project::Type::Component);
    let page = Page::create("Test Page".to_string(), Rc::downgrade(&project));

    let shape = nodes::ShapeNode::create();

    let mut text = nodes::TextNode::create();
    match text.set_property(
        "text",
        Type::String("Hello from playground and toolbox_types".to_string()),
        false,
    ) {
        Ok(_) => {}
        Err(e) => {
            log::error!("Error setting property: {:?}", e);
        }
    }

    let mut shape_ref = shape.borrow_mut();
    match shape_ref.add_child(text.clone(), None) {
        Ok(_) => {}
        Err(e) => {
            log::error!("Error adding child: {:?}", e);
        }
    }

    let styles = shape_ref.styles();
    styles.base.border = StyleOption::Some(Border {
        left: None,
        right: None,
        top: Some(BorderSide {
            color: Color::Themed {
                color: ThemedColor::DarkBlack,
                alpha: 1.0,
            },
            size: Scale::Pixels(2.0),
            style: BorderStyle::Straight,
        }),
        bottom: None,
        corners: Corners {
            top_left: Scale::Pixels(10.0),
            top_right: None,
            bottom_left: None,
            bottom_right: None,
            locked: true,
        },
        locked: Locked::All,
    });

    drop(styles);
    drop(shape_ref);

    let mut page_ref = page.borrow_mut();
    page_ref.add_child(shape.clone(), None);

    drop(page_ref);

    mount_to_body(|| view! { <App page={page}/> })
}

#[component]
fn App(page: Rc<RefCell<Page>>) -> impl IntoView {
    let (page_sig, trigger) = create_page(
        page.clone(),
        vec![NodeFeature::Properties, NodeFeature::Children],
    );

    let class_name = stylers::style! {
        #main-container {
            display: flex;
            flex-direction: column;
            height: 100vh;
            width: 100vw;
        }

        #editor {
            display: flex;
            height: 100%;
            width: 100%;
        }

        #toolbar {
            height: 3rem;
            background-color: var(--light-white);
            border-bottom: 1px solid var(--light-dark-white);
            flex-shrink: 0;
        }

        #sidebar {
            width: 15rem;
            background-color: var(--light-white);
            border-right: 1px solid var(--light-dark-white);
            flex-shrink: 0;
            min-height: 100%;
        }

        #page {
            flex-shrink: 1;
            width: 100%;
            height: 100%;
            background-color: var(--light-light-white);
            overflow: auto;
        }
    };

    view! { class = class_name,
        <div id="main-container">
            <div id="toolbar">toolbar</div>
            <div id="editor">
                <div id="sidebar">
                    sidebar
                </div>
                <div id="page" on:load=move |_| trigger.track()>
                    {move || render(page_sig.get().clone())}
                </div>
            </div>
        </div>
    }
}
