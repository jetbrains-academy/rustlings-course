use vec_trait::*;

fn main() {
    fn is_vec_pop_eq_bar() {
        let foo = vec![String::from("Foo")].append_bar();
        println!("{:?}", foo)
    }
    is_vec_pop_eq_bar()
}
