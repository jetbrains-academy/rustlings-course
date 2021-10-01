use try_from_into::*;
use std::convert::{TryFrom, TryInto};

#[test]
fn test_tuple_out_of_range_positive() {
    assert_eq!(
        Color::try_from((256, 1000, 10000)),
        Err(IntoColorError::IntConversion)
    );
}
#[test]
fn test_tuple_out_of_range_negative() {
    assert_eq!(
        Color::try_from((-1, -10, -256)),
        Err(IntoColorError::IntConversion)
    );
}
#[test]
fn test_tuple_sum() {
    assert_eq!(
        Color::try_from((-1, 255, 255)),
        Err(IntoColorError::IntConversion)
    );
}
#[test]
fn test_tuple_correct() {
    let c: Result<Color, _> = (183, 65, 14).try_into();
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
    let c: Result<Color, _> = [1000, 10000, 256].try_into();
    assert_eq!(c, Err(IntoColorError::IntConversion));
}
#[test]
fn test_array_out_of_range_negative() {
    let c: Result<Color, _> = [-10, -256, -1].try_into();
    assert_eq!(c, Err(IntoColorError::IntConversion));
}
#[test]
fn test_array_sum() {
    let c: Result<Color, _> = [-1, 255, 255].try_into();
    assert_eq!(c, Err(IntoColorError::IntConversion));
}
#[test]
fn test_array_correct() {
    let c: Result<Color, _> = [183, 65, 14].try_into();
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
    assert_eq!(
        Color::try_from(&arr[..]),
        Err(IntoColorError::IntConversion)
    );
}
#[test]
fn test_slice_out_of_range_negative() {
    let arr = [-256, -1, -10];
    assert_eq!(
        Color::try_from(&arr[..]),
        Err(IntoColorError::IntConversion)
    );
}
#[test]
fn test_slice_sum() {
    let arr = [-1, 255, 255];
    assert_eq!(
        Color::try_from(&arr[..]),
        Err(IntoColorError::IntConversion)
    );
}
#[test]
fn test_slice_correct() {
    let v = vec![183, 65, 14];
    let c: Result<Color, _> = Color::try_from(&v[..]);
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
    assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
}
#[test]
fn test_slice_insufficient_length() {
    let v = vec![0, 0];
    assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
}