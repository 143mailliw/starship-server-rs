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

        let result = node_ref.get_property(name);
        drop(node_ref);
        result
    }

    fn set_name(&mut self, name: String) {
        let clone = self.clone();
        let mut node_ref = clone.borrow_mut();

        let result = node_ref.set_name(name);
        drop(node_ref);

        self.commit_changes(NodeFeature::Metadata);
    }

    fn set_property(&mut self, name: &str, value: Type, notify: bool) -> Result<(), PropertyError> {
        let clone = self.clone();
        let mut node_ref = clone.borrow_mut();

        let result = node_ref.set_property("text", Type::String(":)".to_string()), false);
        drop(node_ref);

        if notify {
            let mut node = self.borrow();
            node.commit_changes(NodeFeature::Properties);
        }

        result
    }

    fn get_events(&self) -> Vec<EventVariants> {
        todo!()
    }

    fn send_event(&self, event: Event) -> Result<(), EventError> {
        todo!()
    }

    fn get_default_styles(&self) -> &Stylesheet {
        panic!("get_default_styles requires long-life reference");
    }

    fn styles(&mut self) -> &mut StyleLayers {
        panic!("styles requires long-life reference");
    }
}
