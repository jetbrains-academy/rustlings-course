### Checking for Panics with `should_panic`

In addition to checking that our code returns the correct values we expect, it’s also important to check that our code handles error conditions as we expect. For example, consider the `Guess` type that we created in the section "To Panic or Not to Panic" of the chapter "Recoverable and Unrecoverable Errors", listing "A `Guess` type that will only continue with
values between 1 and 100". Other code that uses `Guess` depends on the guarantee that `Guess` instances will contain only values between 1 and 100\. We can write a test that ensures that attempting to create a `Guess` instance with a value outside that range panics.

We do this by adding another attribute, `should_panic`, to our test function. This attribute makes a test pass if the code inside the function panics; the test will fail if the code inside the function doesn’t panic.

The code snippet below shows a test that checks that the error conditions of `Guess::new` happen when we expect them to.

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

##### Example of testing that a condition will cause a `panic!`

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

When we run the test in "Example of testing that a condition will cause a `panic!`", it will fail:

```text
running 1 test
test tests::greater_than_100 ... FAILED

failures:

---- tests::greater_than_100 stdout ----
note: test did not panic as expected

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

We don’t get a very helpful message in this case, but when we look at the test function, we see that it’s annotated with `#[should_panic]`. The failure we got means that the code in the test function did not cause a panic.

Tests that use `should_panic` can be imprecise because they only indicate that the code has caused some panic. A `should_panic` test would pass even if the test panics for a different reason from the one we were expecting to happen. To make `should_panic` tests more precise, we can add an optional `expected` parameter to the `should_panic` attribute. The test harness will make sure that the failure message contains the provided text. For example, consider the modified code for `Guess` (below) where the `new` function panics with different messages depending on whether the value is too small or too large.

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
thread 'main' panicked at 'Guess value must be greater than or equal to 1, got 200.', src/lib.rs:13:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: panic did not contain expected string
      panic message: `"Guess value must be greater than or equal to 1, got 200."`,
 expected substring: `"Guess value must be less than or equal to 100"`

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

The failure message indicates that this test did indeed panic as we expected, but the panic message did not include the expected string `Guess value must be less than or equal to 100`. The panic message that we did get in this case was `Guess value must be greater than or equal to 1, got 200.` Now we can start figuring out where our bug is!

