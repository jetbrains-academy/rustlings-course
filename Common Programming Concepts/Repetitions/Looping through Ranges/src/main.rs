fn main() {
    for number in 1..5 { // 5 is excluded
        println!("{}", number);
    }

    for number in 1..=5 { // 5 is included (note '=')
        println!("{}", number);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}