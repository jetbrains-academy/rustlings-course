use declaring_a_vector::{create_array, create_vector};

#[test]
fn if_compiles() {
    //Mocking test to run only the compiler's checker
    assert!(true);
}

#[test]
fn test_array_and_vec_similarity() {
    let a = create_array();
    let v = create_vector();
    assert_eq!(a, v[..]);
}
