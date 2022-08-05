use std::cell::RefCell;
use std::rc::Rc;
use crate::lamp::Lamp;

pub struct Switcher {
    lamp: Rc<RefCell<Lamp>>
}

impl Switcher {
    pub fn new(lamp: &Rc<RefCell<Lamp>>) -> Self {
        Switcher { lamp: Rc::clone(lamp) }
    }
    pub fn switch(&self) {
        if self.lamp.borrow().is_on() {
            self.lamp.borrow_mut().switch_off()
        } else {
            self.lamp.borrow_mut().switch_on()
        }
    }
}