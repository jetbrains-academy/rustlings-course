use if_task::bigger;

#[test]
fn ten_is_bigger_than_eight() {
    //Assert
    assert_eq!(10, bigger(10, 8), "Incorrect value was returned.");
}

#[test]
fn fortytwo_is_bigger_than_thirtytwo() {
    //Assert
    assert_eq!(42, bigger(32, 42), "Incorrect value was returned.");
}