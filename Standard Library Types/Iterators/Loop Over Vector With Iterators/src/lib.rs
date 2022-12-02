pub fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|num| {
        num * 2
    }).collect()
}