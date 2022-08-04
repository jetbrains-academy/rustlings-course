#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // Using `sort_by_key` and a closure to sort a list
    // of `Rectangle` instances by their `width` value
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    // Attempting to use an FnOnce closure with sort_by_key
    let mut sort_operations: Vec<Rectangle> = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        // !!! ERROR:
        // sort_operations.push(value);
        r.width
    });
    println!("{:#?}", list);

    // Using an FnMut closure with sort_by_key is allowed
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}