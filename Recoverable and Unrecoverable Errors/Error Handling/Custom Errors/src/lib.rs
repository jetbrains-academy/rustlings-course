use std::num::ParseIntError;

// This is a custom error type that we will be using in `parse_pos_nonzero()`.
#[derive(PartialEq, Debug)]
pub enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError)
}

impl ParsePosNonzeroError {
    pub fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }
    // Add another error conversion function here.
    pub fn from_parse_int(err: ParseIntError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::ParseInt(err)
    }
}

pub fn parse_pos_nonzero(s: &str)
                     -> Result<PositiveNonzeroInteger, ParsePosNonzeroError>
{
    // Change this to return an appropriate error instead of panicking
    // when `parse()` returns an error.
    let x: i64 = s.parse().unwrap();
    PositiveNonzeroInteger::new(x)
        .map_err(ParsePosNonzeroError::from_creation)
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
pub struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
pub enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    pub fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64))
        }
    }
}