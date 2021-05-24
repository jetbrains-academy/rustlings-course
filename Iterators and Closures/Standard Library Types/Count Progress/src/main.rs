use count_progress::*;
use std::collections::HashMap;

fn main() {
    let map = get_map();
    let count_for_result = count_for(&map, Progress::Complete);
    let count_result = count(&map, Progress::Complete);
    println!("{}", "Count 'Complete' exercises using count_for():");
    println!("{}", count_for_result);
    println!("{}", "Count 'Complete' exercises using count():");
    println!("{}", count_result);

    let stack = get_map_stack();
    let count_stack_for_result = count_stack_for(&stack, Progress::Complete);
    let count_stack_result = count_stack(&stack, Progress::Complete);
    println!("{}", "Count 'Complete' exercises using count_stack_for():");
    println!("{}", count_stack_for_result);
    println!("{}", "Count 'Complete' exercises using count_stack():");
    println!("{}", count_stack_result);
}

fn get_map() -> HashMap<String, Progress> {
    use Progress::*;

    let mut map = HashMap::new();
    map.insert(String::from("variables1"), Complete);
    map.insert(String::from("functions1"), Complete);
    map.insert(String::from("hashmap1"), Complete);
    map.insert(String::from("arc1"), Some);
    map.insert(String::from("as_ref_mut"), None);
    map.insert(String::from("from_str"), None);

    map
}

fn get_map_stack() -> Vec<HashMap<String, Progress>> {
    use Progress::*;

    let map = get_map();

    let mut other = HashMap::new();
    other.insert(String::from("variables2"), Complete);
    other.insert(String::from("functions2"), Complete);
    other.insert(String::from("if1"), Complete);
    other.insert(String::from("from_into"), None);
    other.insert(String::from("try_from_into"), None);

    vec![map, other]
}
