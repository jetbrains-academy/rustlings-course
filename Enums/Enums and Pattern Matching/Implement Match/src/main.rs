use implement_match::*;

fn main() {
    let state = test_match_message_call();
    println!("Color:");
    println!("{:?}", state.color);
    println!("Position:");
    println!("{:?}", state.position);
    println!("Quit");
}