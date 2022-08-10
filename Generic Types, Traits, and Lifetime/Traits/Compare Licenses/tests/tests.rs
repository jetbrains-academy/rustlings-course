use compare_licenses::*;

#[test]
fn compare_license_information() {
    let some_software = SomeSoftware {};
    let other_software = OtherSoftware {};

    assert!(compare_license_types(some_software, other_software));
}

#[test]
fn compare_license_information_backwards() {
    let some_software = SomeSoftware {};
    let other_software = OtherSoftware {};

    assert!(compare_license_types(other_software, some_software));
}