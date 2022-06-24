extern crate escargot;

use std::fmt;

#[test]
fn prints_the_second_number_is() {
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual = escargot::CargoBuild::new()
        .bin("tuple_index")
        .run()
        .unwrap()
        .command()
        .output()
        .unwrap()
        .stdout;
    //TODO: recover from an incorrect output
    let actual_as_string = std::str::from_utf8(&actual).unwrap();
    let expected_the_second_number_is  = "The second number is ";
    let expected_number = "2";
    let expected_number_err_1 = "1";
    let expected_number_err_3 = "3";
    assert!(actual_as_string.contains(expected_the_second_number_is), "\"The second number is\" line is missing from the output.");
    assert!(!actual_as_string.contains(expected_number_err_1), "The number is incorrect, seems you have printed the first element.");
    assert!(!actual_as_string.contains(expected_number_err_3), "The number is incorrect, seems you have printed the third element.");
    assert!(actual_as_string.contains(expected_number), "The expected second element of the tuple is missing from the output ");
}





