#[cfg(test)]
mod tests {
    use times_two_tests::times_two;

    #[test]
    fn returns_twice_of_positive_numbers() {
        assert_eq!(times_two(2), 4);
    }

    #[test]
    fn returns_twice_of_negative_numbers() {
        assert_eq!(times_two(-2), -4);
    }
}






