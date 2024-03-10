use std::rc::Rc;
use std::{cell::RefCell, rc::Weak};

use log::error;

use crate::{
    errors::EventError,
    events::{Event, EventVariants, Type},
    observers::Observable,
    styles::stylesheet::{StyleLayers, Stylesheet},
};

use super::nodes::util;
use super::page::Page;
use super::{node::PropertyError, NodeBase, NodeFeature, RegularNode, ValidNode};

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
        unimplemented!("id requires long-life reference, use NodeRc::get_id");
    }

    fn features(&self) -> Vec<NodeFeature> {
        let borrow = self.borrow();
        borrow.features().clone()
    }

    fn name(&self) -> &String {
        unimplemented!("name requires long-life reference");
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

        let result = node_ref.add_child(node, index);

        drop(node_ref);

        // commit changes
        // this **must** be done with a immutable reference to the node
        // otherwise the renderer will panic with a borrow error
        let node = self.borrow();
        node.commit_changes(NodeFeature::Children);

        result
    }

    fn get_children(&self) -> Option<Vec<Rc<RefCell<ValidNode>>>> {
        let node_ref = self.borrow();
        node_ref.get_children()
    }

    fn move_into(
        &mut self,
        target: Rc<RefCell<ValidNode>>,
        index: Option<usize>,
    ) -> Result<Option<Weak<RefCell<ValidNode>>>, crate::errors::TreeError> {
        let page = self.borrow().page();
        let result = util::move_into_from_reference(self.clone(), target, index)?;

        let node = self.borrow();

        node.commit_changes(NodeFeature::Position);

        drop(node);

        if let Some(previous_parent) = result.clone() {
            let upgraded = previous_parent.upgrade();
            if let Some(previous_parent) = upgraded {
                previous_parent.commit_changes(NodeFeature::Children);
            }
        } else if let Some(page) = page.map(|v| v.upgrade()).flatten() {
            let page_ref = page.borrow();
            page_ref.commit_changes(NodeFeature::Children);
        }

        Ok(result)
    }

    fn get_events(&self) -> Vec<EventVariants> {
        todo!()
    }

    fn send_event(&self, _event: Event) -> Result<(), EventError> {
        todo!()
    }

    fn get_default_styles(&self) -> &Stylesheet {
        unimplemented!("get_default_styles requires long-life reference");
    }

    fn styles(&mut self) -> &mut StyleLayers {
        unimplemented!("styles requires long-life reference");
    }

    fn get_styles(&self) -> StyleLayers {
        let node_ref = self.borrow();
        node_ref.get_styles()
    }
}

impl RegularNode for Rc<RefCell<ValidNode>> {
    fn parent(&self) -> Option<Weak<RefCell<ValidNode>>> {
        let node_ref = self.borrow();
        node_ref.parent().clone()
    }

    fn page(&self) -> Option<Weak<RefCell<Page>>> {
        let node_ref = self.borrow();
        node_ref.page().clone()
    }

    fn set_parent(&mut self, parent: Weak<RefCell<ValidNode>>) {
        let mut node_ref = self.borrow_mut();
        node_ref.set_parent(parent);
    }

    fn set_page(&mut self, page: Option<Weak<RefCell<Page>>>) {
        let mut node_ref = self.borrow_mut();
        node_ref.set_page(page);
    }

    fn get_path(&self) -> Result<String, crate::errors::PathError> {
        let node_ref = self.borrow();
        node_ref.get_path()
    }

    fn detach(&self) {
        let node_ref = self.borrow();
        node_ref.detach();
    }
}

impl Observable<NodeFeature> for Rc<RefCell<ValidNode>> {
    fn register(
        &mut self,
        _item: NodeFeature,
        _func: &Rc<RefCell<dyn FnMut()>>,
    ) -> &crate::observers::Observer<NodeFeature> {
        unimplemented!("register requires long-life reference");
    }

    fn unregister(&mut self, id: &str) {
        let mut node_ref = self.borrow_mut();
        node_ref.unregister(id);
    }

    fn commit_changes(&self, item: NodeFeature) {
        let node_ref = self.borrow();
        node_ref.commit_changes(item);
    }
}
