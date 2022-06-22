use arrays::arr;

#[test]
fn if_compiles() {
    //Mocking test to run only the compiler's checker
    assert!(true);
}

#[test]
fn test_arr() {
    assert_eq!(arr(), "Wow, that's a big array!")
}