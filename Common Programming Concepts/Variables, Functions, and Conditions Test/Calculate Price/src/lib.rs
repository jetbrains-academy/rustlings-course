pub fn calculate_price(order_amount: i32) -> i32 {
    if order_amount >= 40 {
        order_amount
    } else {
        order_amount * 2
    }
}
