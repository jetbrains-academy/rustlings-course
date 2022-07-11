fn main() {
    let reference_to_nothing = dangle();
}

// !!! ERROR: returning reference for a local variable
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}