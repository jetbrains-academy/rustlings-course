use struct_update::your_order;
use struct_update::create_order_template;


#[test]
fn test_your_order() {
    let yr_order = your_order();
    let order_tmp = create_order_template();

    assert_eq!(yr_order.name, "Hacker in Rust");
    assert_eq!(yr_order.year, order_tmp.year);
    assert_eq!(yr_order.made_by_phone, order_tmp.made_by_phone);
    assert_eq!(yr_order.made_by_mobile, order_tmp.made_by_mobile);
    assert_eq!(yr_order.made_by_email, order_tmp.made_by_email);
    assert_eq!(yr_order.item_number, order_tmp.item_number);
    assert_eq!(yr_order.count, 1);
}