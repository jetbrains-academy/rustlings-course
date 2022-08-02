use generics_in_structs::*;

fn main() {
    fn store_u32_in_wrapper() {
        println!("{}", "u32 in wrapper:");
        println!("{:?}", Wrapper::new(42).value);
    }

    fn store_str_in_wrapper() {
        println!("{}", "String in wrapper:");
        println!("{:?}", Wrapper::new("Foo").value);
    }
    store_u32_in_wrapper();
    store_str_in_wrapper();
}