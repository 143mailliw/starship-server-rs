use std::{cell::RefCell, rc::Rc};

use toolbox_types::tree::ValidNode;

#[derive(Clone, Default)]
pub enum Selection {
    #[default]
    None,
    Single(Rc<RefCell<ValidNode>>),
    Multiple(Vec<Rc<RefCell<ValidNode>>>),
}

impl Selection {
    pub fn single(node: Rc<RefCell<ValidNode>>) -> Self {
        Selection::Single(node)
    }

    pub fn multiple(nodes: Vec<Rc<RefCell<ValidNode>>>) -> Self {
        Selection::Multiple(nodes)
    }

    pub fn add(&mut self, node: Rc<RefCell<ValidNode>>) {
        match self {
            Selection::None => *self = Selection::Single(node),
            Selection::Single(v) => *self = Selection::Multiple(vec![v.clone(), node]),
            Selection::Multiple(nodes) => nodes.push(node),
        }
    }

    pub fn add_multiple(&mut self, nodes: Vec<Rc<RefCell<ValidNode>>>) {
        match self {
            Selection::None => *self = Selection::Multiple(nodes),
            Selection::Single(v) => {
                *self = Selection::Multiple(vec![v.clone()].into_iter().chain(nodes).collect())
            }
            Selection::Multiple(existing) => {
                *self = Selection::Multiple(existing.clone().into_iter().chain(nodes).collect())
            }
        }
    }

    pub fn remove(&mut self, node: &Rc<RefCell<ValidNode>>) {
        match self {
            Selection::None => {}
            Selection::Single(v) => {
                if Rc::ptr_eq(v, &node) {
                    *self = Selection::None;
                }
            }
            Selection::Multiple(nodes) => {
                nodes.retain(|n| !Rc::ptr_eq(n, &node));
                if nodes.len() == 1 {
                    *self = Selection::Single(nodes[0].clone());
                } else if nodes.is_empty() {
                    *self = Selection::None;
                }
            }
        }
    }

    pub fn has(&self, node: &Rc<RefCell<ValidNode>>) -> bool {
        match self {
            Selection::None => false,
            Selection::Single(v) => Rc::ptr_eq(v, node),
            Selection::Multiple(nodes) => nodes.iter().any(|n| Rc::ptr_eq(n, node)),
        }
    }

    pub fn toggle(&mut self, node: Rc<RefCell<ValidNode>>) {
        if self.has(&node) {
            self.remove(&node);
        } else {
            self.add(node);
        }
    }
}
