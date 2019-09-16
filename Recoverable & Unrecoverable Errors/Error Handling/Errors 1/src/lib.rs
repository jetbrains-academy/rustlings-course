use std::error::Error;

pub fn generate_nametag_text(name: String) -> Result<String, &'static str> {
    if name.len() > 0 {
        Ok(format!("Hi! My name is {}", name))
    } else {
        // The error message should be: "`name` was empty; it must be nonempty."
        Err("`name` was empty; it must be nonempty.")
    }
}
