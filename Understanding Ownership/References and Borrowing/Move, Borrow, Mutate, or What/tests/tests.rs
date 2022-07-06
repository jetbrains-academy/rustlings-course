#[test]
fn prints_hello_twice() {
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual = escargot::CargoBuild::new()
        .bin("move_mutate_or_what")
        .run()
        .unwrap()
        .command()
        .output()
        .unwrap()
        .stdout;
    //TODO: recover from an incorrect output
    let actual_as_string = std::str::from_utf8(&actual).unwrap();
    let expected_first_line  = "Last char is '!'\n";
    let expected_second_line = "The message is \"RUST IS GREAT!\"\n";
    assert!(actual_as_string.contains(expected_first_line), "The output is missing the last character info");
    assert!(actual_as_string.contains(expected_second_line), "The output is missing the line with the message");
}



