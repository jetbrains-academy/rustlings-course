use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

// Tuple implementation
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = String;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        match tuple {
            (r, g, b) if r < 0 || r > 255 || g < 0 || g > 255 || b < 0 || b > 255 => {
                Err(String::from("rgb color values must be integers in range from 0 to 255"))
            },
            (red, green, blue) => {
                Ok(Color {
                    red: red as u8,
                    green: green as u8,
                    blue: blue as u8,
                })
            }
        }
    }
}

// Array implementation
impl TryFrom<[i16; 3]> for Color {
    type Error = String;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        arr[..].try_into()
    }
}

// Slice implementation
impl TryFrom<&[i16]> for Color {
    type Error = String;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(String::from("the slice must have exactly 3 numbers for rgb"));
        }
        (slice[0], slice[1], slice[2]).try_into()
    }
}


