fn lifetime_examples<'a>() {
    let r1: &i32;        // a reference
    let r2: &'a i32;     // a reference with an explicit lifetime
    let r3: &'a mut i32; // a mutable reference with an explicit lifetime
}

fn main() {
    // put you code here to launch it
}
