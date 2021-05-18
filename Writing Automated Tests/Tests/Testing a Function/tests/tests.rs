#[cfg(test)]
mod tests {
    use testing_task::is_even;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(2));
    }

    #[test]
    fn is_false_when_odd() {
        assert_eq!(is_even(5), false);
    }
}






