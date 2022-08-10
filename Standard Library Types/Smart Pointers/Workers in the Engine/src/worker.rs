use std::cell::RefCell;
use std::rc::Rc;

pub struct Worker {
    id: usize,
    log: Rc<RefCell<Vec<String>>>
}

impl Worker {
    pub fn new(id: usize, log: Rc<RefCell<Vec<String>>>) -> Self {
        Worker { id, log }
    }

    pub fn run(&self) {
        self.log.borrow_mut().push(format!("Worker {} did some work", self.id))
    }
}