use returning_result::generate_nametag_text;

#[test]
fn generates_nametag_text_for_a_nonempty_name() {
    assert_eq!(
        generate_nametag_text("Beyoncé".into()),
        Ok("Hi! My name is Beyoncé".into())
    );
}

#[test]
fn explains_why_generating_nametag_text_fails() {
    assert_eq!(
        generate_nametag_text("".into()),
        Err("`name` was empty; it must be nonempty.".into())
    );
}
