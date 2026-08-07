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
    assert!(actual_as_string.contains(expected_first_line), "The output 'Hello!' is missing");
    assert!(actual_as_string.contains(expected_second_line), "The output \"Hello!!\" is missing");
}

#[test]
fn creates_string_inside_hello_with_exclamation() {
    let source = include_str!("../src/main.rs");
    let source_without_whitespace: String = source.chars().filter(|ch| !ch.is_whitespace()).collect();

    assert!(
        source_without_whitespace.contains("hello_with_exclamation()"),
        "Call `hello_with_exclamation` without passing any arguments."
    );
    assert!(
        source_without_whitespace.contains("fnhello_with_exclamation()->String"),
        "`hello_with_exclamation` should not take any arguments — it should create the string itself."
    );
    assert!(
        !source_without_whitespace.contains("lethello="),
        "Don't create the `hello` string in `main` — create it inside `hello_with_exclamation` instead."
    );
}


