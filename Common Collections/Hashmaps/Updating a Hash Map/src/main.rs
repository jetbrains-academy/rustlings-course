use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // Replacing a value stored with a particular key
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // Using the entry method to only insert if the key
    // does not already have a value
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Counting occurrences of words using a hash map
    // that stores words and counts
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
