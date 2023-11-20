use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use enum_dispatch::enum_dispatch;

use super::nodes::{ShapeNode, TextNode};
use crate::{
    errors::{EventError, TreeError},
    events::{Event, EventVariants},
    styles::stylesheet::{StyleLayers, Stylesheet},
};

#[enum_dispatch(ValidNode)]
pub trait Node {
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

    // Setters

    /// Sets the Node's name.
    fn set_name(&mut self, name: String);

    /// Sets the Node's parent.
    fn set_parent(&mut self, parent: Weak<RefCell<ValidNode>>);

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
        node: Weak<RefCell<ValidNode>>,
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

    // Tracking

    /// Registers a Observer on this node for one or more Features. Returns a reference to the
    /// created watcher.
    fn register(&mut self, feature: NodeFeature, func: &Rc<RefCell<dyn FnMut()>>) -> &Observer;

    /// Removes a registered Observer from this node.
    fn unregister(&mut self, id: String);

    /// Informs all Observers associated with this Feature that an update has been performed.
    fn commit_changes(&self, feature: NodeFeature);
}

#[enum_dispatch]
pub enum ValidNode {
    TextNode,
    ShapeNode,
}

#[derive(PartialEq)]
pub enum NodeFeature {
    Styles,
    Children,
    Events,
    Properties,
    Metadata,
}

pub struct Observer {
    pub id: String,
    pub func: Weak<RefCell<dyn FnMut()>>,
    pub feature: NodeFeature,
}

impl Observer {
    pub fn call(&self) {
        if let Some(cell) = self.func.upgrade() {
            let mut func = cell.borrow_mut();
            func();
        }
    }
}
