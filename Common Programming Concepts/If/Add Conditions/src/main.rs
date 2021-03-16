use add_conditions::fizz_if_foo;

fn main() {
    println!("{}", "output for fizz:");
    println!("{}", fizz_if_foo("fizz"));
    println!("{}", "output for fuzz:");
    println!("{}", fizz_if_foo("fuzz"));
    println!("{}", "output for literally anything:");
    println!("{}", fizz_if_foo("literally anything"));
}
