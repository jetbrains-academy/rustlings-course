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

Let's modify the _src/lib.rs_ file. Its content should look like the code snippet below.

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

Right-click on the 'How to Write Tests' task and choose **Open in Terminal** and run the `cargo test` command.
You will see output similar to what is shown below.

```text
$ cargo test
  Compiling how_to_write_tests v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.38s
     Running target/debug/deps/intro-c8e247c4dd65e48f

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

##### Example of an output from running the automatically generated test

Cargo compiled and ran the test. After the `Compiling`, `Finished`, and `Running` lines is the line `running 1 test`. The next line shows the name of the generated test function, called `it_works`, and the result of running that test, `ok`. The overall summary of running the tests appears next. The text `test result: ok.` means that all the tests passed, and the portion that reads `1 passed; 0 failed` totals the number of tests that passed or failed.

Because we don’t have any tests we’ve marked as ignored, the summary shows `0 ignored`. We also haven’t filtered the tests being run, so the end of the summary shows `0 filtered out`. We’ll talk about ignoring and filtering out tests in the section "Running Tests".

The `0 measured` statistic is for benchmark tests that measure performance. Benchmark tests are, as of this writing, only available in nightly Rust. See [the documentation about benchmark tests](https://doc.rust-lang.org/unstable-book/library-features/test.html) to learn more.

The next part of the test output, which starts with `Doc-tests how_to_write_tests`, is for the results of any documentation tests. We don’t have any documentation tests yet, but Rust can compile any code examples that appear in our API documentation. This feature helps us keep our docs and our code in sync! We’ll discuss how to write documentation tests in the [“Documentation Comments as Tests”](https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests) section of Chapter 14 of the Rust Book. For now, we’ll ignore the `Doc-tests` output.

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
Compiling how_to_write_tests v0.1.0
   Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running target/debug/deps/intro-c8e247c4dd65e48f

running 1 test
test tests::exploration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Let’s add another test, but this time we’ll make a test that fails! Tests fail when something in the test function panics. Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed. We talked about the simplest way to cause a panic in Chapter 9, which is to call the `panic!` macro. Enter the new test, `another`, so your _src/lib.rs_ file looks like the code snippet below.

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

##### Example of adding a second test that will fail because we call the panic! macro

Run the tests again using `cargo test`. The output should look like the listing below, which shows that our `exploration` test passed and `another` failed.
```text
Compiling how_to_write_tests v0.1.0
   Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running target/debug/deps/intro-c8e247c4dd65e48f

running 2 tests
test tests::exploration ... ok
test tests::another ... FAILED

failures:

---- tests::another stdout ----
thread 'tests::another' panicked at 'Make this test fail', Writing Automated Tests/Tests/How to Write Tests/src/lib.rs:9:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

##### Example of the test results when one test passes and one test fails

Instead of `ok`, the line `test tests::another` shows `FAILED`. Two new sections appear between the individual results and the summary: the first section displays the detailed reason for each test failure. In this case, `another` failed because it `panicked at "Make this test fail"`, which happened on line 10 in the _src/lib.rs_ file. The next section lists just the names of all the failing tests, which is useful when there are lots of tests and lots of detailed failing test output. We can use the name of a failing test to run just that test to more easily debug it; you can find out more about ways to run tests in the "Running Tests" section.

The summary line displays at the end: overall, our test result is `FAILED`. We had one test pass and one test fail.

Now that you’ve seen what the test results look like in different scenarios, let’s look at some macros other than `panic!` that are useful in tests.
