extern crate escargot;

use std::panic::{PanicInfo};
use std::fmt;

#[test]
fn prints_hello_world_and_starts_new_line() {
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual = escargot::CargoBuild::new()
        .bin("hello_world")
        .run()
        .unwrap()
        .command()
        .output()
        .unwrap()
        .stdout;
    //TODO: recover from an incorrect output
    let actual_as_string = std::str::from_utf8(&actual).unwrap();
    let expected  = "Hello, world!\n";
    let err_print_expected = "Hello, world!";
    assert_ne!(actual_as_string, err_print_expected, "Note, that you should use println! macro, not print!");
    assert_eq!(actual_as_string, expected, "The output was incorrect:");
}






