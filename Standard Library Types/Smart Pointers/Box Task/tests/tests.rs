use box_task::*;

#[test]
fn test_create_empty_list() {
    assert_eq!(List::Nil, create_empty_list())
}

#[test]
fn test_create_non_empty_list() {
    assert_ne!(create_empty_list(), create_non_empty_list())
}
