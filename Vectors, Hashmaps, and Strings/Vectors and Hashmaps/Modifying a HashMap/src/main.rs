use std::collections::HashMap;
use modifying_a_hashmap::*;
use std::collections::hash_map::RandomState;


fn main() {
    fn get_fruit_basket() -> HashMap<modifying_a_hashmap::Fruit, u32, RandomState> {
        let mut basket = HashMap::<modifying_a_hashmap::Fruit, u32>::new();
        basket.insert(modifying_a_hashmap::Fruit::Apple, 4);
        basket.insert(modifying_a_hashmap::Fruit::Mango, 2);
        basket.insert(modifying_a_hashmap::Fruit::Lichi, 5);

        basket
    }
    let mut basket = get_fruit_basket();
    fruit_basket(&mut basket);
    println!("Basket: {:?}", basket);

}



