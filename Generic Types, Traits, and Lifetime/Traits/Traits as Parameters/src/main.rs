use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String;
}

// Trait as a parameter
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound Syntax
pub fn notify2<T: Summary>(item: &T) {}

pub fn notify3<T: Summary>(item1: &T, item2: &T) {}

// Multiple Trait Bounds with the + Syntax
pub fn notify4(item: &(impl Summary + Display)) {}

pub fn notify5<T: Summary + Display>(item: &T) {}

// Clearer Trait Bounds with where Clauses
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { 0 }
fn some_function2<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{ 0 }

fn main() {
    // put you code here to launch it
}
