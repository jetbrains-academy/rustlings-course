fn main() {
    let mut hello = String::from("Hello");

    add_exclamation(&mut hello);

    println!("{} is `{}`", "hello", hello);

    add_exclamation(&mut hello);

    println!("{} is `{}`", "hello", hello);
}

fn add_exclamation(s: &mut String) {
    s.push_str("!");
}
