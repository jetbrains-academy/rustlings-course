fn main() {
    let x = classic_c_structs();
    println!("{:?}", x);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
struct ColorClassicStruct {
    name: String,
    hex: String,
}

struct ColorTupleStruct(String, String);

#[derive(Debug)]
struct UnitStruct;

fn classic_c_structs() -> ColorClassicStruct {
    let green = ColorClassicStruct{
        name: String::from("green"),
        hex: String::from("#00FF00"),
    };
    return green
}

fn tuple_structs() -> ColorTupleStruct {
    let green = ColorTupleStruct(String::from("green"), String::from("#00FF00"));
    return green
}

fn unit_structs() -> String {
    let unit_struct = UnitStruct;
    let message = format!("{:?}s are fun!", unit_struct);
    return message
}