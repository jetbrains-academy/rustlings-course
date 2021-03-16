use from_into::*;

#[test]
fn test_default() {
    // Test that the default person is 30 year old John
    let dp = Person::default();
    assert_eq!(dp.name, "John");
    assert_eq!(dp.age, 30);
}
#[test]
fn test_bad_convert() {
    // Test that John is returned when bad string is provided
    let p = Person::from("");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);
}
#[test]
fn test_good_convert() {
    // Test that "Mark,20" works
    let p = Person::from("Mark,20");
    assert_eq!(p.name, "Mark");
    assert_eq!(p.age, 20);
}
#[test]
fn test_bad_age() {
    // Test that "Mark,twenty" will return the default person due to an error in parsing age
    let p = Person::from("Mark,twenty");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);
}

#[test]
fn test_missing_comma_and_age() {
    let p: Person = Person::from("Mark");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);
}

#[test]
fn test_missing_age() {
    let p: Person = Person::from("Mark,");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);
}

#[test]
fn test_missing_name() {
    let p: Person = Person::from(",1");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);
}

#[test]
fn test_missing_name_and_age() {
    let p: Person = Person::from(",");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);
}

#[test]
fn test_missing_name_and_invalid_age() {
    let p: Person = Person::from(",one");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);
}

#[test]
fn test_trailing_comma() {
    let p: Person = Person::from("Mike,32,");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);
}

#[test]
fn test_trailing_comma_and_some_string() {
    let p: Person = Person::from("Mike,32,man");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);
}