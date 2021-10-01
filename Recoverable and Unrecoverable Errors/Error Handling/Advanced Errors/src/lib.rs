use std::num::ParseIntError;
use std::str::FromStr;

// This is a custom error type that we will be using in the `FromStr`
// implementation.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl From<CreationError> for ParsePosNonzeroError {
    fn from(e: CreationError) -> Self {
        // TODO: complete this implementation so that the `?` operator will
        // work for `CreationError`
    }
}

// TODO: implement another instance of the `From` trait here so that the
// `?` operator will work in the other place in the `FromStr`
// implementation below.

// Don't change anything below this line.

impl FromStr for PositiveNonzeroInteger {
    type Err = ParsePosNonzeroError;
    fn from_str(s: &str) -> Result<PositiveNonzeroInteger, Self::Err> {
        let x: i64 = s.parse()?;
        Ok(PositiveNonzeroInteger::new(x)?)
    }
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}
