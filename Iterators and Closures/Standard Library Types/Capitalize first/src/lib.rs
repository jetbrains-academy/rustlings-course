pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// Step 2
pub fn iterate_string_vec() {
    let words = vec!["hello", "world"];
    let capitalized_words: Vec<String> = vec![capitalize_first(words[0]), capitalize_first(words[1])];
    println!("{:?}", capitalized_words);
}

// Step 3
pub fn iterate_into_string() {
    let words = vec!["hello", " ", "world"];
    let capitalized_words = words.into_iter().map(capitalize_first).collect::<Vec<String>>().join("");
    println!("{:?}", capitalized_words);
}


