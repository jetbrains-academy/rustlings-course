use try_from_into::*;
use std::convert::{TryFrom, TryInto};

#[test]
fn test_tuple_out_of_range_positive() {
    assert!(Color::try_from((256, 1000, 10000)).is_err());
}
#[test]
fn test_tuple_out_of_range_negative() {
    assert!(Color::try_from((-1, -10, -256)).is_err());
}
#[test]
fn test_tuple_sum() {
    assert!(Color::try_from((-1, 255, 255)).is_err());
}
#[test]
fn test_tuple_correct() {
    let c: Result<Color, String> = (183, 65, 14).try_into();
    assert_eq!(
        c,
        Ok(Color {
            red: 183,
            green: 65,
            blue: 14
        })
    );
}
#[test]
fn test_array_out_of_range_positive() {
    let c: Result<Color, String> = [1000, 10000, 256].try_into();
    assert!(c.is_err());
}
#[test]
fn test_array_out_of_range_negative() {
    let c: Result<Color, String> = [-10, -256, -1].try_into();
    assert!(c.is_err());
}
#[test]
fn test_array_sum() {
    let c: Result<Color, String> = [-1, 255, 255].try_into();
    assert!(c.is_err());
}
#[test]
fn test_array_correct() {
    let c: Result<Color, String> = [183, 65, 14].try_into();
    assert_eq!(
        c,
        Ok(Color {
            red: 183,
            green: 65,
            blue: 14
        })
    );
}
#[test]
fn test_slice_out_of_range_positive() {
    let arr = [10000, 256, 1000];
    assert!(Color::try_from(&arr[..]).is_err());
}
#[test]
fn test_slice_out_of_range_negative() {
    let arr = [-256, -1, -10];
    assert!(Color::try_from(&arr[..]).is_err());
}
#[test]
fn test_slice_sum() {
    let arr = [-1, 255, 255];
    assert!(Color::try_from(&arr[..]).is_err());
}
#[test]
fn test_slice_correct() {
    let v = vec![183, 65, 14];
    let c: Result<Color, String> = Color::try_from(&v[..]);
    assert_eq!(
        c,
        Ok(Color {
            red: 183,
            green: 65,
            blue: 14
        })
    );
}
#[test]
fn test_slice_excess_length() {
    let v = vec![0, 0, 0, 0];
    assert!(Color::try_from(&v[..]).is_err());
}
#[test]
fn test_slice_insufficient_length() {
    let v = vec![0, 0];
    assert!(Color::try_from(&v[..]).is_err());
}