use license_it::*;

#[test]
fn is_licensing_info_the_same() {
    let licensing_info = String::from("Some information");
    let some_software = SomeSoftware { version_number: 1 };
    let other_software = OtherSoftware {
        version_number: "v2.0.0".to_string(),
    };
    assert_eq!(some_software.licensing_info(), licensing_info);
    assert_eq!(other_software.licensing_info(), licensing_info);
}