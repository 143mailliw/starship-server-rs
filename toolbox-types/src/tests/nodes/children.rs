#![allow(dead_code)]
#![allow(unused_imports)]
use crate::tree::nodes::{ShapeNode, TextNode};
use crate::tree::Node;

#[test]
fn add_child() {
    let parent_node = ShapeNode::create();
    let child_node = TextNode::create();

    let mut borrowed_parent = parent_node.borrow_mut();

    let parent_id = borrowed_parent.id().clone();
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
}

#[test]
fn remove_child() {
    let parent_node = ShapeNode::create();
    let child_node = TextNode::create();

    let mut borrowed_parent = parent_node.borrow_mut();

    let child_id = child_node.borrow().id().clone();

    borrowed_parent
        .add_child(child_node, None)
        .expect("add_child() failed");

    borrowed_parent.remove_child(child_id);

    let children = borrowed_parent
        .get_children()
        .expect("node does not support children");

    assert_eq!(children.len(), 0, "child not removed from children");
}
