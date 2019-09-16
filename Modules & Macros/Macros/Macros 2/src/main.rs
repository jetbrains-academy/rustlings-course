/*This would be the wrong place to put the main function declaration, as the macro would be
undefined at this moment*/

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
