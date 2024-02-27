use std::cell::RefCell;
use std::rc::Rc;

use log::error;

use crate::{
    errors::EventError,
    events::{Event, EventVariants, Type},
    observers::Observable,
    styles::stylesheet::{StyleLayers, Stylesheet},
};

use super::{node::PropertyError, NodeBase, NodeFeature, ValidNode};

pub trait NodeRc {
    fn get_id(&self) -> String;
}

impl NodeRc for Rc<RefCell<ValidNode>> {
    fn get_id(&self) -> String {
        let node_ref = self.borrow();
        node_ref.id().clone()
    }
}

impl NodeBase for Rc<RefCell<ValidNode>> {
    fn id(&self) -> &String {
        panic!("id requires long-life reference");
    }

    fn features(&self) -> Vec<NodeFeature> {
        let borrow = self.borrow();
        borrow.features().clone()
    }

    fn name(&self) -> &String {
        panic!("name requires long-life reference");
    }

    fn get_property(&self, name: &str) -> Result<Type, PropertyError> {
        let node_ref = match self.try_borrow() {
            Ok(v) => v,
            Err(e) => {
                error!("could not borrow node: {}", e);
                return Err(PropertyError::BorrowError);
            }
        };

        node_ref.get_property(name)
    }

    fn set_name(&mut self, name: String) {
        let clone = self.clone();
        let mut node_ref = clone.borrow_mut();

        node_ref.set_name(name);
        drop(node_ref);

        let node = self.borrow();
        node.commit_changes(NodeFeature::Metadata);
    }

    fn set_property(&mut self, name: &str, value: Type, notify: bool) -> Result<(), PropertyError> {
        let clone = self.clone();
        let mut node_ref = clone.borrow_mut();

        let result = node_ref.set_property(name, value, false);

        if notify {
            let node = self.borrow();
            node.commit_changes(NodeFeature::Properties);
        }

        result
    }

    fn add_child(
        &mut self,
        node: Rc<RefCell<ValidNode>>,
        index: Option<usize>,
    ) -> Result<(), crate::errors::TreeError> {
        let clone = self.clone();
        let mut node_ref = clone.borrow_mut();

        node_ref.add_child(node, index)
    }

    fn get_children(&self) -> Option<Vec<Rc<RefCell<ValidNode>>>> {
        let node_ref = self.borrow();
        node_ref.get_children()
    }

    fn get_events(&self) -> Vec<EventVariants> {
        todo!()
    }

    fn send_event(&self, _event: Event) -> Result<(), EventError> {
        todo!()
    }

    fn get_default_styles(&self) -> &Stylesheet {
        panic!("get_default_styles requires long-life reference");
    }

    fn styles(&mut self) -> &mut StyleLayers {
        panic!("styles requires long-life reference");
    }

    fn get_styles(&self) -> StyleLayers {
        let node_ref = self.borrow();
        node_ref.get_styles()
    }
}
