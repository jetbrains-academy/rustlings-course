fn main() {
    let adam_info = (21, "Adam");
    let adam_info_copy = adam_info; // (*)

    println!("{:?}", adam_info);
    println!("{:?}", adam_info_copy);

    let eva_info = (18, String::from("Eva"));
    let eva_info_copy = eva_info.clone();  // (**)

    println!("{:?}", eva_info);
    println!("{:?}", eva_info_copy);
}
