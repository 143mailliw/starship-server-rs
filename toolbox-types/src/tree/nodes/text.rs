use nanoid::nanoid;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::errors::EventError;
use crate::events::EventVariants;
use crate::styles::stylesheet::{StyleLayers, StyleOption, Stylesheet};
use crate::styles::types::{
    Border, CardinalDirection, Color, Corners, Font, FontWeight, Graphic, Margin, Scale,
    StyleString, ThemedColor, Transform,
};
use crate::tree::node::Observer;
use crate::tree::{Node, NodeFeature, ValidNode};

static TEXTNODE_AUTO_STYLES: Stylesheet = Stylesheet {
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
        size_x: Scale::Auto,
        size_y: Scale::Auto,
        pos_x: Scale::Pixels(0.0),
        pos_y: Scale::Pixels(0.0),
        degrees: 0.0,
    }),
    font: StyleOption::Some(Font {
        name: StyleString::Static("Inter"),
        weight: FontWeight::Normal,
        size: Scale::Points(11.0),
        color: Color::Themed {
            color: ThemedColor::LightWhite,
            alpha: 255,
        },
    }),
    background: StyleOption::Some(Graphic::None),
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
    text_direction: StyleOption::Some(CardinalDirection::Left),
};

pub struct TextNode {
    id: String,
    name: String,
    styles: StyleLayers,
    observers: Vec<Observer>,
    parent: Option<Weak<RefCell<ValidNode>>>,
    pub text: String,
}

impl TextNode {
    #[must_use]
    pub fn create() -> Rc<RefCell<ValidNode>> {
        let node = TextNode {
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
            text: String::new(),
        };

        Rc::new(RefCell::new(node.into()))
    }
}

impl Node for TextNode {
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

    // Events
    fn get_events(&self) -> Vec<EventVariants> {
        vec![
            EventVariants::Clicked,
            EventVariants::RightClicked,
            EventVariants::Hovered,
        ]
    }

    fn send_event(&self, event: crate::events::Event) -> Result<(), EventError> {
        todo!();
    }

    // Styles
    fn get_default_styles(&self) -> &'static Stylesheet {
        &TEXTNODE_AUTO_STYLES
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
