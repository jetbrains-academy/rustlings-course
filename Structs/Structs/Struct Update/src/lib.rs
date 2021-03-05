#[derive(Debug)]
pub struct Order {
    pub name: String,
    pub year: u32,
    pub made_by_phone: bool,
    pub made_by_mobile: bool,
    pub made_by_email: bool,
    pub item_number: u32,
    pub count: u32,
}

pub fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}


pub fn your_order() -> Order {
    let order_template = create_order_template();
    let your_order = Order {
        name: String::from("Hacker in Rust"),
        count: 1,
        ..order_template
    };
    return your_order
}

