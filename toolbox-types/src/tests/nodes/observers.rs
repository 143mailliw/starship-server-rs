#![allow(dead_code)]
#![allow(unused_imports)]
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::channel;

use crate::observers::Observable;
use crate::tree::nodes::ShapeNode;
use crate::tree::{Node, NodeFeature};

#[test]
fn add_observer() {
    let parent_node = ShapeNode::create();
    let mut borrowed_node = parent_node.borrow_mut();

    let (tx, rx) = channel();
    let closure = move || {
        tx.send("works!").unwrap();
    };
    let rc: Rc<RefCell<dyn FnMut()>> = Rc::new(RefCell::new(closure));

    borrowed_node.register(NodeFeature::Metadata, &rc);
    borrowed_node.commit_changes(NodeFeature::Metadata);

    assert_eq!(rx.recv().unwrap(), "works!");
}

#[test]
fn remove_observer() {
    let parent_node = ShapeNode::create();
    let mut borrowed_node = parent_node.borrow_mut();

    let (tx, rx) = channel();
    let closure = move || {
        tx.send("works!").unwrap();
    };
    let rc: Rc<RefCell<dyn FnMut()>> = Rc::new(RefCell::new(closure));

    let observer = borrowed_node
        .register(NodeFeature::Metadata, &rc)
        .id
        .clone();

    borrowed_node.unregister(&observer);
    borrowed_node.commit_changes(NodeFeature::Metadata);

    rx.try_recv().expect_err("channel not empty");
}
