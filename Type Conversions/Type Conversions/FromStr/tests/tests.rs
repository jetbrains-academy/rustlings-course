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
    "John,".parse::<Person>().unwrap();
}

#[test]
#[should_panic]
fn invalid_age() {
    "John,twenty".parse::<Person>().unwrap();
}

#[test]
#[should_panic]
fn missing_comma_and_age() {
    "John".parse::<Person>().unwrap();
}

#[test]
#[should_panic]
fn missing_name() {
    ",1".parse::<Person>().unwrap();
}

#[test]
#[should_panic]
fn missing_name_and_age() {
    ",".parse::<Person>().unwrap();
}

#[test]
#[should_panic]
fn missing_name_and_invalid_age() {
    ",one".parse::<Person>().unwrap();
}

#[test]
#[should_panic]
fn trailing_comma() {
    "John,32,".parse::<Person>().unwrap();
}

#[test]
#[should_panic]
fn trailing_comma_and_some_string() {
    "John,32,man".parse::<Person>().unwrap();
}
