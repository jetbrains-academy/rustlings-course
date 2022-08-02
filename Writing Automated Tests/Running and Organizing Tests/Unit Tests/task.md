### Unit Tests

The purpose of unit tests is to test each unit of code in isolation from the
rest of the code to quickly pinpoint where code is and isn’t working as
expected. You’ll put unit tests in the *src* directory in each file with the
code that they’re testing. The convention is to create a module named `tests`
in each file to contain the test functions and to annotate the module with
`cfg(test)`.

#### The Tests Module and `#[cfg(test)]`

The `#[cfg(test)]` annotation on the tests module tells Rust to compile and run
the test code only when you run `cargo test`, not when you run `cargo build`.
This saves compile time when you only want to build the library and saves space
in the resulting compiled artifact because the tests are not included. You’ll
see that because integration tests go in a different directory, they don’t need
the `#[cfg(test)]` annotation. However, because unit tests go in the same files
as the code, you’ll use `#[cfg(test)]` to specify that they shouldn’t be
included in the compiled result.

Recall the example of a test module and a function generated automatically by cargo new in the first section of
this chapter:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

This code is the automatically generated test module. The attribute `cfg`
stands for *configuration* and tells Rust that the following item should only
be included given a certain configuration option. In this case, the
configuration option is `test`, which is provided by Rust for compiling and
running tests. By using the `cfg` attribute, Cargo compiles our test code only
if we actively run the tests with `cargo test`. This includes any helper
functions that might be within this module, in addition to the functions
annotated with `#[test]`.

#### Testing Private Functions

There’s debate within the testing community about whether private
functions should be tested directly, and other languages make it difficult or
impossible to test private functions. Regardless of which testing ideology you
adhere to, Rust’s privacy rules do allow you to test private functions.
Consider the code below with the private function `internal_adder`.


```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

##### Testing a private function

Note that the `internal_adder` function is not marked as `pub`, but because
tests are just Rust code and the `tests` module is just another module, you can
bring `internal_adder` into a test’s scope and call it. If you don’t think
private functions should be tested, there’s nothing in Rust that will compel
you to do so.