use leptos::{
    create_effect, create_signal, create_trigger, on_cleanup, ReadSignal, SignalGet, SignalSet,
    Trigger,
};
use log::{error, info};
use std::cell::RefCell;
use std::rc::Rc;
use toolbox_types::observers::Observable;
use toolbox_types::tree::{NodeBase, NodeFeature, ValidNode};

pub fn create_node(
    node: Rc<RefCell<ValidNode>>,
    features: Vec<NodeFeature>,
) -> (ReadSignal<Rc<RefCell<ValidNode>>>, Trigger) {
    let trigger = create_trigger();
    let (node_sig, _set_node) = create_signal(node);
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
        let cell = node_sig.get();
        let mut node = cell.borrow_mut();

        let rc: Rc<RefCell<dyn FnMut()>> = Rc::new(RefCell::new(closure));
        set_rc.set(Some(rc.clone()));
        set_ids.set(
            features
                .clone()
                .iter()
                .map(|feature| node.register(*feature, &rc).id.clone())
                .collect(),
        );

        let id = node.id().clone();
        drop(node);
        id
    });

    on_cleanup(move || {
        let cell = node_sig.get();
        let mut node = cell.borrow_mut();

        for id in ids.get() {
            node.unregister(&id);
        }
    });

    (node_sig, trigger)
}
