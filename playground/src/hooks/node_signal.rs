use leptos::{
    create_effect, create_signal, create_trigger, ReadSignal, SignalGet, SignalSet, Trigger,
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
    let (setup, set_setup) = create_signal(false);

    let closure = move || {
        info!("mwomp");
        let x = trigger.try_notify();
        if !x {
            error!("no reactive runtime found while calling observer");
        }
    };

    let rc: Rc<RefCell<dyn FnMut()>> = Rc::new(RefCell::new(closure));
    set_rc.set(Some(rc.clone()));

    create_effect(move |_| {
        let cell = node_sig.get();
        let mut node = cell.borrow_mut();

        if !setup.get() {
            for feature in features.clone() {
                info!("{:#?}", feature);

                node.register(feature, &rc);
            }
            set_setup.set(true);
        }

        info!("count changed");

        let id = node.id().clone();

        drop(node);

        id
    });

    (node_sig, trigger)
}
