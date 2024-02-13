use leptos::{create_effect, create_signal, ReadSignal, SignalGet};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use toolbox_types::observers::Observable;
use toolbox_types::tree::{NodeBase, NodeFeature, ValidNode};

pub fn create_node_signal(
    node: Rc<RefCell<ValidNode>>,
    features: Vec<NodeFeature>,
) -> ReadSignal<Rc<RefCell<ValidNode>>> {
    let (count, set_count) = create_signal(0);
    let (node_sig, set_node) = create_signal(node);

    create_effect(move |_| {
        let cell = node_sig.get();
        let mut node = cell.borrow_mut();

        if count.get() == 0 {
            for feature in features.clone() {
                let closure = move || {
                    set_count(count.get() + 1);
                };
                let rc: Rc<RefCell<dyn FnMut()>> = Rc::new(RefCell::new(closure));

                node.register(feature, &rc);
            }
        }

        node.id().clone()
    });

    node_sig
}
