fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn longest2<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    // !!! ERROR: cannot return value referencing local variable `result`
    result.as_str()
}

fn main() {
    // put you code here to launch it
}
