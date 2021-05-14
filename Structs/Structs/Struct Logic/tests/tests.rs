use struct_logic::*;


#[test]
#[should_panic]
fn fail_creating_weightless_package() {
    let sender_country = String::from("Spain");
    let recipient_country = String::from("Austria");

    Package::new(sender_country, recipient_country, -2210);
}

#[test]
fn create_international_package() {
    let sender_country = String::from("Spain");
    let recipient_country = String::from("Russia");

    let package = Package::new(sender_country, recipient_country, 1200);

    assert!(package.is_international());
}

#[test]
fn create_local_package() {
    let sender_country = String::from("Canada");
    let recipient_country = sender_country.clone();

    let package = Package::new(sender_country, recipient_country, 1200);

    assert!(!package.is_international());
}

#[test]
fn calculate_transport_fees() {
    let sender_country = String::from("Spain");
    let recipient_country = String::from("Spain");

    let cents_per_gram = 3;

    let package = Package::new(sender_country, recipient_country, 1500);

    assert_eq!(package.get_fees(cents_per_gram), 4500);
}