use trim_a_string::trimmed_space;

fn main() {
    let str_literal = "   Rust   ";
    println!("{}", trimmed_space(str_literal));

    let string = String::from("  Rust  ");
    println!("{}", trimmed_space(&string[..]));
}
