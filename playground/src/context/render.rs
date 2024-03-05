use std::cell::RefCell;
use std::rc::Rc;

use leptos::RwSignal;
use toolbox_types::project::Project;
use toolbox_types::tree::page::Page;

#[derive(Debug, Clone, Copy)]
pub enum RenderingContext {
    Editor,
    Viewer,
}

#[derive(Debug, Clone, Copy)]
pub struct EditorContext {
    pub current_page: RwSignal<Rc<RefCell<Page>>>,
    pub pages: RwSignal<Vec<Rc<RefCell<Page>>>>,
    pub project: RwSignal<Rc<RefCell<Project>>>,
}

impl EditorContext {
    pub fn new(
        current_page: Rc<RefCell<Page>>,
        pages: Vec<Rc<RefCell<Page>>>,
        project: Rc<RefCell<Project>>,
    ) -> Self {
        Self {
            current_page: RwSignal::new(current_page),
            pages: RwSignal::new(pages),
            project: RwSignal::new(project),
        }
    }
}
