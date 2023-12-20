use nanoid::nanoid;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::errors::{EventError, TreeError};
use crate::events::EventVariants;
use crate::observers::{Observable, Observer};
use crate::styles::stylesheet::{StyleLayers, StyleOption, Stylesheet};
use crate::styles::types::{
    Border, Color, Corners, FlexDirection, Graphic, Layout, Margin, Scale, ThemedColor, Transform,
};
use crate::tree::page::Page;
use crate::tree::{CreatableNode, NodeBase, NodeFeature, RegularNode, ValidNode};

use super::util::add_child;

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
    layout: StyleOption::Some(Layout::Flex {
        direction: FlexDirection::ColumnDown,
    }),
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
    observers: Vec<Observer<NodeFeature>>,
    parent: Option<Weak<RefCell<ValidNode>>>,
    page: Option<Weak<RefCell<Page>>>,
    this_node: Weak<RefCell<ValidNode>>,
    children: Vec<Rc<RefCell<ValidNode>>>,
}

impl CreatableNode for ShapeNode {
    fn create() -> Rc<RefCell<ValidNode>> {
        Rc::new_cyclic(|this| {
            let node = ShapeNode {
                id: nanoid!(),
                name: "Text".to_string(),
                styles: StyleLayers {
                    base: Stylesheet {
                        margin: StyleOption::Default,
                        padding: StyleOption::Default,
                        layout: StyleOption::Default,
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
                page: None,
                this_node: this.clone(),
                children: vec![],
            };

            RefCell::new(node.into())
        })
    }
}

impl Observable<NodeFeature> for ShapeNode {
    fn register(
        &mut self,
        item: NodeFeature,
        func: &Rc<RefCell<dyn FnMut()>>,
    ) -> &Observer<NodeFeature> {
        let watcher = Observer {
            id: nanoid!(),
            func: Rc::<RefCell<dyn FnMut()>>::downgrade(func),
            item,
        };

        self.observers.push(watcher);

        self.observers.last().unwrap()
    }

    fn unregister(&mut self, id: &str) {
        self.observers.retain(|v| v.id != id);
    }

    fn commit_changes(&self, item: NodeFeature) {
        for observer in &self.observers {
            if observer.item == item {
                observer.call();
            }
        }
    }
}

impl NodeBase for ShapeNode {
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

    // Setters
    fn set_name(&mut self, name: String) {
        self.name = name;
        self.commit_changes(NodeFeature::Metadata);
    }

    // Children
    fn get_children(&self) -> Option<Vec<Rc<RefCell<ValidNode>>>> {
        Some(self.children.iter().map(Rc::clone).collect())
    }

    fn add_child(
        &mut self,
        node: Rc<RefCell<ValidNode>>,
        index: Option<usize>,
    ) -> Result<(), TreeError> {
        add_child(
            node,
            index,
            &self.id,
            &self.this_node,
            &mut self.children,
            &self.parent,
            &self.page,
        )
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
}

impl RegularNode for ShapeNode {
    // Getters
    fn parent(&self) -> Option<Weak<RefCell<ValidNode>>> {
        self.parent.clone()
    }

    fn page(&self) -> Option<Weak<RefCell<Page>>> {
        self.page.clone()
    }

    // Setters
    fn set_parent(&mut self, parent: Weak<RefCell<ValidNode>>) {
        self.parent = Some(parent);
        self.commit_changes(NodeFeature::Metadata);
    }

    fn set_page(&mut self, page: Option<Weak<RefCell<Page>>>) {
        self.page = page;
        self.commit_changes(NodeFeature::Metadata);
    }
}
