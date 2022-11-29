pub fn maybe_ice_cream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a value of 0
    // The Option output should gracefully handle cases where time_of_day > 23.
    if time_of_day < 22 {
        Some(5)
    } else if time_of_day < 24 {
        Some(0)
    } else {
        None
    }
}

#[test]
fn raw_value() {
    let ice_creams = maybe_ice_cream(12);
    assert_eq!(ice_creams.unwrap(), 5);
}