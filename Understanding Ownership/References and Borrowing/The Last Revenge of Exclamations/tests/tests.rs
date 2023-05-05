#[test]
fn prints_hello_twice() {
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual = escargot::CargoBuild::new()
        .bin("more_exclamations_6")
        .run()
        .unwrap()
        .command()
        .output()
        .unwrap()
        .stdout;
    //TODO: recover from an incorrect output
    let actual_as_string = std::str::from_utf8(&actual).unwrap().trim().to_string();
    let lines: Vec<&str> = actual_as_string.split("\n").collect();
    let expected_first_line = "hello is `Hello!`";
    let expected_second_line = "hello is `Hello!!`";
    assert!(
        lines.len() >= 2,
        "Make sure that the program prints at least two lines"
    );
    assert_eq!(
        expected_first_line, lines[0],
        "\nThe output is missing the first line with information about hello with '!' /
        \nExpected: {}\nActual: {}\n",
        expected_first_line, lines[0]
    );
    assert_eq!(
        expected_second_line, lines[1],
        "\nThe output is missing the second line with information about hello with '!!' /
        \nExpected: {}\nActual: {}\n",
        expected_second_line, lines[1]
    );
}
