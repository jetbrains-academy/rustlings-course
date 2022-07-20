#[test]
fn prints_hello1_twice() {
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual = escargot::CargoBuild::new()
        .bin("more_exclamations_3")
        .run()
        .unwrap()
        .command()
        .output()
        .unwrap()
        .stdout;
    //TODO: recover from an incorrect output
    let actual_as_string = std::str::from_utf8(&actual).unwrap();
    let expected_first_line  = "hello1 is `Hello!`\n";
    let expected_second_line = "hello1 is `Hello!!`\n";
    assert!(actual_as_string.contains(expected_first_line), "The output with one '!' is missing");
    assert!(actual_as_string.contains(expected_second_line), "The output with \"!!\" is missing");
}



