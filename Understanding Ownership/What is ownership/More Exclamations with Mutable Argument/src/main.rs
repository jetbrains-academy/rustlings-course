fn main() {
    let hello = String::from("Hello");

    let mut hello1 = add_exclamation(hello);

    println!("{} is `{}`", "hello1", hello1);

    hello1.push_str("!");

    println!("{} is `{}`", "hello1", hello1);
}

fn add_exclamation(mut s: String) -> String {
    s.push_str("!");
    s
}
