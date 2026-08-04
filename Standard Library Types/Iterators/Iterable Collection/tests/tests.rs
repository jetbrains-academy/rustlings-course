#[test]
fn if_compiles() {
    // A mock test only to run the compiler's checker.
    assert!(true);
}

#[test]
fn main_assertions_pass() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_iterable_collection"))
        .output()
        .expect("failed to run the compiled task binary");

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr).trim()
    );
}
