use std::cell::Cell;

#[derive(Debug,Default)]
pub struct Lamp {
    on: Cell<bool>
}

impl Lamp {
    pub fn is_on(&self) -> bool {
        self.on.get()
    }
    pub fn switch_on(&self) {
        self.on.set(true)
    }
    pub fn switch_off(&self) {
        self.on.set(false)
    }
}
