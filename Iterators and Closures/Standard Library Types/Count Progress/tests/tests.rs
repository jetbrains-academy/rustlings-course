use count_progress::*;
use std::collections::HashMap;

#[test]
fn count_complete() {
    let map = get_map();
    assert_eq!(3, count(&map, Progress::Complete));
}

#[test]
fn count_equals_for() {
    let map = get_map();
    assert_eq!(
        count_for(&map, Progress::Complete),
        count(&map, Progress::Complete)
    );
}

#[test]
fn count_stack_complete() {
    let stack = get_map_stack();
    assert_eq!(6, count_stack(&stack, Progress::Complete));
}

#[test]
fn count_stack_equals_for() {
    let stack = get_map_stack();
    assert_eq!(
        count_stack_for(&stack, Progress::Complete),
        count_stack(&stack, Progress::Complete)
    );
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