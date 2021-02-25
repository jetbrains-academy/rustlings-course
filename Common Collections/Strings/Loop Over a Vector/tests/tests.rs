use loop_over_a_vector::vec_loop;

#[test]
fn test_vec_loop() {
    let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
    let ans = vec_loop(v.clone());

    assert_eq!(
        ans,
        v.iter()
            .map(|x| x * 2)
            .collect::<Vec<i32>>()
    );
}