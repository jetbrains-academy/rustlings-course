use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

// This is the custom error type that we will be using for the parser for
// `Climate`.
#[derive(Debug, PartialEq)]
pub enum ParseClimateError {
    Empty,
    BadLen,
    NoCity,
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
}

// This `From` implementation allows the `?` operator to work on
// `ParseIntError` values.
impl From<ParseIntError> for ParseClimateError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}

// This `From` implementation allows the `?` operator to work on
// `ParseFloatError` values.
impl From<ParseFloatError> for ParseClimateError {
    fn from(e: ParseFloatError) -> Self {
        Self::ParseFloat(e)
    }
}

impl Error for ParseClimateError {}

// The `Display` trait allows for other code to obtain the error formatted
// as a user-visible string.
impl Display for ParseClimateError {
    // TODO: Complete this function so that it produces the correct strings
    // for each error variant.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Imports the variants to make the following code more compact.
        use ParseClimateError::*;
        match self {
            NoCity => write!(f, "no city name"),
            ParseFloat(e) => write!(f, "error parsing temperature: {}", e),
            Empty => write!(f, "empty input"),
            BadLen => write!(f, "incorrect number of fields"),
            ParseInt(e) => write!(f, "error parsing year: {}", e),
            _ => write!(f, "unhandled error!"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Climate {
    pub city: String,
    pub year: u32,
    pub temp: f32,
}

// Parser for `Climate`.
impl FromStr for Climate {
    type Err = ParseClimateError;
    // TODO: Complete this function by making it handle the missing error cases.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<_> = s.split(',').collect();
        let (city, year, temp) = match &v[..] {
            [city, year, temp] => (city.to_string(), year, temp),
            // ????
            _ => return Err(ParseClimateError::BadLen),
        };
        let year: u32 = year.parse()?;
        let temp: f32 = temp.parse()?;
        Ok(Climate { city, year, temp })
    }
}




