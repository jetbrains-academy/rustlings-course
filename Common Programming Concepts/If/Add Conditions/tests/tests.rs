use add_conditions::fizz_if_foo;

#[test]
fn foo_for_fizz() {
    assert_eq!(fizz_if_foo("fizz"), "foo")
}

#[test]
fn bar_for_fuzz() {
    assert_eq!(fizz_if_foo("fuzz"), "bar")
}

#[test]
fn default_to_baz() {
    assert_eq!(fizz_if_foo("literally anything"), "baz")
}