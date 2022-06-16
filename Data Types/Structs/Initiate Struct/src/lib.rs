pub struct ColorClassicStruct {
    pub name: String,
    pub hex: String,
}

pub struct ColorTupleStruct(pub String, pub String);

#[derive(Debug)]
pub struct UnitStruct;

pub fn classic_c_structs() -> ColorClassicStruct {
    let green = ColorClassicStruct{
        name: String::from("green"),
        hex: String::from("#00FF00"),
    };
    return green
}

pub fn tuple_structs() -> ColorTupleStruct {
    let green = ColorTupleStruct(String::from("green"), String::from("#00FF00"));
    return green
}

pub fn unit_structs() -> String {
    let unit_struct = UnitStruct;
    let message = format!("{:?}s are fun!", unit_struct);
    return message
}