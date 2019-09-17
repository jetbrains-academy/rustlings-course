pub fn factorial(num: u64) -> u64 {
    if num == 1 {
        return 1;
    }
    else {
        num*factorial(num-1)
    }
}
