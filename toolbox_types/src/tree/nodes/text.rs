use nanoid::nanoid;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::errors::EventError;
use crate::events::{EventVariants, Type};
use crate::observers::{Observable, Observer};
use crate::styles::stylesheet::{StyleLayers, StyleOption, Stylesheet};
use crate::styles::types::{
    Border, Color, Corners, Direction, Font, FontWeight, Graphic, Locked, Margin, Scale,
    StyleString, TextAlignment, ThemedColor, Transform,
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
        locked: Locked::All,
    }),
    padding: StyleOption::Some(Margin {
        top: Scale::Pixels(0.0),
        bottom: Scale::Pixels(0.0),
        left: Scale::Pixels(0.0),
        right: Scale::Pixels(0.0),
        locked: Locked::All,
    }),
    layout: StyleOption::Unsupported,
    transform: StyleOption::Some(Transform {
        size_x: Scale::Auto,
        size_y: Scale::Auto,
        pos_x: Scale::Pixels(0.0),
        pos_y: Scale::Pixels(0.0),
        anchor: Direction::TopLeft,
        degrees: 0.0,
    }),
    font: StyleOption::Some(Font {
        name: StyleString::Static("Inter"),
        weight: FontWeight::Normal,
        size: Scale::Points(11.0),
        color: Color::Themed {
            color: ThemedColor::Black,
            alpha: 1.0,
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
        locked: Locked::All,
    }),
    text_direction: StyleOption::Some(TextAlignment::Left),
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

    fn send_event(&self, _event: crate::events::Event) -> Result<(), EventError> {
        todo!();
    }

    // Styles
    fn get_default_styles(&self) -> &'static Stylesheet {
        &TEXTNODE_AUTO_STYLES
    }

    fn styles(&mut self) -> &mut StyleLayers {
        &mut self.styles
    }

    fn get_styles(&self) -> StyleLayers {
        let base_styles = self.styles.clone();
        StyleLayers {
            base: base_styles.base.merge(TEXTNODE_AUTO_STYLES.clone()),
            hover: base_styles.hover,
            active: base_styles.active,
            focused: base_styles.focused,
            checked: base_styles.checked,
        }
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
    }

    fn set_page(&mut self, page: Option<Weak<RefCell<Page>>>) {
        self.page = page;
    }
}
