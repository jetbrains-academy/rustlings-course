fn main() {

    // This line should be deleted

    let mut hello1 = with_exclamation();

    println!("{} is `{}`", "hello1", hello1);

    hello1.push_str("!");

    println!("{} is `{}`", "hello1", hello1);
}

fn with_exclamation() -> String {
    let mut str = String::from("Hello");
    str.push_str("!");
    str
}
