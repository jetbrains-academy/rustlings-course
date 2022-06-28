use calculate_price::calculate_price;

#[test]
fn verify_test() {
    //Arrange
    let price1 = calculate_price(55);
    let price2 = calculate_price(40);
    let price3 = calculate_price(20);

    //Assert
    assert_eq!(price1, 55, "There was an incorrect price for an order containing more then 40 apples.");
    assert_eq!(price2, 40, "There was an incorrect price for an order containing 40 apples.");
    assert_eq!(price3, 40, "There was an incorrect price for an order containing less then 40 apples.");
}