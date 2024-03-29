use std::cell::RefCell;
use std::rc::{Rc, Weak};

use log::warn;

pub struct Observer<T> {
    pub id: String,
    pub func: Weak<RefCell<dyn FnMut()>>,
    pub item: T,
}

impl<T> Observer<T> {
    pub fn call(&self) {
        if let Some(cell) = self.func.upgrade() {
            let mut func = cell.borrow_mut();
            func();
        } else {
            warn!(
                "Observer {} has been dropped, but it is still registered. This will leak memory for the lifetime of the Observable.",
                self.id
            );
        }
    }
}

pub trait Observable<T> {
    /// Registers a Observer on this Observable with the given item.
    fn register(&mut self, item: T, func: &Rc<RefCell<dyn FnMut()>>) -> &Observer<T>;

    /// Removes a registered Observer from this Observable.
    fn unregister(&mut self, id: &str);

    /// Informs all Observers associated with this Observable that an update has been performed.
    fn commit_changes(&self, item: T);
}
