use crate::worker::Worker;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Engine {
    log: Rc<RefCell<Vec<String>>>,
    workers: Vec<Worker>
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            log: Rc::new(RefCell::new(vec![])),
            workers: vec![]
        }
    }

    pub fn add_worker(&mut self, id: usize) {
        self.workers.push(Worker::new(id, Rc::clone(&self.log)))
    }

    pub fn run(&self) {
        self.workers.iter().for_each(Worker::run);
    }

    pub fn print_log(&self) {
        for entry in self.log.borrow().iter() {
            println!("{}", entry)
        }
    }
}