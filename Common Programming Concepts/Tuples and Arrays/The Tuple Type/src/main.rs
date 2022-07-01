fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
    println!("The last element is: {}", tup.2);
}
