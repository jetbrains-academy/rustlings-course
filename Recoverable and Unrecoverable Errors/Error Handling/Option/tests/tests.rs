#[test]
fn prints_the_vector_from_fill_vec() {
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual = escargot::CargoBuild::new()
        .bin("option")
        .run()
        .unwrap()
        .command()
        .output()
        .unwrap()
        .stdout;
    //TODO: recover from an incorrect output
    let actual_as_string = std::str::from_utf8(&actual).unwrap();
    let expected_first_line  = "The last item in the list is 3\n";
    let expected_second_line = "The second-to-last item in the list is 3\n";
    assert!(actual_as_string.contains(expected_first_line), "The output is missing the last element of the vec");
    assert!(actual_as_string.contains(expected_second_line), "The output is missing the second-to-last element. Seems like there was a panic!");
}





