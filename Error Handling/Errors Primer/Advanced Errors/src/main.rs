use advanced_errors::*;
use std::str::FromStr;

fn main() {
    println!("{:?}", PositiveNonzeroInteger::from_str("not a number"));
    println!("{:?}", PositiveNonzeroInteger::from_str("-555"));
    println!("{:?}", PositiveNonzeroInteger::from_str("0"))
}
