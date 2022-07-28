use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
#[derive(Debug)]
pub enum Fruit {
    Apple,
    Banana,
    Mango,
    Lichi,
    Pineapple,
}

pub fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = vec![
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lichi,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
        basket.entry(Fruit::from(fruit)).or_insert(6);
    }
}