### Using External Packages

Suppose we want to get random numbers. For that, we'll need an external package called `rand`. To use `rand` in our project, we add this line to _Cargo.toml_:

```toml
    [dependencies]
    rand = "0.8.5"
```

Adding `rand` as a dependency in _Cargo.toml_ tells Cargo to download the `rand` package and any dependencies from _https://crates.io_ and make `rand` available to our project.

Then, to bring `rand` definitions into the scope of our package, we add a `use` line starting with the name of the crate, `rand`, and list the items we wanted to bring into scope. For example, we bring the `Rng` trait into scope and call the `rand::thread_rng` function:

```rust
    use rand::Rng;
    fn main() {
        let secret_number = rand::thread_rng().gen_range(1..=100);
    }
```

Members of the Rust community have made many packages available at _https://crates.io_, and pulling any of them into your package involves these same steps: listing them in your package’s _Cargo.toml_ file and using `use` to bring items from their crates into scope.

Note that the standard library (`std`) is also a crate that’s external to our package. Because the standard library is shipped with the Rust language, we don’t need to change _Cargo.toml_ to include `std`. But we do need to refer to it with `use` to bring items from there into our package’s scope. For example, with `HashMap` we would use this line:

```rust
    use std::collections::HashMap;
```

This is an absolute path starting with `std`, the name of the standard library crate.
