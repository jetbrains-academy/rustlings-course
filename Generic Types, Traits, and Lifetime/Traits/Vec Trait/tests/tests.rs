use vec_trait::*;

#[test]
fn is_vec_pop_eq_bar() {
    let mut foo = vec![String::from("Foo")].append_bar();
    assert_eq!(foo.pop().unwrap(), String::from("Bar"));
    assert_eq!(foo.pop().unwrap(), String::from("Foo"));
}