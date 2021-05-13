use std::error;
use std::str::FromStr;

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: usize,
}

impl FromStr for Person {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Person, Self::Err> {

        if s.len() == 0 {
            return Err(Box::new("empty string cannot be parsed into Person"));
            // return Err(Box::new(String::from("empty string cannot be parsed into Person")));
        }
        let parts: Vec<&str> = s.split(',').collect();
        if (parts.len() < 2) | (parts.len() > 2) {
            return Err(Box::new("must contain 2 fields exactly: name and age"));
            //return Err(Box::new(String::from("must contain 2 fields exactly: name and age")));
        }
        let name = if parts[0] != "" {
            String::from(parts[0])
        } else {
            return Err(Box::new("name field must not be empty"));
            //return Err(Box::new(String::from("name field must not be empty")));
        };
        if let Ok(age) = parts[1].parse::<usize>() {
            return Ok(Person {name, age});
        } else {
            return Err(Box::new("cannot parse age"));
            //return Err(Box::new(String::from("cannot parse age")));
        };
    }
}


