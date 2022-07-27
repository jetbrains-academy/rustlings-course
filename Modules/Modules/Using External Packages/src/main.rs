use rand::Rng;
use std::collections::HashMap;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number: {}", secret_number)
}
