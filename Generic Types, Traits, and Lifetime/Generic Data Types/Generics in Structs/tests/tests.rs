use generics_in_structs::*;

#[test]
fn store_u32_in_wrapper() {
    assert_eq!(Wrapper::new(42).value, 42);
}

#[test]
fn store_str_in_wrapper() {
    assert_eq!(Wrapper::new("Foo").value, "Foo");
}