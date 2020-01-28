extern crate escargot;

#[test]
fn if_compiles() {
    //Mocking test to run only the compiler's checker
    let actual = escargot::CargoBuild::new()
        .bin("slices")
        .run()
        .unwrap()
        .command()
        .output()
        .unwrap()
        .stdout;
    //TODO: recover from an incorrect output
    let actual_as_string = std::str::from_utf8(&actual).unwrap();
    let expected_true  = "Nice slice!";
    assert!(actual_as_string.contains(expected_true), "The output should be \"Nice slice!\". Check, if the statement returns true.");
}






