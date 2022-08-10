use lamp_with_switchers::{lamp::*, switcher::*};

#[test]
fn one_lamp_two_switchers() {
    let lamp = Lamp::default();

    let sw1 = Switcher::new(&lamp);
    let sw2 = Switcher::new(&lamp);

    assert!(!lamp.is_on());
    sw1.switch();
    assert!(lamp.is_on());
    sw2.switch();
    assert!(!lamp.is_on());
}
