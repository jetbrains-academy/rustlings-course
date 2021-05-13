#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    pub dividend: i32,
    pub divisor: i32,
}

// Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        Err(DivisionError::DivideByZero)
    }
    else if a % b == 0 {
        Ok(a/b)
    }
    else {
        Err(DivisionError::NotDivisible(NotDivisibleError{ dividend: a, divisor: b }))
    }
}

// Complete the function and return a value of the correct type so the test passes.
// Desired output: Ok([1, 11, 1426, 3])
pub fn result_with_list() -> () {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
}

// Complete the function and return a value of the correct type so the test passes.
// Desired output: [Ok(1), Ok(11), Ok(1426), Ok(3)]
pub fn list_of_results() -> () {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     // Iterator exercises using your `divide` function
//
//     #[test]
//     fn result_with_list() {
//         let numbers = vec![27, 297, 38502, 81];
//         let division_results = numbers.into_iter().map(|n| divide(n, 27));
//         let x= division_results.collect::<Result<Vec<_>, _>>();
//         assert_eq!(format!("{:?}", x), "Ok([1, 11, 1426, 3])");
//     }
//     #[test]
//     fn list_of_results() {
//         let numbers = vec![27, 297, 38502, 81];
//         let division_results = numbers.into_iter().map(|n| divide(n, 27));
//         let x = division_results.collect::<Vec<Result<i32, _>>>();
//         assert_eq!(format!("{:?}", x), "[Ok(1), Ok(11), Ok(1426), Ok(3)]");
//     }
// }