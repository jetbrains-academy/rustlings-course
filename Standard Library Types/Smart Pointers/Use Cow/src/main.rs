use std::borrow::Cow;
use use_cow::abs_all;

fn main() {
    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    let output = abs_all(&mut input);
    println!("{:?}", output);
}
