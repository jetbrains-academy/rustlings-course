#[test]
fn prints_the_second_number_is() {
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual = escargot::CargoBuild::new()
        .bin("show_type")
        .run()
        .unwrap()
        .command()
        .output()
        .unwrap()
        .stdout;
    //TODO: recover from an incorrect output
    let actual_as_string = std::str::from_utf8(&actual).unwrap();

    assert!(actual_as_string.contains("1 has i32 type"), "The type of `1` is different");
    assert!(actual_as_string.contains("1..4 has Range<usize> type"), "The type of `1..4` is different");
    assert!(actual_as_string.contains("nice_slice == [2, 3, 4] has bool type"), "The type of `nice_slice == [2, 3, 4]` is different");
}