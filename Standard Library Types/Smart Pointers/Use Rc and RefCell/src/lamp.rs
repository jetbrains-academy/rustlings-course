#[derive(Debug,Default)]
pub struct Lamp {
    on: bool
}

impl Lamp {
    pub fn is_on(&self) -> bool {
        self.on
    }
    pub fn switch_on(&mut self) {
        self.on = true
    }
    pub fn switch_off(&mut self) {
        self.on = false
    }
}
