use initiate_struct::classic_c_structs;
use initiate_struct::tuple_structs;
use initiate_struct::unit_structs;

fn main() {
    let cl_str = classic_c_structs();
    println!("{}", "Classic Struct:");
    println!("Name: {:?}", cl_str.name);
    println!("Hex: {:?}", cl_str.hex);

    let tup_str = tuple_structs();
    println!("{}", "Tuple Struct:");
    println!("Name: {:?}", tup_str.0);
    println!("Hex: {:?}", tup_str.1);

    let msg = unit_structs();
    println!("{}", msg);
}

