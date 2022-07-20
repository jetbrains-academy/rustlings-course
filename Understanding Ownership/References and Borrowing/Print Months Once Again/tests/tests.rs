#[test]
fn prints_hello_twice() {
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual = escargot::CargoBuild::new()
        .bin("print_months_once_again")
        .run()
        .unwrap()
        .command()
        .output()
        .unwrap()
        .stdout;
    //TODO: recover from an incorrect output
    let actual_as_string = std::str::from_utf8(&actual).unwrap();
    let direct_order = "January February March April May June July August September October November December";
    let reversed_order = "December November October September August July June May April March February January";
    assert_eq!(actual_as_string.matches(direct_order).count(), 1, "We expect months to be printed in a direct order only once");
    assert_eq!(actual_as_string.matches(reversed_order).count(), 2, "We expect months to be printed in a reversed order twice");
}



