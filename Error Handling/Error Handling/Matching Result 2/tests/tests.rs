#[test]
fn if_compiles() {
    // Mocking test to run only the compiler's checker.
    assert!(true);
}

#[test]
fn calculates_remaining_tokens() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_matching_result_2"))
        .output()
        .expect("failed to run the compiled task binary");

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr).trim()
    );

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("You now have 59 tokens."),
        "The output is incorrect: expected `You now have 59 tokens.`, got `{}`.",
        stdout.trim()
    );
}
