use std::cell::RefCell;
use std::rc::Rc;

use leptos::{
    create_effect, create_signal, create_trigger, view, IntoView, ReadSignal, SignalGet, SignalSet,
    Trigger,
};
use log::{error, info};
use stylers::style;
use toolbox_types::observers::Observable;
use toolbox_types::tree::page::{Page, Title};
use toolbox_types::tree::{NodeBase, NodeFeature};

use crate::rendering::nodes::render_children;

pub fn create_page(
    page: Rc<RefCell<Page>>,
    features: Vec<NodeFeature>,
) -> (ReadSignal<Rc<RefCell<Page>>>, Trigger) {
    let trigger = create_trigger();
    let (page_sig, _set_page) = create_signal(page);
    // we need to keep our observer closure in scope for the lifetime of this signal
    let (_rc, set_rc) = create_signal::<Option<Rc<RefCell<dyn FnMut()>>>>(None);
    let (setup, set_setup) = create_signal(false);

    let closure = move || {
        let x = trigger.try_notify();
        if !x {
            error!("no reactive runtime found while calling observer");
        }
    };

    let rc: Rc<RefCell<dyn FnMut()>> = Rc::new(RefCell::new(closure));
    set_rc.set(Some(rc.clone()));

    create_effect(move |_| {
        let cell = page_sig.get();
        let mut page = cell.borrow_mut();

        if !setup.get() {
            for feature in features.clone() {
                info!("{:#?}", feature);

                page.register(feature, &rc);
            }
            set_setup.set(true);
        }
    });

    (page_sig, trigger)
}

pub fn render(page: Rc<RefCell<Page>>) -> impl IntoView {
    let (page_sig, trigger) =
        create_page(page, vec![NodeFeature::Properties, NodeFeature::Children]);

    let class_name = style! {
        #page {
            width: 65vw;
            margin-left: auto;
            margin-right: auto;
            background-color: var(--light-light-white);

        }
        #page-title {
            font-size: 32pt;
            font-weight: 800;
            margin-top: 4rem;
            margin-bottom: 1rem;
            color: var(--light-dark-black);
        }
    };

    view! { class = class_name,
        <div id="page" on:load=move |_| trigger.track()>
            <h1 id="page-title">{move || {
                match &page_sig.get().borrow().title {
                    Title::Basic { content } => content.clone(),
                }
            }}</h1>
            <div>
                {move || {render_children(page_sig.get().borrow().get_children())}}
            </div>
        </div>
    }
}
