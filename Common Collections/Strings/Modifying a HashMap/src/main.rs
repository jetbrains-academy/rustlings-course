use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lichi,
    Pineapple,
}

fn main() {
    fn get_fruit_basket() -> HashMap<Fruit, u32> {
        let mut basket = HashMap::<Fruit, u32>::new();
        basket.insert(Fruit::Apple, 4);
        basket.insert(Fruit::Mango, 2);
        basket.insert(Fruit::Lichi, 5);

        basket
    }
    let mut basket = get_fruit_basket();
    let b = fruit_basket(&mut basket);
    println!("Basket: {:?}", b);

    //for (Fruit, value) in &basket {
      //  println!("{}: {}", Fruit, value);
    //}
}


fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
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

