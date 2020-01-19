pub fn factorial(num: u64) -> u64 {
    (1..=num).fold(1, |acc, x| acc * x)
}
