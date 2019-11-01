## How to Write Tests

Tests are Rust functions that verify that the non-test code is functioning in the expected manner. The bodies of test functions typically perform these three actions:

1.  Set up any needed data or state.
2.  Run the code you want to test.
3.  Assert the results are what you expect.

Let’s look at the features Rust provides specifically for writing tests that take these actions, which include the `test` attribute, a few macros, and the `should_panic` attribute.


### The Anatomy of a Test Function

At its simplest, a test in Rust is a function that’s annotated with the `test` attribute. Attributes are metadata about pieces of Rust code; one example is the `derive` attribute we used with structs in Chapter 5\. To change a function into a test function, add `#[test]` on the line before `fn`. When you run your tests with the `cargo test` command, Rust builds a test runner binary that runs the functions annotated with the `test` attribute and reports on whether each test function passes or fails.

When we make a new library project with Cargo, a test module with a test function in it is automatically generated for us. This module helps you start writing your tests so you don’t have to look up the exact structure and syntax of test functions every time you start a new project. You can add as many additional test functions and as many test modules as you want!

We’ll explore some aspects of how tests work by experimenting with the template test generated for us without actually testing any code. Then we’ll write some real-world tests that call some code that we’ve written and assert that its behavior is correct.

Let's modify the _src/lib.rs_ file. Its content should look like Listing 11-1.

```rust
    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {
            assert_eq!(2 + 2, 4);
        }
    }
```


##### Example of a test module and a function generated automatically by cargo new

For now, let’s ignore the top two lines and focus on the function to see how it works. Note the `#[test]` annotation before the `fn` line: this attribute indicates this is a test function, so the test runner knows to treat this function as a test. We could also have non-test functions in the `tests` module to help set up common scenarios or perform common operations, so we need to indicate which functions are tests by using the `#[test]` attribute.

The function body uses the `assert_eq!` macro to assert that 2 + 2 equals 4. This assertion serves as an example of the format for a typical test. Let’s run it to see that this test passes.

Right-click on the 'Intro' task and choose **Open in Terminal** and run the `cargo test` command.
 You will see the same output as in Listing 11-2.

```text
   $ cargo test
  Compiling intro v0.1.0
 Finished dev [unoptimized + debuginfo] target(s) in 0.38s
     Running target/debug/deps/intro-c8e247c4dd65e48f

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

##### Example of an output from running the automatically generated test

Cargo compiled and ran the test. After the `Compiling`, `Finished`, and `Running` lines is the line `running 1 test`. The next line shows the name of the generated test function, called `it_works`, and the result of running that test, `ok`. The overall summary of running the tests appears next. The text `test result: ok.` means that all the tests passed, and the portion that reads `1 passed; 0 failed` totals the number of tests that passed or failed.

Because we don’t have any tests we’ve marked as ignored, the summary shows `0 ignored`. We also haven’t filtered the tests being run, so the end of the summary shows `0 filtered out`. We’ll talk about ignoring and filtering out tests in the next section, [“Controlling How Tests Are Run.”](https://doc.rust-lang.org/stable/book/ch11-02-running-tests.html#controlling-how-tests-are-run)

The `0 measured` statistic is for benchmark tests that measure performance. Benchmark tests are, as of this writing, only available in nightly Rust. See [the documentation about benchmark tests](https://doc.rust-lang.org/unstable-book/library-features/test.html) to learn more.

The next part of the test output, which starts with `Doc-tests adder`, is for the results of any documentation tests. We don’t have any documentation tests yet, but Rust can compile any code examples that appear in our API documentation. This feature helps us keep our docs and our code in sync! We’ll discuss how to write documentation tests in the [“Documentation Comments as Tests”](https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests) section of Chapter 14\. For now, we’ll ignore the `Doc-tests` output.

Let’s change the name of our test to see how that changes the test output. Change the `it_works` function to a different name, such as `exploration`, like so:

```rust
    #[cfg(test)]
    mod tests {
        #[test]
        fn exploration() {
            assert_eq!(2 + 2, 4);
        }
    }
```

Then run `cargo test` again. The output now shows `exploration` instead of `it_works`:

```text
Compiling intro v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running target/debug/deps/intro-c8e247c4dd65e48f

running 1 test
test tests::exploration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Let’s add another test, but this time we’ll make a test that fails! Tests fail when something in the test function panics. Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed. We talked about the simplest way to cause a panic in Chapter 9, which is to call the `panic!` macro. Enter the new test, `another`, so your _src/lib.rs_ file looks like Listing 11-3.

```rust
    #[cfg(test)]
    mod tests {
        #[test]
        fn exploration() {
            assert_eq!(2 + 2, 4);
        }

        #[test]
        fn another() {
            panic!("Make this test fail");
        }
    }
```

##### Example of how adding a second test that will fail because we call the panic! macro

