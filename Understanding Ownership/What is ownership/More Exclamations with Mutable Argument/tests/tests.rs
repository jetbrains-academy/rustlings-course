#[test]
fn prints_hello1_twice() {
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual = escargot::CargoBuild::new()
        .bin("more_exclamations_4")
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
    assert!(actual_as_string.contains(expected_first_line), "The output is missing the line with information about hello1 before adding '!'");
    assert!(actual_as_string.contains(expected_second_line), "The output is missing the line with information about hello1 after adding '!'");
}

#[test]
fn passes_hello_to_add_exclamation() {
    let source = include_str!("../src/main.rs");
    let source_without_whitespace: String = source.chars().filter(|ch| !ch.is_whitespace()).collect();

    assert!(
        source_without_whitespace.contains("add_exclamation(hello)"),
        "Use the `hello` variable in the call to `add_exclamation`."
    );
}


