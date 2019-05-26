use if1::bigger;

#[test]
fn ten_is_bigger_than_eight() {
    assert_eq!(10, bigger(10, 8));
}

#[test]
fn fortytwo_is_bigger_than_thirtytwo() {
    assert_eq!(42, bigger(32, 42));
}