Run the tests again using `cargo test`. The output should look like Listing 11-4, which shows that our `exploration` test passed and `another` failed.
```text
Compiling intro v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running target/debug/deps/intro-c8e247c4dd65e48f

running 2 tests
test tests::exploration ... ok
test tests::another ... FAILED

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

##### Example of the test results when one test passes and one test fails

Instead of `ok`, the line `test tests::another` shows `FAILED`. Two new sections appear between the individual results and the summary: the first section displays the detailed reason for each test failure. In this case, `another` failed because it `panicked at 'Make this test fail'`, which happened on line 10 in the _src/lib.rs_ file. The next section lists just the names of all the failing tests, which is useful when there are lots of tests and lots of detailed failing test output. We can use the name of a failing test to run just that test to more easily debug it; we’ll talk more about ways to run tests in the [“Controlling How Tests Are Run”](https://doc.rust-lang.org/stable/book/ch11-02-running-tests.html#controlling-how-tests-are-run) section.

The summary line displays at the end: overall, our test result is `FAILED`. We had one test pass and one test fail.

Now that you’ve seen what the test results look like in different scenarios, let’s look at some macros other than `panic!` that are useful in tests.

### Checking Results with the assert! Macro

The `assert!` macro, provided by the standard library, is useful when you want to ensure that some condition in a test evaluates to `true`. We give the `assert!` macro an argument that evaluates to a Boolean. If the value is `true`, `assert!` does nothing and the test passes. If the value is `false`, the `assert!` macro calls the `panic!` macro, which causes the test to fail. Using the `assert!` macro helps us check that our code is functioning in the way we intend.

In Chapter 5, Listing 5-15, we used a `Rectangle` struct and a `can_hold` method, which are repeated here in Listing 11-5\. Let’s put this code in the _src/lib.rs_ file and write some tests for it using the `assert!` macro.

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

##### Example of using the `Rectangle` struct and its `can_hold` method from Chapter Using Structs to Structure Related Data

The `can_hold` method returns a Boolean, which means it’s a perfect use case for the `assert!` macro. In Listing 11-6, we write a test that exercises the `can_hold` method by creating a `Rectangle` instance that has a width of 8 and a height of 7 and asserting that it can hold another `Rectangle` instance that has a width of 5 and a height of 1.

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

Note that we’ve added a new line inside the `tests` module: `use super::*;`. The `tests` module is a regular module that follows the usual visibility rules we covered in Chapter 7 in the [“Modules as the Privacy Boundary”](https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html) section. Because the `tests` module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module. We use a glob here so anything we define in the outer module is available to this `tests` module.

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
    # #[derive(Debug)]
    # struct Rectangle {
    #     width: u32,
    #     height: u32,
    # }
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

[

### Testing Equality with the assert_eq! and assert_ne! Macros

A common way to test functionality is to compare the result of the code under test to the value you expect the code to return to make sure they’re equal. You could do this using the `assert!` macro and passing it an expression using the `==` operator. However, this is such a common test that the standard library provides a pair of macros—`assert_eq!` and `assert_ne!`—to perform this test more conveniently. These macros compare two arguments for equality or inequality, respectively. They’ll also print the two values if the assertion fails, which makes it easier to see _why_ the test failed; conversely, the `assert!` macro only indicates that it got a `false` value for the `==` expression, not the values that lead to the `false` value.

In Listing 11-7, we write a function named `add_two` that adds `2` to its parameter and returns the result. Then we test this function using the `assert_eq!` macro.

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

```rust
    running 1 test
    test tests::it_adds_two ... FAILED

    failures:

    ---- tests::it_adds_two stdout ----
    thread 'tests::it_adds_two' panicked at 'assertion failed: `(left == right)`
      left: `4`,
     right: `5`', src/lib.rs:11:9
    note: Run with `RUST_BACKTRACE=1` for a backtrace.

    failures:
        tests::it_adds_two

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Our test caught the bug! The `it_adds_two` test failed, displaying the message `assertion failed: `(left == right)`` and showing that `left` was `4` and `right` was `5`. This message is useful and helps us start debugging: it means the `left` argument to `assert_eq!` was `4` but the `right` argument, where we had `add_two(2)`, was `5`.

Note that in some languages and test frameworks, the parameters to the functions that assert two values are equal are called `expected` and `actual`, and the order in which we specify the arguments matters. However, in Rust, they’re called `left` and `right`, and the order in which we specify the value we expect and the value that the code under test produces doesn’t matter. We could write the assertion in this test as `assert_eq!(add_two(2), 4)`, which would result in a failure message that displays `assertion failed: `(left == right)`` and that `left` was `5` and `right` was `4`.

The `assert_ne!` macro will pass if the two values we give it are not equal and fail if they’re equal. This macro is most useful for cases when we’re not sure what a value _will_ be, but we know what the value definitely _won’t_ be if our code is functioning as we intend. For example, if we’re testing a function that is guaranteed to change its input in some way, but the way in which the input is changed depends on the day of the week that we run our tests, the best thing to assert might be that the output of the function is not equal to the input.

Under the surface, the `assert_eq!` and `assert_ne!` macros use the operators `==` and `!=`, respectively. When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the `PartialEq` and `Debug` traits. All the primitive types and most of the standard library types implement these traits. For structs and enums that you define, you’ll need to implement `PartialEq` to assert that values of those types are equal or not equal. You’ll need to implement `Debug` to print the values when the assertion fails. Because both traits are derivable traits, as mentioned in Listing 5-12 in Chapter 5, this is usually as straightforward as adding the `#[derive(PartialEq, Debug)]` annotation to your struct or enum definition. See Appendix C, [“Derivable Traits,”](https://doc.rust-lang.org/stable/book/appendix-03-derivable-traits.html) for more details about these and other derivable traits.

_You can refer to the following chapter in the Rust Programming Language Book:[Writing Tests](https://doc.rust-lang.org/stable/book/ch11-01-writing-tests.html)_