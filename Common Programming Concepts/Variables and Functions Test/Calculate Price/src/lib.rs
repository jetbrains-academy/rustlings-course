pub fn calculateprice(order_amount: i32) -> i32 {
    if order_amount > 40 {
        return order_amount
    } else {
        return order_amount * 2
    }
}
