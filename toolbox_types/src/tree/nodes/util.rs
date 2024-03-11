use std::cell::RefCell;
use std::rc::{Rc, Weak};

use log::info;

use crate::errors::TreeError;
use crate::observers::Observable;
use crate::tree::node_rc::NodeRc;
use crate::tree::page::Page;
use crate::tree::{NodeBase, NodeFeature, RegularNode, ValidNode};

pub(super) fn add_child(
    child_node: Rc<RefCell<ValidNode>>,
    index: Option<usize>,
    id: &str,
    weak_self: &Weak<RefCell<ValidNode>>,
    children: &mut Vec<Rc<RefCell<ValidNode>>>,
    parent_node: &Option<Weak<RefCell<ValidNode>>>,
    page: Option<Weak<RefCell<Page>>>,
) -> Result<(), TreeError> {
    // TODO: inform previous parent (if it exists) that it no longer owns this node
    let cloned = child_node.clone();
    let mut candidate_node = cloned
        .try_borrow_mut()
        .map_err(|_| TreeError::ChildBorrowed)?;

    if id == candidate_node.id() {
        return Err(TreeError::SelfParent);
    }

    if let Some(parent) = parent_node.clone() {
        let mut curr_cell: Option<Rc<RefCell<ValidNode>>> = parent.upgrade();

        while let Some(curr_node) = curr_cell.clone() {
            let borrowed = curr_node
                .try_borrow()
                .map_err(|_| TreeError::ParentBorrowed)?;

            if borrowed.id() == candidate_node.id() {
                return Err(TreeError::Loop);
            }

            let node_opt = borrowed.parent();
            curr_cell = node_opt.and_then(|v| v.upgrade());
        }
    }

    candidate_node.set_parent(weak_self.clone());
    candidate_node.set_page(page.clone());

    children.insert(index.unwrap_or(children.len()), child_node);
    Ok(())
}

pub(crate) fn check_index(
    index: usize,
    destination_id: &String,
    target: Rc<RefCell<ValidNode>>,
) -> usize {
    if let Some(parent) = target.parent().map(|v| v.upgrade()).flatten() {
        if parent.get_id() == *destination_id {
            let curent_index = parent
                .get_children()
                .expect("parent has no children")
                .iter()
                .position(|v| v.get_id() == target.get_id())
                .expect("node not found in parent");

            if index >= curent_index {
                index - 1
            } else {
                index
            }
        } else {
            index
        }
    } else {
        index
    }
}

pub(crate) fn move_into(
    destination: &mut impl NodeBase,
    target: Rc<RefCell<ValidNode>>,
    index: Option<usize>,
) -> Result<Option<Weak<RefCell<ValidNode>>>, TreeError> {
    let original_parent = target.parent();

    if destination.id() == &target.get_id() {
        return Err(TreeError::SelfParent);
    }

    if !destination.features().contains(&NodeFeature::Children) {
        return Err(TreeError::ChildrenUnsupported);
    }

    let checked_index = index.map(|v| check_index(v, destination.id(), target.clone()));
    target.detach();
    destination.add_child(target.clone(), checked_index)?;

    Ok(original_parent)
}

pub fn move_into_from_reference(
    destination: Rc<RefCell<impl NodeBase + Observable<NodeFeature>>>,
    target: Rc<RefCell<ValidNode>>,
    index: Option<usize>,
) -> Result<Option<Weak<RefCell<ValidNode>>>, TreeError> {
    let original_parent = target.parent();
    let original_page = target.page();

    let destination_borrowed = destination.borrow();

    if destination_borrowed.id() == &target.get_id() {
        return Err(TreeError::SelfParent);
    }

    if !destination_borrowed
        .features()
        .contains(&NodeFeature::Children)
    {
        return Err(TreeError::ChildrenUnsupported);
    }

    drop(destination_borrowed);

    let checked_index = index.map(|v| check_index(v, &destination.get_id(), target.clone()));

    target.detach();

    let mut destination_mut = destination.borrow_mut();
    destination_mut.add_child(target.clone(), checked_index)?;

    drop(destination_mut);

    destination.borrow().commit_changes(NodeFeature::Children);
    target.borrow().commit_changes(NodeFeature::Metadata);

    if let Some(parent) = original_parent.clone().and_then(|v| v.upgrade()) {
        parent.commit_changes(NodeFeature::Children);
    } else if let Some(page) = original_page.and_then(|v| v.upgrade()) {
        let page_ref = page.borrow();
        page_ref.commit_changes(NodeFeature::Children);
    }

    Ok(original_parent)
}
