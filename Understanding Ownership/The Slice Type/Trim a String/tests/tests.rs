use trim_a_string::trimmed_space;

// TODO: replace this with an actual test
#[test]
fn test() {
    assert_eq!(trimmed_space("   aaa   "), "aaa", "It doesn't seem to trim space");
    assert_eq!(trimmed_space("   a a   "), "a a", "It should keep the inner spaces");
    assert_eq!(trimmed_space("aaa"), "aaa", "It seems to trim something wrong");
    assert_eq!(trimmed_space(&String::from("   Hello!")[..]), "Hello!", "It should work for String slices");
}
