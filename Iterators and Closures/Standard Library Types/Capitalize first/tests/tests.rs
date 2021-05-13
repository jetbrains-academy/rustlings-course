use capitalize_first::*;

#[test]
fn test_success() {
    assert_eq!(capitalize_first("hello"), "Hello");
}

#[test]
fn test_empty() {
    assert_eq!(capitalize_first(""), "");
}

// Step 2.
#[test]
fn test_iterate_string_vec() {
    let words = vec!["hello", "world"];
    assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    // let capitalized_words: Vec<String> = vec![capitalize_first(words[0]), capitalize_first(words[1])];
    //     assert_eq!(capitalized_words, ["Hello", "World"]);
}

#[test]
fn test_iterate_into_string() {
    let words = vec!["hello", " ", "world"];
    assert_eq!(capitalize_words_string(&words), "Hello World");
    // let capitalized_words = words.into_iter().map(capitalize_first).collect::<Vec<String>>().join("");
    //     assert_eq!(capitalized_words, "Hello World");
}