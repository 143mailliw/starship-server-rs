use std::cell::RefCell;
use std::rc::Rc;

use leptos::{
    create_effect, create_signal, create_trigger, on_cleanup, view, IntoView, ReadSignal,
    SignalGet, SignalSet, Trigger,
};
use log::error;
use stylers::style;
use toolbox_types::observers::Observable;
use toolbox_types::tree::page::{Page, Title};
use toolbox_types::tree::{NodeBase, NodeFeature};

use crate::rendering::nodes::Children;

pub fn create_page(
    page: Rc<RefCell<Page>>,
    features: Vec<NodeFeature>,
) -> (ReadSignal<Rc<RefCell<Page>>>, Trigger) {
    let trigger = create_trigger();
    let (page_sig, _set_page) = create_signal(page);
    // we need to keep our observer closure in scope for the lifetime of this signal
    let (_rc, set_rc) = create_signal::<Option<Rc<RefCell<dyn FnMut()>>>>(None);
    let (ids, set_ids) = create_signal::<Vec<String>>(vec![]);

    let closure = move || {
        let x = trigger.try_notify();
        if !x {
            error!("no reactive runtime found while calling observer");
        }
    };

    create_effect(move |_| {
        let cell = page_sig.get();
        let mut page = cell.borrow_mut();

        let rc: Rc<RefCell<dyn FnMut()>> = Rc::new(RefCell::new(closure));
        set_rc.set(Some(rc.clone()));
        set_ids.set(
            features
                .clone()
                .iter()
                .map(|feature| page.register(*feature, &rc).id.clone())
                .collect(),
        );

        let id = page.id().clone();
        drop(page);
        id
    });

    on_cleanup(move || {
        let mut page = page_sig.get();

        for id in ids.get() {
            page.unregister(&id);
        }
    });

    (page_sig, trigger)
}

pub fn render(page: Rc<RefCell<Page>>) -> impl IntoView {
    let (page_sig, trigger) =
        create_page(page, vec![NodeFeature::Properties, NodeFeature::Children]);

    let class_name = style! {
        #page {
            width: 60vw;
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
        <div id="page">
            <h1 id="page-title">{move || {
                trigger.track();
                match &page_sig.get().borrow().title {
                    Title::Basic { content } => content.clone(),
                }
            }}</h1>
            <div>
                {move || {
                    trigger.track();
                    let children = page_sig.get().get_children();
                    view! {<Children nodes={children}/>}
                }}
            </div>
        </div>
    }
}
