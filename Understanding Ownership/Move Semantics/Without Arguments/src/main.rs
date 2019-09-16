fn main() {
    //This line should be deleted

    let mut vec1 = fill_vec(/*this argument should be delted*/);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` no longer take `vec: Vec<i32>` as argument
fn fill_vec() -> Vec<i32> {
    let vec0 = Vec::new();

    let mut vec = vec0;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
