use string_trait::*;

#[test]
fn is_foo_bar() {
    assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
}

#[test]
fn is_bar_bar() {
    assert_eq!(
        String::from("").append_bar().append_bar(),
        String::from("BarBar")
    );
}