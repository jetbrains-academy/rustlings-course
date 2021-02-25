use std::collections::HashMap;
use modifying_a_hashmap::*;

fn get_fruit_basket() -> HashMap<Fruit, u32> {
    let mut basket = HashMap::<Fruit, u32>::new();
    basket.insert(Fruit::Apple, 4);
    basket.insert(Fruit::Mango, 2);
    basket.insert(Fruit::Lichi, 5);

    basket
}

#[test]
fn test_given_fruits_are_not_modified() {
    let mut basket = get_fruit_basket();
    fruit_basket(&mut basket);
    assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
    assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
    assert_eq!(*basket.get(&Fruit::Lichi).unwrap(), 5);
}

#[test]
fn at_least_five_types_of_fruits() {
    let mut basket = get_fruit_basket();
    fruit_basket(&mut basket);
    let count_fruit_kinds = basket.len();
    assert!(count_fruit_kinds == 5);
}

#[test]
fn greater_than_eleven_fruits() {
    let mut basket = get_fruit_basket();
    fruit_basket(&mut basket);
    let count = basket
        .values()
        .sum::<u32>();
    assert!(count > 11);
}