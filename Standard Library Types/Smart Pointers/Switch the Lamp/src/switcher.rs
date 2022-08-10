use crate::lamp::Lamp;

pub struct Switcher<'a> {
    lamp: &'a Lamp
}

impl<'a> Switcher<'a> {
    pub fn new(lamp: &'a Lamp) -> Self {
        Switcher { lamp }
    }
    pub fn switch(&self) {
        if self.lamp.is_on() {
            self.lamp.switch_off()
        } else {
            self.lamp.switch_on()
        }
    }
}