use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: usize,
}

#[derive(Debug, PartialEq)]
pub enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.len() == 0 {
            // return Err(String::from("empty string cannot be parsed into Person"));
            return Err(ParsePersonError::Empty);
        }
        let parts: Vec<&str> = s.split(',').collect();
        if (parts.len() < 2) | (parts.len() > 2) {
            // return Err(String::from("must contain 2 fields exactly: name and age"));
            return Err(ParsePersonError::BadLen);
        }
        let name = if parts[0] != "" {
            String::from(parts[0])
        } else {
            // return Err(String::from("name field must not be empty"));
            return Err(ParsePersonError::NoName);
        };
        return if let Ok(age) = parts[1].parse::<usize>() {
            Ok(Person { name, age })
        } else {
            // Err(ParsePersonError::ParseInt());
            return Err(ParsePersonError :: ParseInt(_))
        };
    }
}