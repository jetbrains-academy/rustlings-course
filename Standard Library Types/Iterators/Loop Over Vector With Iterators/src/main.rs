use loop_over_vector_with_iterators::vec_map;

fn main () {
    let v: Vec<i32> = vec![1,2,3,4,5];
    let ans = vec_map(&v);
    println!("{:?}", ans);
}