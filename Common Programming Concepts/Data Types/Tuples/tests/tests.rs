extern crate escargot;

use std::fmt;

#[test]
fn prints_is_years_old() {
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual = escargot::CargoBuild::new()
        .bin("tuples")
        .run()
        .unwrap()
        .command()
        .output()
        .unwrap()
        .stdout;
    //TODO: recover from an incorrect output
    let actual_as_string = std::str::from_utf8(&actual).unwrap();
    let expected_is  = "is";
    let expected_years = "years";
    let expected_old = "old";
    assert!(actual_as_string.contains(expected_is), "The \"{} is {} years old\" line is missing from the output ");
    assert!(actual_as_string.contains(expected_old), "The \"{} is {} years old\" line is missing from the output ");
    assert!(actual_as_string.contains(expected_years), "The \"{} is {} years old\" line is missing from the output ");
}





