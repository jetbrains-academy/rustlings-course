## Adding Custom Failure Messages

You can also add a custom message to be printed with the failure message as optional arguments to the `assert!`, `assert_eq!`, and `assert_ne!` macros. Any arguments specified after the one required argument to `assert!` or the two required arguments to `assert_eq!` and `assert_ne!` are passed along to the `format!` macro (discussed in Chapter 8 in the [“Concatenation with the `+` Operator or the `format!` Macro”](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro) section), so you can pass a format string that contains `{}` placeholders and values to go in those placeholders. Custom messages are useful to document what an assertion means; when a test fails, you’ll have a better idea of what the problem is with the code.

For example, let’s say we have a function that greets people by name and we want to test that the name we pass into the function appears in the output:

```rust
    pub fn greeting(name: &str) -> String {
        format!("Hello {}!", name)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn greeting_contains_name() {
            let result = greeting("Carol");
            assert!(result.contains("Carol"));
        }
    }
```

The requirements for this program haven’t been agreed upon yet, and we’re pretty sure the `Hello` text at the beginning of the greeting will change. We decided we don’t want to have to update the test when the requirements change, so instead of checking for exact equality to the value returned from the `greeting` function, we’ll just assert that the output contains the text of the input parameter.

Let’s introduce a bug into this code by changing `greeting` to not include `name` to see what this test failure looks like:

```rust
    pub fn greeting(name: &str) -> String {
        String::from("Hello!")
    }
```

Running this test produces the following:

```text
    running 1 test
    test tests::greeting_contains_name ... FAILED

    failures:

    ---- tests::greeting_contains_name stdout ----
    thread 'tests::greeting_contains_name' panicked at 'assertion failed:
    result.contains("Carol")', src/lib.rs:12:9
    note: Run with `RUST_BACKTRACE=1` for a backtrace.

    failures:
        tests::greeting_contains_name
```

This result just indicates that the assertion failed and which line the assertion is on. A more useful failure message in this case would print the value we got from the `greeting` function. Let’s change the test function, giving it a custom failure message made from a format string with a placeholder filled in with the actual value we got from the `greeting` function:

```rust
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }
```

Now when we run the test, we’ll get a more informative error message:

```text
    ---- tests::greeting_contains_name stdout ----
    thread 'tests::greeting_contains_name' panicked at 'Greeting did not
    contain name, value was `Hello!`', src/lib.rs:12:9
    note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

We can see the value we actually got in the test output, which would help us debug what happened instead of what we were expecting to happen.

### Checking for Panics with should_panic

](ch11-01-writing-tests.html#checking-for-panics-with-should_panic)

In addition to checking that our code returns the correct values we expect, it’s also important to check that our code handles error conditions as we expect. For example, consider the `Guess` type that we created in Chapter 9, Listing 9-10\. Other code that uses `Guess` depends on the guarantee that `Guess` instances will contain only values between 1 and 100\. We can write a test that ensures that attempting to create a `Guess` instance with a value outside that range panics.

We do this by adding another attribute, `should_panic`, to our test function. This attribute makes a test pass if the code inside the function panics; the test will fail if the code inside the function doesn’t panic.

Listing 11-8 shows a test that checks that the error conditions of `Guess::new` happen when we expect them to.

```rust
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess {
                value
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        #[should_panic]
        fn greater_than_100() {
            Guess::new(200);
        }
    }
```

###### Example of testing that a condition will cause a panic!

We place the `#[should_panic]` attribute after the `#[test]` attribute and before the test function it applies to. Let’s look at the result when this test passes:

```text
    running 1 test
    test tests::greater_than_100 ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Looks good! Now let’s introduce a bug in our code by removing the condition that the `new` function will panic if the value is greater than 100:

```rust
    // --snip--

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1  {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess {
                value
            }
        }
    }
```

When we run the test in Listing 11-8, it will fail:

```text
    running 1 test
    test tests::greater_than_100 ... FAILED

    failures:

    failures:
        tests::greater_than_100

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

