pub fn calculateprice(order_amount: i32) -> i32 {
    return if order_amount > 40 {
        order_amount
    } else {
        order_amount * 2
    }
}
