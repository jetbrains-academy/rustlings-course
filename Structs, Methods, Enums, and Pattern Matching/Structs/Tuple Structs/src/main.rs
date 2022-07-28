struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct Unit();

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let _unit = Unit;

    println!("Color({}, {}, {})", black.0, black.1, black.2);
    println!("Point({}, {}, {})", origin.0, origin.1, origin.2);
}
