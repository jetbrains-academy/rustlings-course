#### The `?` Operator Can Only Be Used in Functions That Return `Result`

The `?` operator can only be used in functions that have a return type of `Result`, because it is defined to work in the same way as the `match` expression we defined in the example with code causing error on match. The part of the `match` that requires a return type of `Result` is `return Err(e)`, so the return type of the function has to be a `Result` to be compatible with this `return`.

Let’s look at what happens if we use the `?` operator in the `main` function, which you’ll recall has a return type of `()`:

```rust
    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt")?;
    }
```

When we compile this code, we get the following error message:

```text
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `Try`)
 --> src/main.rs:4:13
  |
3 | / fn main() {
4 | |     let f = File::open("hello.txt")?;
  | |             ^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a function that returns `()`
5 | | }
  | |_- this function should return `Result` or `Option` to accept `?`
  |
  = help: the trait `Try` is not implemented for `()`
  = note: required by `from_error`
```

This error points out that we’re only allowed to use the `?` operator in a function that returns `Result` or `Option` or another type that implements
`std::ops::Try`. When you’re writing code in a function that doesn’t return one of these types, and you want to use `?` when you call other functions that return `Result<T, E>`, you have two choices to fix this problem. One technique is to change the return type of your function to be `Result<T, E>` if you have no restrictions preventing that. The other technique is to use a `match` or one of the `Result<T, E>` methods to handle the `Result<T, E>` in whatever way is appropriate.

The `main` function is special, and there are restrictions on what its return type must be. One valid return type for main is `()`, and conveniently, another valid return type is `Result<T, E>`, as shown here:

```rust
    use std::error::Error;
    use std::fs::File;

    fn main() -> Result<(), Box<dyn Error>> {
        let f = File::open("hello.txt")?;

        Ok(())
    }
```

The `Box<dyn Error>` type is called a _trait object_, which is discussed in the [“Using Trait Objects that Allow for Values of Different Types”](https://doc.rust-lang.org/stable/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types) section in Chapter 17 of the book\. For now, you can read `Box<dyn Error>` to mean “any kind of error.” Using `?` in a `main` function with this return type is allowed.

Now that we’ve discussed the details of calling `panic!` or returning `Result`, let’s return to the topic of how to decide which is appropriate to use in which cases.

_You can refer to the following chapter in the Rust Programming Language Book: [Recoverable Errors with Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#recoverable-errors-with-result)_