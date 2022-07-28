use std::collections::HashMap;

fn main() {
    // Creating a new hash map and inserting some keys and values
    let mut scores1 = HashMap::new();

    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Yellow"), 50);

    println!("{:?}", scores1);

    // Creating a hash map from a list of teams and a list of scores
    let teams = vec![String::from("Blue"),
                                 String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("{:?}", scores2);
}
