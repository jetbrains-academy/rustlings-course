#[test]
fn prints_the_second_number_is() {
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual = escargot::CargoBuild::new()
        .bin("strings_and_strs")
        .run()
        .unwrap()
        .command()
        .output()
        .unwrap()
        .stdout;
    //TODO: recover from an incorrect output
    let actual_as_string = std::str::from_utf8(&actual).unwrap();
    let expected_blue = "blue";
    let expected_red = "red";
    let expected_hi = "hi";
    let expected_rust_is_fun = "rust is fun";
    let expected_nice_weather = "nice weather";
    let expected_interpolation_station = "Interpolation Station";
    let expected_a = "a";
    let expected_hello_there = "hello there";
    let expected_happy_monday = "Happy Tuesday!";
    let expected_my_shift_key_is_sticky = "my shift key is sticky";

    assert!(actual_as_string.contains(expected_blue), "The output is missing the line \"blue\"");
    assert!(actual_as_string.contains(expected_red), "The output is missing the line \"red\"");
    assert!(actual_as_string.contains(expected_hi), "The output is missing the line \"hi\"");
    assert!(actual_as_string.contains(expected_rust_is_fun), "The output is missing the line \"rust_is_fun\"");
    assert!(actual_as_string.contains(expected_nice_weather), "The output is missing the line \"nice weather\"");
    assert!(actual_as_string.contains(expected_interpolation_station), "The output is missing the line \"Interpolation Station\"");
    assert!(actual_as_string.contains(expected_a), "The output is missing the line \"a\"");
    assert!(actual_as_string.contains(expected_hello_there), "The output is missing the line \"hello there\"");
    assert!(actual_as_string.contains(expected_happy_monday), "The output is missing the line \"Happy Monday!\"");
    assert!(actual_as_string.contains(expected_my_shift_key_is_sticky), "The output is missing the line \"my shift key is sticky\"");
}