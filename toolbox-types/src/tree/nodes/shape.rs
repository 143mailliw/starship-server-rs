use nanoid::nanoid;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::errors::{EventError, TreeError};
use crate::events::EventVariants;
use crate::styles::stylesheet::{StyleLayers, StyleOption, Stylesheet};
use crate::styles::types::{
    Border, Color, Corners, Graphic, Margin, Scale, ThemedColor, Transform,
};
use crate::tree::node::Observer;
use crate::tree::{Node, NodeFeature, ValidNode};

static SHAPENODE_AUTO_STYLES: Stylesheet = Stylesheet {
    margin: StyleOption::Some(Margin {
        top: Scale::Pixels(0.0),
        bottom: Scale::Pixels(0.0),
        left: Scale::Pixels(0.0),
        right: Scale::Pixels(0.0),
    }),
    padding: StyleOption::Some(Margin {
        top: Scale::Pixels(0.0),
        bottom: Scale::Pixels(0.0),
        left: Scale::Pixels(0.0),
        right: Scale::Pixels(0.0),
    }),
    layout: StyleOption::Unsupported,
    transform: StyleOption::Some(Transform {
        size_x: Scale::Pixels(100.0),
        size_y: Scale::Pixels(100.0),
        pos_x: Scale::Pixels(0.0),
        pos_y: Scale::Pixels(0.0),
        degrees: 0.0,
    }),
    font: StyleOption::Unsupported,
    background: StyleOption::Some(Graphic::Color(Color::Themed {
        color: ThemedColor::LightBlack,
        alpha: 255,
    })),
    border: StyleOption::Some(Border {
        left: None,
        right: None,
        top: None,
        bottom: None,
        corners: Corners {
            top_left: Scale::Pixels(0.0),
            top_right: None,
            bottom_left: None,
            bottom_right: None,
            locked: true,
        },
    }),
    text_direction: StyleOption::Unsupported,
};

pub struct ShapeNode {
    id: String,
    name: String,
    styles: StyleLayers,
    observers: Vec<Observer>,
    parent: Option<Weak<RefCell<ValidNode>>>,
    this_node: Weak<RefCell<ValidNode>>,
    children: Vec<Rc<RefCell<ValidNode>>>,
}

impl Node for ShapeNode {
    fn new() -> Rc<RefCell<ValidNode>> {
        Rc::new_cyclic(|this| {
            let mut node = ShapeNode {
                id: nanoid!(),
                name: "Text".to_string(),
                styles: StyleLayers {
                    base: Stylesheet {
                        margin: StyleOption::Default,
                        padding: StyleOption::Default,
                        layout: StyleOption::Unsupported,
                        transform: StyleOption::Default,
                        font: StyleOption::Default,
                        background: StyleOption::Default,
                        border: StyleOption::Unsupported,
                        text_direction: StyleOption::Default,
                    },
                    hover: StyleOption::Default,
                    active: StyleOption::Default,
                    focused: StyleOption::Unsupported,
                    checked: StyleOption::Unsupported,
                },
                observers: vec![],
                parent: None,
                this_node: this.clone(),
                children: vec![],
            };

            RefCell::new(ValidNode::Shape(node))
        })
    }

    // Getters
    fn id(&self) -> &String {
        &self.id
    }

    fn features(&self) -> Vec<NodeFeature> {
        vec![
            NodeFeature::Styles,
            NodeFeature::Events,
            NodeFeature::Properties,
            NodeFeature::Metadata,
            NodeFeature::Children,
        ]
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn parent(&self) -> Option<Weak<RefCell<ValidNode>>> {
        self.parent.clone()
    }

    // Setters
    fn set_name(&mut self, name: String) {
        self.name = name;
        self.commit_changes(NodeFeature::Metadata);
    }

    fn set_parent(&mut self, parent: Weak<RefCell<ValidNode>>) {
        self.parent = Some(parent);
        self.commit_changes(NodeFeature::Metadata);
    }

    // Children
    fn get_children(&self) -> Option<Vec<Rc<RefCell<ValidNode>>>> {
        Some(self.children.iter().map(Rc::clone).collect())
    }

    fn add_child(
        &mut self,
        node: Weak<RefCell<ValidNode>>,
        index: Option<usize>,
    ) -> Result<(), TreeError> {
        if let Some(candidate_node) = node.upgrade() {
            let mut curr_node: Option<Rc<RefCell<ValidNode>>> = self.this_node.upgrade();

            while let Some(node) = curr_node.clone() {
                if node.borrow().id() == candidate_node.borrow().id() {
                    return Err(TreeError::Loop);
                }

                let node_opt = node.borrow().parent();
                curr_node = node_opt.and_then(|v| v.upgrade());
            }

            candidate_node
                .borrow_mut()
                .set_parent(self.this_node.clone());
            self.children
                .insert(index.unwrap_or(self.children.len()), candidate_node);
            Ok(())
        } else {
            Err(TreeError::DoesNotExist)
        }
    }

    fn remove_child(&mut self, id: String) {
        self.children.retain(|v| v.borrow().id() != &id);
    }

    // Events
    fn get_events(&self) -> Vec<EventVariants> {
        vec![
            EventVariants::Clicked,
            EventVariants::RightClicked,
            EventVariants::Hovered,
            EventVariants::Scrolled,
        ]
    }

    fn send_event(&self, event: crate::events::Event) -> Result<(), EventError> {
        todo!();
    }

    // Styles
    fn get_default_styles(&self) -> &'static Stylesheet {
        &SHAPENODE_AUTO_STYLES
    }

    fn styles(&mut self) -> &mut StyleLayers {
        &mut self.styles
    }

    // Tracking
    fn register(&mut self, feature: NodeFeature, func: &Rc<RefCell<dyn FnMut()>>) -> &Observer {
        let watcher = Observer {
            id: nanoid!(),
            func: Rc::<RefCell<dyn FnMut()>>::downgrade(func),
            feature,
        };

        self.observers.push(watcher);

        self.observers.last().unwrap()
    }

    fn unregister(&mut self, id: String) {
        self.observers.retain(|v| v.id != id);
    }

    fn commit_changes(&self, feature: NodeFeature) {
        for observer in &self.observers {
            if observer.feature == feature {
                observer.call();
            }
        }
    }
}
