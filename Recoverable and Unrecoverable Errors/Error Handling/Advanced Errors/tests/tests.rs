use advanced_errors::*;
use std::str::FromStr;

#[test]
fn test_parse_error() {
    // We can't construct a ParseIntError, so we have to pattern match.
    assert!(matches!(
            PositiveNonzeroInteger::from_str("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
}

#[test]
fn test_negative() {
    assert_eq!(
        PositiveNonzeroInteger::from_str("-555"),
        Err(ParsePosNonzeroError::Creation(CreationError::Negative))
    );
}

#[test]
fn test_zero() {
    assert_eq!(
        PositiveNonzeroInteger::from_str("0"),
        Err(ParsePosNonzeroError::Creation(CreationError::Zero))
    );
}

#[test]
fn test_positive() {
    let x = PositiveNonzeroInteger::new(42);
    assert!(x.is_ok());
    assert_eq!(PositiveNonzeroInteger::from_str("42"), Ok(x.unwrap()));
}