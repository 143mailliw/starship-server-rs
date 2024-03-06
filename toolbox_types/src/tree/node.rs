#![allow(clippy::module_name_repetitions)]
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use enum_dispatch::enum_dispatch;
use log::info;

use super::{
    nodes::{ShapeNode, TextNode},
    page::Page,
};
use crate::{
    errors::{EventError, PathError, TreeError},
    events::{Event, EventVariants, Type},
    observers::Observable,
    styles::stylesheet::{StyleLayers, Stylesheet},
};

pub trait ContainerNode: Observable<NodeFeature> + NodeBase {}

pub trait CreatableNode: RegularNode {
    /// Creates a new instance of the implementing `RegularNode` and returns it as a wrapped
    /// `ValidNode`.
    #[must_use]
    fn create() -> Rc<RefCell<ValidNode>>;
}

#[enum_dispatch]
pub trait RegularNode: Observable<NodeFeature> + NodeBase {
    /// Returns a weak reference to the parent Node, if it exists.
    #[must_use]
    fn parent(&self) -> Option<Weak<RefCell<ValidNode>>>;

    /// Returns a weak reference to the page the Node is a member of. Returns None if this Node is
    /// not associated with any pages.
    fn page(&self) -> Option<Weak<RefCell<Page>>>;

    /// Sets the Node's parent.
    fn set_parent(&mut self, parent: Weak<RefCell<ValidNode>>);

    /// Sets the Node's page.
    fn set_page(&mut self, page: Option<Weak<RefCell<Page>>>);

    /// Detaches this node from it's parent or page (if it is a root node). This will remove the
    /// node from the tree; dropping all references to the node will erase it from memory.
    fn detach(&self) {
        if let Some(parent) = self.parent() {
            if let Some(parent) = parent.upgrade() {
                let mut parent = parent.borrow_mut();
                parent.remove_child(self.id().clone());
            }
        } else if let Some(page) = self.page() {
            if let Some(page) = page.upgrade() {
                let mut page = page.borrow_mut();
                page.remove_child(self.id().clone());
            }
        }
    }

    /// Retrieves the path of the node in the tree. The path is a string that represents the node's
    /// location in the tree. It is formatted as `page_id:parent_id/.../node_id`. The path
    /// will always start with the page's ID.
    ///
    /// When the node is a root node, the path will be `page_id:node_id`.
    fn get_path(&self) -> Result<String, PathError> {
        let mut path = vec![self.id().clone()];

        let mut parent_option = self
            .parent()
            .map(|v| v.upgrade().ok_or(PathError::BrokenParent));

        while let Some(parent) = parent_option.clone() {
            let parent_unwrapped = parent?;
            let node = parent_unwrapped.borrow();

            path.push(node.id().clone());

            parent_option = node
                .parent()
                .map(|v| v.upgrade().ok_or(PathError::BrokenParent));
        }

        path.reverse();

        let page = self
            .page()
            .map(|v| v.upgrade())
            .flatten()
            .ok_or(PathError::NoPage)?;
        let page_ref = page.borrow();
        let id = page_ref.id();

        Ok(format!("{}:{}", id, path.join("/")))
    }
}

#[enum_dispatch]
pub trait NodeBase {
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

    /// Sets a Node's properties.
    fn get_property(&self, name: &str) -> Result<Type, PropertyError>;

    // Setters

    /// Sets the Node's name.
    fn set_name(&mut self, name: String);

    /// Sets a Node's properties.
    fn set_property(&mut self, name: &str, value: Type, notify: bool) -> Result<(), PropertyError>;

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

    /// Returns a copy of the Node's current styles, including the default styles. This is
    /// generated by merging the default styles with the current styles.
    fn get_styles(&self) -> StyleLayers;
}

#[enum_dispatch(NodeBase, RegularNode)]
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

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum NodeFeature {
    Styles,
    Children,
    Events,
    Properties,
    Metadata,
    Position,
}

#[derive(Debug)]
pub enum PropertyError {
    NotFound,
    InvalidType,
    BorrowError,
}
