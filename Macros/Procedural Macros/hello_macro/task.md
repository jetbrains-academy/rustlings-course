### Step 1: Creating a Library Crate

The first step is to make a new library crate. Here we already did it, 
but if you want to explore it on your own, you can do it like this:

```text
$ cargo new hello_macro --lib
```

Next, we’ll define the `HelloMacro` trait and its associated function in `lib.rs`:

```rust
pub trait HelloMacro {
    fn hello_macro();
}
```

We have a trait and its function. At this point, our crate user could implement the trait in `main.rs` to achieve the desired functionality, like so:

```rust
use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
```

However, they would need to write the implementation block for each type they wanted to use with `hello_macro`; we want to spare them from having to do this work.

Additionally, we can’t yet provide the `hello_macro` function with default implementation that will print the name of the type the trait is implemented on: Rust doesn’t have reflection capabilities, so it can’t look up the type’s name at runtime. We need a macro to generate code at compile time.