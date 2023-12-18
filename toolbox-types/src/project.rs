#![allow(clippy::missing_panics_doc)] // only panic in this code is impossible
use nanoid::nanoid;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::observers::{Observable, Observer};
use crate::tree::page::Page;

#[derive(Clone, Copy)]
pub enum Type {
    Component,
    Library,
}

pub struct Project {
    name: String,
    version: u16,
    project_type: Type,
    pages: Option<Vec<Rc<RefCell<Page>>>>,
    this_project: Weak<RefCell<Project>>,
    observers: Vec<Observer<Field>>,
}

impl Project {
    #[must_use]
    pub fn create(&self, name: String, project_type: Type) -> Rc<RefCell<Project>> {
        Rc::new_cyclic(|this| {
            let project = Project {
                name,
                project_type,
                version: 0,
                pages: Some(vec![]),
                this_project: this.clone(),
                observers: vec![],
            };

            RefCell::new(project)
        })
    }

    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    #[must_use]
    pub fn version(&self) -> u16 {
        self.version
    }

    #[must_use]
    pub fn project_type(&self) -> Type {
        self.project_type
    }

    #[must_use]
    pub fn pages(&self) -> Option<Vec<Rc<RefCell<Page>>>> {
        self.pages
            .as_ref()
            .map(|v| v.iter().map(Rc::clone).collect())
    }

    pub fn add_page(&mut self, page: &Rc<RefCell<Page>>) {
        if let Some(pages) = self.pages.as_mut() {
            pages.push(page.clone());
            page.borrow_mut().set_project(self.this_project.clone());
        }
    }
}

impl Observable<Field> for Project {
    fn register(&mut self, item: Field, func: &Rc<RefCell<dyn FnMut()>>) -> &Observer<Field> {
        let observer = Observer {
            id: nanoid!(),
            func: Rc::downgrade(func),
            item,
        };

        self.observers.push(observer);

        self.observers.last().unwrap()
    }

    fn unregister(&mut self, id: &str) {
        self.observers.retain(|v| v.id != *id);
    }

    fn commit_changes(&self, item: Field) {
        for observer in &self.observers {
            if observer.item == item {
                observer.call();
            }
        }
    }
}

#[derive(PartialEq)]
pub enum Field {
    Name,
    Version,
    Type,
    Pages,
}
