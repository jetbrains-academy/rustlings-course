use declaring_a_vector::array_and_vec;

#[test]
fn if_compiles() {
    //Mocking test to run only the compiler's checker
    assert!(true);
}

#[test]
fn test_array_and_vec_similarity() {
    let (a, v) = array_and_vec();
    assert_eq!(a, v[..]);
}
