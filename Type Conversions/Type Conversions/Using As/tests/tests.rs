use using_as::average;

#[test]
fn returns_proper_type_and_value() {
    assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
}
