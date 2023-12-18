use std::cell::RefCell;
use std::rc::Weak;

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
        }
    }
}
