use std::error;
use std::fmt;

// Don't change anything in this file
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

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}