We don’t get a very helpful message in this case, but when we look at the test function, we see that it’s annotated with `#[should_panic]`. The failure we got means that the code in the test function did not cause a panic.

Tests that use `should_panic` can be imprecise because they only indicate that the code has caused some panic. A `should_panic` test would pass even if the test panics for a different reason from the one we were expecting to happen. To make `should_panic` tests more precise, we can add an optional `expected` parameter to the `should_panic` attribute. The test harness will make sure that the failure message contains the provided text. For example, consider the modified code for `Guess` in Listing 11-9 where the `new` function panics with different messages depending on whether the value is too small or too large.

```rust
    // --snip--

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 {
                panic!("Guess value must be greater than or equal to 1, got {}.",
                       value);
            } else if value > 100 {
                panic!("Guess value must be less than or equal to 100, got {}.",
                       value);
            }

            Guess {
                value
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        #[should_panic(expected = "Guess value must be less than or equal to 100")]
        fn greater_than_100() {
            Guess::new(200);
        }
    }
```

##### Example of testing that a condition will cause a panic! with a particular panic message

This test will pass because the value we put in the `should_panic` attribute’s `expected` parameter is a substring of the message that the `Guess::new` function panics with. We could have specified the entire panic message that we expect, which in this case would be `Guess value must be less than or equal to 100, got 200.` What you choose to specify in the expected parameter for `should_panic` depends on how much of the panic message is unique or dynamic and how precise you want your test to be. In this case, a substring of the panic message is enough to ensure that the code in the test function executes the `else if value > 100` case.

To see what happens when a `should_panic` test with an `expected` message fails, let’s again introduce a bug into our code by swapping the bodies of the `if value < 1` and the `else if value > 100` blocks:

```rust
    if value < 1 {
        panic!("Guess value must be less than or equal to 100, got {}.", value);
    } else if value > 100 {
        panic!("Guess value must be greater than or equal to 1, got {}.", value);
    }
```

This time when we run the `should_panic` test, it will fail:

```text
    running 1 test
    test tests::greater_than_100 ... FAILED

    failures:

    ---- tests::greater_than_100 stdout ----
    thread 'tests::greater_than_100' panicked at 'Guess value must be
    greater than or equal to 1, got 200.', src/lib.rs:11:13
    note: Run with `RUST_BACKTRACE=1` for a backtrace.
    note: Panic did not include expected string 'Guess value must be less than or
    equal to 100'

    failures:
        tests::greater_than_100

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

The failure message indicates that this test did indeed panic as we expected, but the panic message did not include the expected string `'Guess value must be less than or equal to 100'`. The panic message that we did get in this case was `Guess value must be greater than or equal to 1, got 200.` Now we can start figuring out where our bug is!

[

### Using Result<T, E> in Tests

So far, we’ve written tests that panic when they fail. We can also write tests that use `Result<T, E>`! Here’s the test from Listing 11-1, rewritten to use `Result<T, E>` and return an `Err` instead of panicking:

```rust
    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() -> Result<(), String> {
            if 2 + 2 == 4 {
                Ok(())
            } else {
                Err(String::from("two plus two does not equal four"))
            }
        }
    }
```

The `it_works` function now has a return type, `Result<(), String>`. In the body of the function, rather than calling the `assert_eq!` macro, we return `Ok(())` when the test passes and an `Err` with a `String` inside when the test fails.

Writing tests so they return a `Result<T, E>` enables you to use the question mark operator in the body of tests, which can be a convenient way to write tests that should fail if any operation within them returns an `Err` variant.

You can’t use the `#[should_panic]` annotation on tests that use `Result<T, E>`. Instead, you should return an `Err` value directly when the test should fail.

Now that you know several ways to write tests, let’s look at what is happening when we run our tests and explore the different options we can use with `cargo test`.

_You can refer to the following chapter in the Rust Programming Language Book:[Adding Custom Failure Messages](https://doc.rust-lang.org/stable/book/ch11-01-writing-tests.html#adding-custom-failure-messages)_