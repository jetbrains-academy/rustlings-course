use returning_result::generate_nametag_text;

#[test]
fn generates_nametag_text_for_a_nonempty_name() {
    let str: String = format!("{:?}", generate_nametag_text("Beyoncé".into()));
    assert!(str.starts_with("Ok"), "The `generate_nametag_text` function should return `Result` type");
    assert!(str.contains("Hi! My name is Beyoncé"), "The nametag text is wrong");
}

#[test]
fn explains_why_generating_nametag_text_fails() {
    let str: String = format!("{:?}", generate_nametag_text("".into()));
    assert!(str.starts_with("Err"), "The `generate_nametag_text` function should return `Result` type");
    assert!(str.contains("`name` was empty; it must be nonempty."), "Another error message is expected");
}
