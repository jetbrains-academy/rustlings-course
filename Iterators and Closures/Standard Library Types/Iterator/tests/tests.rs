//use iterator3::{divide, DivisionError, NotDivisibleError};
use iterator3::*;

#[test]
fn test_success() {
    assert_eq!(divide(81, 9), Ok(9));
}

#[test]
fn test_not_divisible() {
    assert_eq!(
        divide(81, 6),
        Err(DivisionError::NotDivisible(NotDivisibleError {
            dividend: 81,
            divisor: 6
        }))
    );
}

#[test]
fn test_divide_by_0() {
    assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
}

#[test]
fn test_divide_0_by_something() {
    assert_eq!(divide(0, 81), Ok(0));
}

#[test]
fn test_result_with_list() {
    assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
}

#[test]
fn test_list_of_results() {
    assert_eq!(
        format!("{:?}", list_of_results()),
        "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
    );
}
