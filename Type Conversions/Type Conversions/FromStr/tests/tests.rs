use fromstr::*;

#[test]
fn empty_input() {
    assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
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
fn missing_age() {
    assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
}

#[test]
fn invalid_age() {
    assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
}

#[test]
fn missing_comma_and_age() {
    assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
}

#[test]
fn missing_name() {
    assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
}

#[test]
fn missing_name_and_age() {
    assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName)
        ));
}

#[test]
fn missing_name_and_invalid_age() {
    assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName)
        ));
}

#[test]
fn trailing_comma() {
    assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
}

#[test]
fn trailing_comma_and_some_string() {
    assert_eq!(
        "John,32,man".parse::<Person>(),
        Err(ParsePersonError::BadLen)
    );
}