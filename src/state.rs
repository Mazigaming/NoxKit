use std::rc::Rc;
use std::cell::RefCell;

pub struct Signal<T> {
    value: Rc<RefCell<T>>,
    listeners: Rc<RefCell<Vec<Box<dyn Fn()>>>>,
}

impl<T: Clone> Signal<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(value)),
            listeners: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn get(&self) -> T {
        self.value.borrow().clone()
    }

    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        {
            let mut value = self.value.borrow_mut();
            f(&mut *value);
        }
        self.notify();
    }

    pub fn subscribe<F>(&self, f: F)
    where
        F: Fn() + 'static,
    {
        self.listeners.borrow_mut().push(Box::new(f));
    }

    fn notify(&self) {
        for listener in self.listeners.borrow().iter() {
            listener();
        }
    }
}

impl<T: Clone> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            listeners: self.listeners.clone(),
        }
    }
}

pub struct Computed<T> {
    value: Rc<RefCell<T>>,
}

impl<T: Clone + 'static> Computed<T> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn() -> T + 'static,
    {
        let value = Rc::new(RefCell::new(f()));
        Self { value }
    }

    pub fn get(&self) -> T {
        self.value.borrow().clone()
    }
}

impl<T: Clone + 'static> Clone for Computed<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

pub fn create_computed<T, F>(f: F) -> Computed<T>
where
    T: Clone + 'static,
    F: Fn() -> T + 'static,
{
    Computed::new(f)
}

pub fn create_signal<T: Clone>(value: T) -> Signal<T> {
    Signal::new(value)
}

pub fn create_memo<T, S, F>(dependency: &Signal<S>, f: F) -> Computed<T>
where
    T: Clone + 'static,
    S: Clone + 'static,
    F: Fn(S) -> T + 'static,
{
    let dep = dependency.clone();
    let initial_val = f(dep.get());
    let computed_val = Rc::new(RefCell::new(initial_val));
    
    let c_val = computed_val.clone();
    let f = Rc::new(f);
    
    dependency.subscribe(move || {
        let mut val = c_val.borrow_mut();
        *val = f(dep.get());
    });
    
    Computed { value: computed_val }
}
