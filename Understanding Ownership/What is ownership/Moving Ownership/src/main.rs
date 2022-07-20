fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // !!! ERROR: value is moved to s2
    println!("{}, world!", s1);
}
