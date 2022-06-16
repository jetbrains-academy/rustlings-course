use matching_result::total_cost;

#[test]
fn item_quantity_is_a_valid_number() {
    assert_eq!(total_cost("34"), Ok(171));
}

#[test]
fn item_quantity_is_an_invalid_number() {
    assert_eq!(
        total_cost("beep boop").unwrap_err().to_string(),
        "invalid digit found in string"
    );
}
