### Adding Useful Functionality with Derived Traits

It’d be nice to be able to print an instance of `Rectangle` while we’re
debugging our program and see the values for all its fields. The code below tries
using the `println!` macro as we have used in previous chapters. This won’t
work, however.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}
```

#### Attempting to print a `Rectangle` instance

When we compile this code, we get an error with this core message:

```text
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

The `println!` macro can do many kinds of formatting, and by default, the curly
brackets tell `println!` to use formatting known as `Display`: output intended
for direct end user consumption. The primitive types we’ve seen so far
implement `Display` by default because there’s only one way you’d want to show
a `1` or any other primitive type to a user. But with structs, the way
`println!` should format the output is less clear because there are more
display possibilities: Do you want commas or not? Do you want to print the
curly brackets? Should all the fields be shown? Due to this ambiguity, Rust
doesn’t try to guess what we want, and structs don’t have a provided
implementation of `Display`.

If we continue reading the errors, we’ll find this helpful note:

```text
   = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

Let’s try it! The `println!` macro call will now look like `println!("rect1 is
{:?}", rect1);`. Putting the specifier `:?` inside the curly brackets tells
`println!` we want to use an output format called `Debug`. The `Debug` trait
enables us to print our struct in a way that is useful for developers so we can
see its value while we’re debugging our code.

Compile the code with this change. Drat! We still get an error:

```text
error[E0277]: `Rectangle` doesn't implement `Debug`
```

But again, the compiler gives us a helpful note:

```text
    = help: the trait `Debug` is not implemented for `Rectangle`
    = note: add `#[derive(Debug)]` or manually implement `Debug`
```

Rust *does* include functionality to print out debugging information, but we
have to explicitly opt in to make that functionality available for our struct.
To do that, we add the annotation `#[derive(Debug)]` just before the struct
definition, as shown below.

<span class="filename">Filename: src/main.rs</span>

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```

#### Adding the annotation to derive the `Debug` trait and printing the `Rectangle` instance using debug formatting

Now when we run the program, we won’t get any errors, and we’ll see the
following output:

```console
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/structs`
rect1 is Rectangle { width: 30, height: 50 }
```

Nice! It’s not the prettiest output, but it shows the values of all the fields
for this instance, which would definitely help during debugging. When we have
larger structs, it’s useful to have output that’s a bit easier to read; in
those cases, we can use `{:#?}` instead of `{:?}` in the `println!` string.
When we use the `{:#?}` style in the example, the output will look like this:

```console
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/structs`
rect1 is Rectangle {
    width: 30,
    height: 50,
}
```

Rust has provided a number of traits for us to use with the `derive` annotation
that can add useful behavior to our custom types. Those traits and their
behaviors are listed in [Appendix C][app3] of the Rust Book. We’ll cover how to implement these traits
with custom behavior as well as how to create your own traits in the chapter "Generic Types, Traits and Lifetimes".

Our `area` function is very specific: it only computes the area of rectangles.
It would be helpful to tie this behavior more closely to our `Rectangle`
struct because it won’t work with any other type. Let’s look at how we can
continue to refactor this code by turning the `area` function into an `area`
*method* defined on our `Rectangle` type.

[app3]: https://github.com/rust-lang/book/blob/master/src/appendix-03-derivable-traits.md
