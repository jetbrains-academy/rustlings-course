extern crate escargot;

#[test]
fn compiles_and_tests() {
    let tests = escargot::CargoBuild::new()
        .tests()
        .run_tests().unwrap();


    for tst in tests.flatten() {
        if tst.kind() == "lib" {
            if tst.exec().is_ok() {
                return
            } else {
                panic!("You were supposed to make the test compile!")
            }
        }
    }

    panic!("You were supposed to make the test compile!")
}





