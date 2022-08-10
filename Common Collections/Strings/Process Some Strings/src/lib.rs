pub fn trim_me(input: &str) -> String {
    input.trim().to_string()
}

pub fn compose_me(input: &str) -> String {
    format!("{}{}", input, " world!")
}

pub fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons")
}