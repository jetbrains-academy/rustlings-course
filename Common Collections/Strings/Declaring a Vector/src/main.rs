fn main() {
    //println! ("{}", array_and_vec())
    let (a, v) = array_and_vec();
    println!("Array: {:?}", a);
    println!("Vector: {:?}", v);
}

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    let v = a.to_vec();

    (a, v)
}

//fn array_and_vec() {
//}
