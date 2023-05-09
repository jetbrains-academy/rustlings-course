fn main() {
    let year1993 = 1993;
    print_ten_years_ago(year1993);

    let year2021 = 2021;
    print_ten_years_ago(year2021);
}

fn print_ten_years_ago(year: i32) {
    println!("{}: ten years ago was {}", year, year - 10);
}