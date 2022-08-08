#[test]
fn all_threads_are_joined() {
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual = escargot::CargoBuild::new()
        .bin("sum_all_numbers")
        .run()
        .unwrap()
        .command()
        .output()
        .unwrap()
        .stdout;
    //TODO: recover from an incorrect output

    let lines: Vec<&str> = std::str::from_utf8(&actual).unwrap().lines().rev().take(2).collect();
    assert_eq!(lines[0], "sum received: 55", "Sum is wrong, should be 55");
    assert_eq!(lines[1], "total numbers received: 10", "Total count is wrong, should be 10");
}