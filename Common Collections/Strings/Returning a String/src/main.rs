fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

pub fn current_favorite_color() -> String {
    String::from("blue")
}
