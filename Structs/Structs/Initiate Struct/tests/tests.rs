use initiate_struct::classic_c_structs;
use initiate_struct::tuple_structs;
use initiate_struct::unit_structs;

#[test]
fn test_classic_c_structs() {
    let green = classic_c_structs();

    assert_eq!(green.name, "green");
    assert_eq!(green.hex, "#00FF00");
}

#[test]
fn test_tuple_structs() {
    let green = tuple_structs();

    assert_eq!(green.0, "green");
    assert_eq!(green.1, "#00FF00");
}

#[test]
fn test_unit_structs() {
    let message = unit_structs();
    assert_eq!(message, "UnitStructs are fun!");
}
