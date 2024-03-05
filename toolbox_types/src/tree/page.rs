use nanoid::nanoid;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::errors::{EventError, TreeError};
use crate::events::{EventVariants, Type};
use crate::observers::{Observable, Observer};
use crate::project::Project;
use crate::styles::stylesheet::{StyleLayers, StyleOption, Stylesheet};
use crate::styles::types::{FlexDirection, Layout, Locked, Margin, Scale};
use crate::tree::{ContainerNode, NodeBase, NodeFeature, RegularNode, ValidNode};

use super::node::PropertyError;

static PAGE_AUTO_STYLES: Stylesheet = Stylesheet {
    margin: StyleOption::Unsupported,
    padding: StyleOption::Some(Margin {
        top: Scale::Pixels(0.0),
        bottom: Scale::Pixels(0.0),
        left: Scale::Pixels(0.0),
        right: Scale::Pixels(0.0),
        locked: Locked::All,
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
        let page = Rc::new_cyclic(|this| {
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
                project: project.clone(),
                title: Title::Basic {
                    content: name.clone(),
                },
                route: vec![Route::Basic {
                    path: name.to_ascii_lowercase().replace(' ', "_"),
                }],
            };

            RefCell::new(node)
        });

        project.upgrade().unwrap().borrow_mut().add_page(&page);

        page
    }

    pub fn project(&self) -> Option<Rc<RefCell<Project>>> {
        self.project.upgrade()
    }

    pub fn set_project(&mut self, project: Weak<RefCell<Project>>) {
        self.project = project;
    }

    pub fn find_node_by_path(&self, path: String) -> Option<Rc<RefCell<ValidNode>>> {
        let node_ids = path.split('/');
        let mut last_node: Option<Rc<RefCell<ValidNode>>> = None;

        for id in node_ids {
            if let Some(node) = last_node {
                match node.borrow().get_children() {
                    Some(children) => {
                        last_node = children
                            .iter()
                            .find(|node| {
                                let node = node.try_borrow().unwrap();
                                node.id() == id
                            })
                            .cloned();
                    }
                    None => return None,
                }
            } else {
                last_node = self
                    .children
                    .iter()
                    .find(|node| {
                        let node = node.try_borrow().unwrap();
                        node.id() == id
                    })
                    .cloned();
            }

            last_node.as_ref()?;
        }

        last_node
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

impl NodeBase for Page {
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

    // TODO: default defining this function crashes rustc so don't do that for now, i guess
    fn get_property(&self, name: &str) -> Result<Type, PropertyError> {
        Err(PropertyError::NotFound)
    }

    // Setters
    fn set_name(&mut self, name: String) {
        self.name = name;
        self.commit_changes(NodeFeature::Metadata);
    }

    fn set_property(&mut self, name: &str, value: Type, notify: bool) -> Result<(), PropertyError> {
        Err(PropertyError::NotFound)
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
        let mut candidate_node = cloned
            .try_borrow_mut()
            .map_err(|_| TreeError::ChildBorrowed)?;

        //candidate_node.detach();
        candidate_node.set_page(Some(self.this_node.clone()));

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

    fn get_styles(&self) -> StyleLayers {
        let base_styles = self.styles.clone();
        StyleLayers {
            base: base_styles.base.merge(PAGE_AUTO_STYLES.clone()),
            hover: base_styles.hover,
            active: base_styles.active,
            focused: base_styles.focused,
            checked: base_styles.checked,
        }
    }
}

impl ContainerNode for Page {}
