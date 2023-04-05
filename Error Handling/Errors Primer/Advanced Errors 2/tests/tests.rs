use advanced_errors_2::*;

#[test]
fn test_empty() {
    let res = "".parse::<Climate>();
    assert_eq!(res, Err(ParseClimateError::Empty));
    assert_eq!(res.unwrap_err().to_string(), "empty input");
}
#[test]
fn test_one_entry() {
    let res = "abc".parse::<Climate>();
    assert_eq!(res, Err(ParseClimateError::BadLen));
    assert_eq!(res.unwrap_err().to_string(), "incorrect number of fields");
}
#[test]
fn test_short() {
    let res = "Boston,1991".parse::<Climate>();
    assert_eq!(res, Err(ParseClimateError::BadLen));
    assert_eq!(res.unwrap_err().to_string(), "incorrect number of fields");
}
#[test]
fn test_long() {
    let res = "Paris,1920,17.2,extra".parse::<Climate>();
    assert_eq!(res, Err(ParseClimateError::BadLen));
    assert_eq!(res.unwrap_err().to_string(), "incorrect number of fields");
}
#[test]
fn test_no_city() {
    let res = ",1997,20.5".parse::<Climate>();
    assert_eq!(res, Err(ParseClimateError::NoCity));
    assert_eq!(res.unwrap_err().to_string(), "no city name");
}
#[test]
fn test_parse_int_neg() {
    let res = "Barcelona,-25,22.3".parse::<Climate>();
    assert!(matches!(res, Err(ParseClimateError::ParseInt(_))));
    let err = res.unwrap_err();
    if let ParseClimateError::ParseInt(ref inner) = err {
        assert_eq!(
            err.to_string(),
            format!("error parsing year: {}", inner.to_string())
        );
    } else {
        unreachable!();
    };
}
#[test]
fn test_parse_int_bad() {
    let res = "Beijing,foo,15.0".parse::<Climate>();
    assert!(matches!(res, Err(ParseClimateError::ParseInt(_))));
    let err = res.unwrap_err();
    if let ParseClimateError::ParseInt(ref inner) = err {
        assert_eq!(
            err.to_string(),
            format!("error parsing year: {}", inner.to_string())
        );
    } else {
        unreachable!();
    };
}
#[test]
fn test_parse_float() {
    let res = "Manila,2001,bar".parse::<Climate>();
    assert!(matches!(res, Err(ParseClimateError::ParseFloat(_))));
    let err = res.unwrap_err();
    if let ParseClimateError::ParseFloat(ref inner) = err {
        assert_eq!(
            err.to_string(),
            format!("error parsing temperature: {}", inner.to_string())
        );
    } else {
        unreachable!();
    };
}
#[test]
fn test_parse_good() {
    let res = "Munich,2015,23.1".parse::<Climate>();
    assert_eq!(
        res,
        Ok(Climate {
            city: "Munich".to_string(),
            year: 2015,
            temp: 23.1,
        })
    );
}
// #[test]
// #[ignore]
// fn test_downcast() {
//     let res = "São Paulo,-21,28.5".parse::<Climate>();
//     assert!(matches!(res, Err(ParseClimateError::ParseInt(_))));
//     let err = res.unwrap_err();
//     let inner: Option<&(dyn Error + 'static)> = err.source();
//     assert!(inner.is_some());
//     assert!(inner.unwrap().is::<ParseIntError>());
// }
