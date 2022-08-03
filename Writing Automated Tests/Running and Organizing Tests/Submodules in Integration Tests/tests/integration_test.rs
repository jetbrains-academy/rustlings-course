mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, submodules::add_two(2));
}