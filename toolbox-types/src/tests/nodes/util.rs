use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::{cell::RefCell, sync::mpsc::channel};

use crate::observers::{Observable, Observer};
use crate::tree::{CreatableNode, NodeBase, NodeFeature, RegularNode, ValidNode};

pub(super) fn add_child<Parent, Child>() -> (Rc<RefCell<ValidNode>>, Rc<RefCell<ValidNode>>)
where
    Parent: CreatableNode,
    Child: CreatableNode,
{
    let parent_node = Parent::create();
    let child_node = Child::create();

    let cloned = parent_node.clone();
    let mut borrowed_parent = cloned.borrow_mut();

    let child_id = child_node.borrow().id().clone();

    borrowed_parent
        .add_child(child_node, None)
        .expect("add_child() failed");

    let children = borrowed_parent
        .get_children()
        .expect("node does not support children");

    assert_eq!(children.len(), 1, "child not added to children");

    let borrowed_child = children[0].borrow();
    assert_eq!(
        borrowed_child.id(),
        &child_id,
        "incorrect child id: expected '{}' but id is '{}'",
        child_id,
        borrowed_child.id()
    );

    let parent = borrowed_child
        .parent()
        .expect("child node has no parent")
        .upgrade()
        .expect("failed to upgrade child node");
    let same_nodes = parent.as_ptr() == parent_node.as_ptr();

    assert!(same_nodes, "parent is not the same as the actual parent");

    (parent_node, children[0].clone())
}

pub(super) fn remove_child<Parent, Child>()
where
    Parent: CreatableNode,
    Child: CreatableNode,
{
    let (parent, child) = add_child::<Parent, Child>();

    let mut borrowed_parent = parent.borrow_mut();

    let child_id = child.borrow().id().clone();

    borrowed_parent.remove_child(child_id);

    let children = borrowed_parent
        .get_children()
        .expect("node does not support children");

    assert_eq!(children.len(), 0, "child not removed from children");
}

pub(super) fn add_observer<T>() -> (Rc<RefCell<ValidNode>>, String, Receiver<String>)
where
    T: CreatableNode,
{
    let node = T::create();
    let cloned = node.clone();
    let mut borrowed_node = cloned.borrow_mut();

    let (tx, rx) = channel();
    let closure = move || {
        tx.send("works!".to_string()).unwrap();
    };
    let rc: Rc<RefCell<dyn FnMut()>> = Rc::new(RefCell::new(closure));

    let observer_id = borrowed_node
        .register(NodeFeature::Metadata, &rc)
        .id
        .clone();
    borrowed_node.commit_changes(NodeFeature::Metadata);

    assert_eq!(rx.recv().unwrap(), "works!");

    (node, observer_id, rx)
}

pub(super) fn remove_observer<T>()
where
    T: CreatableNode,
{
    let (node, id, rx) = add_observer::<T>();
    let mut borrowed_node = node.borrow_mut();

    borrowed_node.unregister(&id);
    borrowed_node.commit_changes(NodeFeature::Metadata);

    rx.try_recv().expect_err("channel not empty");
}
