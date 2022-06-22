use loop_over_a_vector::vec_loop;

fn main() {
    let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
    let ans = vec_loop(v.clone());
    println!("Vector: {:?}", ans);
}





