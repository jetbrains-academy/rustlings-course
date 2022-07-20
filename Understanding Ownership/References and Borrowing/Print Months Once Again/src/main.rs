fn main() {
    let mut months = ["January", "February", "March",
        "April", "May", "June",
        "July", "August", "September",
        "October", "November", "December"];

    print_months(&months);
    print_months_reversed(&mut months);
    print_months(&months);
}

fn print_months(months: &[&str; 12]) {
    for month in months {
        print!("{} ", month)
    }
    println!()
}

fn print_months_reversed(months: &mut [&str; 12]) {
    months.reverse();
    print_months(&months);
}