use loop_over_vector_with_iterators::vec_map;

#[test]
fn test_vec_map() {
    let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
    let ans = vec_map(&v);

    assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
}