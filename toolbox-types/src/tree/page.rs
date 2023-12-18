use nanoid::nanoid;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::errors::{EventError, TreeError};
use crate::events::{EventVariants, Type};
use crate::observers::{Observable, Observer};
use crate::project::Project;
use crate::styles::stylesheet::{StyleLayers, StyleOption, Stylesheet};
use crate::styles::types::{FlexDirection, Layout, Margin, Scale};
use crate::tree::{Node, NodeFeature, ValidNode};

static PAGE_AUTO_STYLES: Stylesheet = Stylesheet {
    margin: StyleOption::Unsupported,
    padding: StyleOption::Some(Margin {
        top: Scale::Pixels(0.0),
        bottom: Scale::Pixels(0.0),
        left: Scale::Pixels(0.0),
        right: Scale::Pixels(0.0),
    }),
    layout: StyleOption::Some(Layout::Flex {
        direction: FlexDirection::ColumnDown,
    }),
    transform: StyleOption::Unsupported,
    font: StyleOption::Unsupported,
    background: StyleOption::Unsupported,
    border: StyleOption::Unsupported,
    text_direction: StyleOption::Unsupported,
};

pub enum Route {
    Basic { path: String },
    Typed { name: String, value_type: Type },
    IdRef { name: String, id: String },
}

pub enum Title {
    Basic { content: String },
}

pub struct Page {
    id: String,
    name: String,
    styles: StyleLayers,
    observers: Vec<Observer<NodeFeature>>,
    this_node: Weak<RefCell<Page>>,
    children: Vec<Rc<RefCell<ValidNode>>>,
    project: Weak<RefCell<Project>>,
    pub title: Title,
    pub route: Vec<Route>,
}

impl Page {
    #[must_use]
    pub fn create(name: String, project: Weak<RefCell<Project>>) -> Rc<RefCell<Page>> {
        Rc::new_cyclic(|this| {
            let node = Page {
                id: nanoid!(),
                name: name.clone(),
                styles: StyleLayers {
                    base: Stylesheet {
                        margin: StyleOption::Unsupported,
                        padding: StyleOption::Default,
                        layout: StyleOption::Default,
                        transform: StyleOption::Unsupported,
                        font: StyleOption::Unsupported,
                        background: StyleOption::Unsupported,
                        border: StyleOption::Unsupported,
                        text_direction: StyleOption::Unsupported,
                    },
                    hover: StyleOption::Unsupported,
                    active: StyleOption::Unsupported,
                    focused: StyleOption::Unsupported,
                    checked: StyleOption::Unsupported,
                },
                observers: vec![],
                this_node: this.clone(),
                children: vec![],
                project,
                title: Title::Basic {
                    content: name.clone(),
                },
                route: vec![Route::Basic {
                    path: name.to_ascii_lowercase().replace(' ', "_"),
                }],
            };

            RefCell::new(node)
        })
    }

    pub fn project(&self) -> Option<Rc<RefCell<Project>>> {
        self.project.upgrade()
    }

    pub fn set_project(&mut self, project: Weak<RefCell<Project>>) {
        self.project = project;
    }
}

impl Observable<NodeFeature> for Page {
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

impl Node for Page {
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
        None
    }

    // Setters
    fn set_name(&mut self, name: String) {
        self.name = name;
        self.commit_changes(NodeFeature::Metadata);
    }

    fn set_parent(&mut self, _parent: Weak<RefCell<ValidNode>>) {
        panic!("tried to call set_parent on Page");
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
        let cloned = node.clone();
        let candidate_node = cloned
            .try_borrow_mut()
            .map_err(|_| TreeError::ChildBorrowed)?;

        if &self.id == candidate_node.id() {
            return Err(TreeError::SelfParent);
        }

        self.children
            .insert(index.unwrap_or(self.children.len()), node);
        Ok(())
    }

    fn remove_child(&mut self, id: String) {
        self.children.retain(|v| v.borrow().id() != &id);
    }

    // Events
    fn get_events(&self) -> Vec<EventVariants> {
        vec![EventVariants::Scrolled]
    }

    fn send_event(&self, event: crate::events::Event) -> Result<(), EventError> {
        todo!();
    }

    // Styles
    fn get_default_styles(&self) -> &'static Stylesheet {
        &PAGE_AUTO_STYLES
    }

    fn styles(&mut self) -> &mut StyleLayers {
        &mut self.styles
    }
}
