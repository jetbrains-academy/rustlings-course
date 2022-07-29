// A function is compiled without lifetime annotations,
// even though the parameter and return type are references
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// The actual signature
//fn first_word<'a>(s: &'a str) -> &'a str { ... }


fn main() {
    // put you code here to launch it
}
