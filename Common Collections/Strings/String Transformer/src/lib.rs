pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

pub mod transformer {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output = vec![];
        for (string, command) in input {
            output.push(
            match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(size) =>
                    format!("{}{}", string, "bar".repeat(size).to_string())
            })
        }
        output
    }
}