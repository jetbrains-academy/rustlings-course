use iterators4::factorial;

#[test]
fn factorial_of_1() {
    assert_eq!(1, factorial(1));
}

#[test]
fn factorial_of_2() {
    assert_eq!(2, factorial(2));
}

#[test]
fn factorial_of_4() {
    assert_eq!(24, factorial(4));
}
