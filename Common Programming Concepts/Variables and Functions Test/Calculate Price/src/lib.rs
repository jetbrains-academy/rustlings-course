use std::cmp::Ordering;

pub fn calculateprice(order_amount: i32) -> i32 {
    match order_amount.cmp(&40) {
        Ordering::Less => order_amount*2,
        Ordering::Equal => order_amount*2,
        Ordering::Greater => order_amount
    }
}
