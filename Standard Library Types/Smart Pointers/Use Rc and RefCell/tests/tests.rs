use std::cell::RefCell;
use std::rc::Rc;

use lamp_with_switchers::{lamp::*, switcher::*};

#[test]
fn one_lamp_two_switchers() {
    let lamp =
        Rc::new(
            RefCell::new(
                Lamp::default()));

    let sw1 = Switcher::new(&lamp);
    let sw2 = Switcher::new(&lamp);

    assert!(!lamp.borrow().is_on());
    sw1.switch();
    assert!(lamp.borrow().is_on());
    sw2.switch();
    assert!(!lamp.borrow().is_on());
}
