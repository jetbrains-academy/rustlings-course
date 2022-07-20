fn main() {
    let part1 = "Rust ";
    let part2 = "is the ";
    let part3 = "greatest programming language";
    let mut message = String::from(part1);
    message.push_str(part2);
    message.push_str(part3);
    println!("{}", message)
}
