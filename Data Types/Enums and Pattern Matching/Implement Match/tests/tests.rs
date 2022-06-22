use implement_match::test_match_message_call;

#[test]
fn test_match() {
    let state = test_match_message_call();
    assert_eq!(state.color, (255, 0, 255));
    assert_eq!(state.position.x, 10);
    assert_eq!(state.position.y, 15);
    assert_eq!(state.quit, true);
}
