use std::error;
use std::fmt;
use std::io;

// PositiveNonzeroInteger is a struct defined below the tests.
pub fn read_and_validate(b: &mut dyn io::BufRead) -> Result<PositiveNonzeroInteger, Box<dyn error::Error>> {
    let mut line = String::new();
    b.read_line(&mut line)?;
    let num: i64 = line.trim().parse()?;
    let answer = PositiveNonzeroInteger::new(num)?;
    Ok(answer)
}

// This is a test helper function that turns a &str into a BufReader.
pub fn test_with_str(s: &str) -> Result<PositiveNonzeroInteger, Box<dyn error::Error>> {
    let mut b = io::BufReader::new(s.as_bytes());
    read_and_validate(&mut b)
}

#[derive(PartialEq, Debug)]
pub struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    pub fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value == 0 {
            Err(CreationError::Zero)
        } else if value < 0 {
            Err(CreationError::Negative)
        } else {
            Ok(PositiveNonzeroInteger(value as u64))
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum CreationError {
    Negative,
    Zero,
}


impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "Number is negative",
            CreationError::Zero => "Number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}


//impl fmt::Display for CreationError {
   // fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //    f.write_str((self as &error::Error).description())
    //}
//}

//impl error::Error for CreationError {
  //  fn description(&self) -> &str {
    //    match *self {
      //      CreationError::Negative => "Negative",
        //    CreationError::Zero => "Zero",
        //}
    //}
//}