use fromstr::*;

#[test]
fn empty_input() {
    assert!("".parse::<Person>().is_err());
}
#[test]
fn good_input() {
    let p = "John,32".parse::<Person>();
    assert!(p.is_ok());
    let p = p.unwrap();
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 32);
}
#[test]
#[should_panic]
fn missing_age() {
    assert!("John,".parse::<Person>().is_err());
}

#[test]
#[should_panic]
fn invalid_age() {
    assert!("John,twenty".parse::<Person>().is_err());
}

#[test]
#[should_panic]
fn missing_comma_and_age() {
    assert!("John".parse::<Person>().is_err());
}

#[test]
#[should_panic]
fn missing_name() {
    assert!(",1".parse::<Person>().is_err());
}

#[test]
#[should_panic]
fn missing_name_and_age() {
    assert!(",".parse::<Person>().is_err());
}

#[test]
#[should_panic]
fn missing_name_and_invalid_age() {
    assert!(",one".parse::<Person>().is_err());
}

#[test]
#[should_panic]
fn trailing_comma() {
    assert!("John,32,".parse::<Person>().is_err());
}

#[test]
#[should_panic]
fn trailing_comma_and_some_string() {
    assert!("John,32,man".parse::<Person>().is_err());
}
