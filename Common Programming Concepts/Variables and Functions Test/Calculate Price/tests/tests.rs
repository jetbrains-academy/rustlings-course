use calculate_price::calculateprice;

#[test]
fn verify_test() {
    //Arrange
    let price1 = calculateprice(55);
    let price2 = calculateprice(40);
    let price3 = calculateprice(20);

    //Assert
    assert_eq!(price1, 55, "There was an incorrect price for an order containing more then 40 apples.");
    assert_eq!(price2, 80, "There was an incorrect price for an order containing 40 apples.");
    assert_eq!(price3, 40, "There was an incorrect price for an order containing less then 40 apples.");
}