#[test]
fn if_compiles() {
    // Mocking test to run only the compiler's checker.
    assert!(true);
}

#[test]
fn main_assertion_passes() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_implement_a_macro"))
        .output()
        .expect("failed to run the compiled task binary");

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr).trim()
    );
}
