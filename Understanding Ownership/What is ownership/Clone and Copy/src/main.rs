fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();    // Clone

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;                // Copy

    println!("x = {}, y = {}", x, y);
}
