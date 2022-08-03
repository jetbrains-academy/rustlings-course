### Testing Equality with the assert_eq! and assert_ne! Macros

A common way to test functionality is to compare the result of the code under test to the value you expect the code to return to make sure they’re equal. You could do this using the `assert!` macro and passing it an expression using the `==` operator. However, this is such a common test that the standard library provides a pair of macros—`assert_eq!` and `assert_ne!`—to perform this test more conveniently. These macros compare two arguments for equality or inequality, respectively. They’ll also print the two values if the assertion fails, which makes it easier to see _why_ the test failed; conversely, the `assert!` macro only indicates that it got a `false` value for the `==` expression, not the values that lead to the `false` value.

In the code snippet below, we write a function named `add_two` that adds `2` to its parameter and returns the result. Then we test this function using the `assert_eq!` macro.

```rust
    pub fn add_two(a: i32) -> i32 {
        a + 2
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_adds_two() {
            assert_eq!(4, add_two(2));
        }
    }
```

##### Example of testing the function add_two using the assert_eq! macro

Let’s check that it passes!

```text
    running 1 test
    test tests::it_adds_two ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The first argument we gave to the `assert_eq!` macro, `4`, is equal to the result of calling `add_two(2)`. The line for this test is `test tests::it_adds_two ... ok`, and the `ok` text indicates that our test passed!

Let’s introduce a bug into our code to see what it looks like when a test that uses `assert_eq!` fails. Change the implementation of the `add_two` function to instead add `3`:

```rust
    pub fn add_two(a: i32) -> i32 {
        a + 3
    }
```

Run the tests again:

```text
running 1 test
test tests::it_adds_two ... FAILED

failures:

---- tests::it_adds_two stdout ----
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `4`,
 right: `5`', src/lib.rs:11:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::it_adds_two

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Our test caught the bug! The `it_adds_two` test failed, displaying the message `assertion failed: '(left == right)'` and showing that `left` was `4` and `right` was `5`. This message is useful and helps us start debugging: it means the `left` argument to `assert_eq!` was `4` but the `right` argument, where we had `add_two(2)`, was `5`.

Note that in some languages and test frameworks, the parameters to the functions that assert two values are equal are called `expected` and `actual`, and the order in which we specify the arguments matters. However, in Rust, they’re called `left` and `right`, and the order in which we specify the value we expect and the value that the code under test produces doesn’t matter. We could write the assertion in this test as `assert_eq!(add_two(2), 4)`, which would result in a failure message that displays `assertion failed: '(left == right)'` and that `left` was `5` and `right` was `4`.

The `assert_ne!` macro will pass if the two values we give it are not equal and fail if they’re equal. This macro is most useful for cases when we’re not sure what a value _will_ be, but we know what the value definitely _won’t_ be if our code is functioning as we intend. For example, if we’re testing a function that is guaranteed to change its input in some way, but the way in which the input is changed depends on the day of the week that we run our tests, the best thing to assert might be that the output of the function is not equal to the input.

Under the surface, the `assert_eq!` and `assert_ne!` macros use the operators `==` and `!=`, respectively. When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the `PartialEq` and `Debug` traits. All the primitive types and most of the standard library types implement these traits. For structs and enums that you define, you’ll need to implement `PartialEq` to assert that values of those types are equal or not equal. You’ll need to implement `Debug` to print the values when the assertion fails. Because both traits are derivable traits, as mentioned in the listing "Adding the annotation to derive the `Debug`
trait and printing the `Rectangle` instance using debug formatting" in the chapter "Structs", section "Example Structs", this is usually as straightforward as adding the `#[derive(PartialEq, Debug)]` annotation to your struct or enum definition. See Appendix C, [“Derivable Traits,”](https://doc.rust-lang.org/stable/book/appendix-03-derivable-traits.html) for more details about these and other derivable traits.
