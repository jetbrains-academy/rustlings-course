use lamp_with_switchers::{lamp::*, switcher::*};

fn main() {
    let lamp = Lamp::default();

    let sw1 = Switcher::new(&lamp);
    let sw2 = Switcher::new(&lamp);

    println!("{:?}", lamp);
    sw1.switch();
    println!("{:?}", lamp);
    sw2.switch();
    println!("{:?}", lamp);
}
