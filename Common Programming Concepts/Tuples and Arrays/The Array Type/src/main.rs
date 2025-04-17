fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    const MONTHS: [&str; 12] = ["January", "February", "March",
        "April", "May", "June",
        "July", "August", "September",
        "October", "November", "December"];

    println!("The last month is {}", MONTHS[MONTHS.len() - 1]);
}
