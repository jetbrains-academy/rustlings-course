#[test]
fn prints_the_second_number_is() {
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual = escargot::CargoBuild::new()
        .bin("public_modifier_2")
        .run()
        .unwrap()
        .command()
        .output()
        .unwrap()
        .stdout;
    //TODO: recover from an incorrect output
    let actual_as_string = std::str::from_utf8(&actual).unwrap();
    let expected_output  = "favorite snacks: Pear and Cucumber\n";
    assert_eq!(actual_as_string, expected_output, "The output provided is not equal to the expected one.");
}




