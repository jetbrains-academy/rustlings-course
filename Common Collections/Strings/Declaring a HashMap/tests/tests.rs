use declaring_a_hashmap::fruit_basket;

#[test]
fn at_least_three_types_of_fruits() {
    let basket = fruit_basket();
    assert!(basket.len() >= 3);
}

#[test]
fn at_least_five_fruits() {
    let basket = fruit_basket();
    assert!(basket
        .values()
        .sum::<u32>() >= 5);
}