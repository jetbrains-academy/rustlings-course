use string_transformer::{Command, transformer::transformer};

#[test]
fn it_works() {
    let output = transformer(vec![
        ("hello".into(), Command::Uppercase),
        (" all roads lead to rome! ".into(), Command::Trim),
        ("foo".into(), Command::Append(1)),
        ("bar".into(), Command::Append(5)),
    ]);
    assert_eq!(output[0], "HELLO");
    assert_eq!(output[1], "all roads lead to rome!");
    assert_eq!(output[2], "foobar");
    assert_eq!(output[3], "barbarbarbarbarbar");
}