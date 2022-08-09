use combine_traits::{some_func, SomeStruct};

fn main() {
    let ss = SomeStruct {name: "Some Struct".to_string()};
    some_func(ss);
}
