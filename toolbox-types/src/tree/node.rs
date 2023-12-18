#![allow(clippy::module_name_repetitions)]
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use enum_dispatch::enum_dispatch;

use super::{
    nodes::{ShapeNode, TextNode},
    page::Page,
};
use crate::{
    errors::{EventError, TreeError},
    events::{Event, EventVariants},
    observers::Observable,
    styles::stylesheet::{StyleLayers, Stylesheet},
};

#[enum_dispatch(ValidNode)]
pub trait Node: Observable<NodeFeature> {
    // Getters

    /// Returns the Node's ID.
    #[must_use]
    fn id(&self) -> &String;

    /// Returns a the NodeFeatures supported by the Node. This indicates what functions are valid
    /// for this node. For example, some nodes may not support children.
    #[must_use]
    fn features(&self) -> Vec<NodeFeature>;

    /// Returns the Node's name.
    #[must_use]
    fn name(&self) -> &String;

    /// Returns a weak reference to the parent Node, if it exists.
    #[must_use]
    fn parent(&self) -> Option<Weak<RefCell<ValidNode>>>;

    /// Returns a weak reference to the page the Node is a member of. Returns None if this Node is
    /// not associated with any pages.
    fn page(&self) -> Option<Weak<RefCell<Page>>>;

    // Setters

    /// Sets the Node's name.
    fn set_name(&mut self, name: String);

    /// Sets the Node's parent.
    fn set_parent(&mut self, parent: Weak<RefCell<ValidNode>>);

    /// Sets the Node's page.
    fn set_page(&mut self, page: Option<Weak<RefCell<Page>>>);

    // Children

    /// Returns the children of the Node.
    #[must_use]
    fn get_children(&self) -> Option<Vec<Rc<RefCell<ValidNode>>>> {
        None
    }

    /// Adds a child to the Node (if possible).
    ///
    /// # Errors
    /// This function will return an error if the Node does not support children or if adding the
    /// specified node as a child of the current node would result in a loop.
    fn add_child(
        &mut self,
        node: Rc<RefCell<ValidNode>>,
        index: Option<usize>,
    ) -> Result<(), TreeError> {
        Err(TreeError::ChildrenUnsupported)
    }

    /// Removes a child from the node based on it's ID.
    fn remove_child(&mut self, id: String) {}
    // Events

    /// Returns the Events supported by the Node.
    #[must_use]
    fn get_events(&self) -> Vec<EventVariants>;

    /// Sends an Event to this Node.
    ///
    /// # Errors
    /// This function will return an error if an error occurs during the execution of the action
    /// associated with the handler for this event.
    fn send_event(&self, event: Event) -> Result<(), EventError>;

    // Styles

    /// Returns a reference to the default Stylesheet for the Node.
    fn get_default_styles(&self) -> &Stylesheet;

    /// Returns a mutable reference to the Node's current styles. Updates to styles must be
    /// committed with `commit_changes`, otherwise the updates will not be broadcast to Watchers.
    fn styles(&mut self) -> &mut StyleLayers;
}

#[enum_dispatch]
pub enum ValidNode {
    TextNode,
    ShapeNode,
}

// this kinda sucks but enum_dispatch does not support generic traits on non-generic enums
impl Observable<NodeFeature> for ValidNode {
    fn register(
        &mut self,
        item: NodeFeature,
        func: &Rc<RefCell<dyn FnMut()>>,
    ) -> &crate::observers::Observer<NodeFeature> {
        match self {
            ValidNode::TextNode(v) => v.register(item, func),
            ValidNode::ShapeNode(v) => v.register(item, func),
        }
    }

    fn unregister(&mut self, id: &str) {
        match self {
            ValidNode::TextNode(v) => v.unregister(id),
            ValidNode::ShapeNode(v) => v.unregister(id),
        }
    }

    fn commit_changes(&self, item: NodeFeature) {
        match self {
            ValidNode::TextNode(v) => v.commit_changes(item),
            ValidNode::ShapeNode(v) => v.commit_changes(item),
        }
    }
}

#[derive(PartialEq)]
pub enum NodeFeature {
    Styles,
    Children,
    Events,
    Properties,
    Metadata,
}
