use std::rc::Rc;
use std::cell::RefCell;

pub struct Signal<T> {
    value: Rc<RefCell<T>>,
}

impl<T: Clone> Signal<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(value)),
        }
    }

    pub fn get(&self) -> T {
        self.value.borrow().clone()
    }

    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        let mut value = self.value.borrow_mut();
        f(&mut *value);
        // In a real implementation, this would trigger UI updates
    }
}

pub fn create_signal<T: Clone>(value: T) -> Signal<T> {
    Signal::new(value)
}
