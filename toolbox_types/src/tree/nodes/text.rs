use log::info;
use nanoid::nanoid;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::errors::EventError;
use crate::events::{EventVariants, Type};
use crate::observers::{Observable, Observer};
use crate::styles::stylesheet::{StyleLayers, StyleOption, Stylesheet};
use crate::styles::types::{
    Border, CardinalDirection, Color, Corners, Font, FontWeight, Graphic, Margin, Scale,
    StyleString, ThemedColor, Transform,
};
use crate::tree::node::PropertyError;
use crate::tree::page::Page;
use crate::tree::{CreatableNode, NodeBase, NodeFeature, RegularNode, ValidNode};

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
    observers: Vec<Observer<NodeFeature>>,
    parent: Option<Weak<RefCell<ValidNode>>>,
    page: Option<Weak<RefCell<Page>>>,
    pub text: String,
}

impl CreatableNode for TextNode {
    fn create() -> Rc<RefCell<ValidNode>> {
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
            page: None,
            text: String::new(),
        };

        Rc::new(RefCell::new(node.into()))
    }
}

impl Observable<NodeFeature> for TextNode {
    fn register(
        &mut self,
        item: NodeFeature,
        func: &Rc<RefCell<dyn FnMut()>>,
    ) -> &Observer<NodeFeature> {
        info!("registering observer for feature {:#?}", item);
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
        info!("informing observers for feature {:#?}", item);
        for observer in &self.observers {
            info!("informing {}", observer.id);
            if observer.item == item {
                observer.call();
            }
        }
    }
}

impl NodeBase for TextNode {
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

    fn get_property(&self, name: &str) -> Result<Type, PropertyError> {
        match name {
            "text" => Ok(Type::String(self.text.clone())),
            _ => Err(PropertyError::NotFound),
        }
    }

    // Setters
    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_property(&mut self, name: &str, value: Type, notify: bool) -> Result<(), PropertyError> {
        match name {
            "text" => match value {
                Type::String(v) => {
                    self.text = v;
                    if notify {
                        self.commit_changes(NodeFeature::Properties);
                    }
                    Ok(())
                }
                _ => Err(PropertyError::InvalidType),
            },
            _ => Err(PropertyError::NotFound),
        }
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
}

impl RegularNode for TextNode {
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
