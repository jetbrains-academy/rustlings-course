#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
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

fn your_order() -> Order {
    let order_template = create_order_template();
    let your_order = Order {
        name: String::from("Hacker in Rust"),
        count: 1,
        ..order_template
    };
    return your_order
}

fn main() {
    let x = your_order();
    println!("{:?}", x);
}


