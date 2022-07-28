fn plus_one(x: Option<i32>) -> Option<i32> {
    // !!! ERROR: Match must be exhaustive
    match x {
        Some(i) => Some(i + 1),
    }
}

fn main() {
    // put you code here to launch it
}
