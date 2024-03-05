use std::cell::RefCell;
use std::rc::{Rc, Weak};

use log::info;

use crate::errors::TreeError;
use crate::tree::page::Page;
use crate::tree::{NodeBase, RegularNode, ValidNode};

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
