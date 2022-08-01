### Checking Results with the assert! Macro

The `assert!` macro, provided by the standard library, is useful when you want to ensure that some condition in a test evaluates to `true`. We give the `assert!` macro an argument that evaluates to a Boolean. If the value is `true`, `assert!` does nothing and the test passes. If the value is `false`, the `assert!` macro calls the `panic!` macro, which causes the test to fail. Using the `assert!` macro helps us check that our code is functioning in the way we intend.

In the chapter "Structs/Method Syntax", listing "Implementing the `can_hold` method on
`Rectangle` that takes another `Rectangle` instance as a parameter", we used a `Rectangle` struct and a `can_hold` method, which are repeated here below. Let’s put this code in the _src/lib.rs_ file and write some tests for it using the `assert!` macro.

```rust
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
```

##### Example of using the `Rectangle` struct and its `can_hold` method from the chapter "Structs/Method Syntax"

The `can_hold` method returns a Boolean, which means it’s a perfect use case for the `assert!` macro. In the code snippet below, we write a test that exercises the `can_hold` method by creating a `Rectangle` instance that has a width of 8 and a height of 7 and asserting that it can hold another `Rectangle` instance that has a width of 5 and a height of 1.

```rust
   #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn larger_can_hold_smaller() {
            let larger = Rectangle { width: 8, height: 7 };
            let smaller = Rectangle { width: 5, height: 1 };

            assert!(larger.can_hold(&smaller));
        }
    }
```

##### Example of a test for `can_hold` that checks whether a larger rectangle can indeed hold a smaller rectangle

Note that we’ve added a new line inside the `tests` module: `use super::*;`. The `tests` module is a regular module that follows the usual visibility rules we covered in the Intro of "Modules" (chapter "Modules and Macros"). Because the `tests` module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module. We use a glob here so anything we define in the outer module is available to this `tests` module.

We’ve named our test `larger_can_hold_smaller`, and we’ve created the two `Rectangle` instances that we need. Then we called the `assert!` macro and passed it the result of calling `larger.can_hold(&smaller)`. This expression is supposed to return `true`, so our test should pass. Let’s find out!

```text
    running 1 test
    test tests::larger_can_hold_smaller ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

It does pass! Let’s add another test, this time asserting that a smaller rectangle cannot hold a larger rectangle:

```rust
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn larger_can_hold_smaller() {
            // --snip--
        }

        #[test]
        fn smaller_cannot_hold_larger() {
            let larger = Rectangle { width: 8, height: 7 };
            let smaller = Rectangle { width: 5, height: 1 };

            assert!(!smaller.can_hold(&larger));
        }
    }
```

Because the correct result of the `can_hold` function in this case is `false`, we need to negate that result before we pass it to the `assert!` macro. As a result, our test will pass if `can_hold` returns `false`:

```text
    running 2 tests
    test tests::smaller_cannot_hold_larger ... ok
    test tests::larger_can_hold_smaller ... ok

    test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Two tests that pass! Now let’s see what happens to our test results when we introduce a bug in our code. Let’s change the implementation of the `can_hold` method by replacing the greater than sign with a less than sign when it compares the widths:

```rust
    // --snip--

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width < other.width && self.height > other.height
        }
    }
```

Running the tests now produces the following:

```text
    running 2 tests
    test tests::smaller_cannot_hold_larger ... ok
    test tests::larger_can_hold_smaller ... FAILED

    failures:

    ---- tests::larger_can_hold_smaller stdout ----
    thread 'tests::larger_can_hold_smaller' panicked at 'assertion failed:
    larger.can_hold(&smaller)', src/lib.rs:22:9
    note: Run with `RUST_BACKTRACE=1` for a backtrace.

    failures:
        tests::larger_can_hold_smaller

    test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Our tests caught the bug! Because `larger.width` is 8 and `smaller.width` is 5, the comparison of the widths in `can_hold` now returns `false`: 8 is not less than 5.
