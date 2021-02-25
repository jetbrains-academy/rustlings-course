fn main() {
    let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
    let ans = vec_loop(v.clone());
    println!("Vector: {:?}", ans);
}

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for i in v.iter_mut() {
        *i *= 2;
    }
        // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}



