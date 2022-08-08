use std::collections::HashSet;
use std::iter::FromIterator;

#[test]
fn all_threads_are_joined() {
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual = escargot::CargoBuild::new()
        .bin("join_all_handles")
        .run()
        .unwrap()
        .command()
        .output()
        .unwrap()
        .stdout;
    //TODO: recover from an incorrect output

    let lines: HashSet<&str> = HashSet::from_iter(std::str::from_utf8(&actual).unwrap().lines());
    assert_eq!(10, lines.len(), "Not all the threads were joined");
}