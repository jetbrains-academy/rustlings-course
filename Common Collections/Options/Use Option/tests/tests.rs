use use_option::maybe_ice_cream;

#[test]
fn check_ice_cream() {
    assert_eq!(maybe_ice_cream(9), Some(5));
    assert_eq!(maybe_ice_cream(10), Some(5));
    assert_eq!(maybe_ice_cream(23), Some(0));
    assert_eq!(maybe_ice_cream(22), Some(0));
    assert_eq!(maybe_ice_cream(25), None);
}

