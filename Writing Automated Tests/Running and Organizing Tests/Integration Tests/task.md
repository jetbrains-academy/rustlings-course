### Integration Tests

In Rust, integration tests are entirely external to your library. They use your
library in the same way any other code would, which means they can only call
functions that are part of your library’s public API. Their purpose is to test
whether many parts of your library work together correctly. Units of code that
work correctly on their own could have problems when integrated, so test
coverage of the integrated code is important as well. To create integration
tests, you first need a *tests* directory.

#### The *tests* Directory

We created a *tests* directory at the top level of our project directory, next
to *src*. Cargo knows to look for integration test files in this directory. We
can then make as many test files as we want to in this directory, and Cargo
will compile each of the files as an individual crate.

Let’s look at an integration test. With the code from listing "Testing a private function" in the
*src/lib.rs* file, look in a *tests* directory, where there is a file named
*tests/integration_test.rs* with the code from the listing below.

```rust
use integration_tests;

#[test]
fn it_adds_two() {
    assert_eq!(4, integration_tests::add_two(2));
}
```

##### An integration test of a function in the `integration_tests` crate

We’ve added `use integration_tests;` at the top of the code, which we didn’t need in the
unit tests. The reason is that each file in the `tests` directory is a separate
crate, so we need to bring our library into each test crate’s scope.

We don’t need to annotate any code in *tests/integration_test.rs* with
`#[cfg(test)]`. Cargo treats the `tests` directory specially and compiles files
in this directory only when we run `cargo test`. Run `cargo test` now:

```text
Compiling integration_tests v0.1.0 
    Finished test [unoptimized + debuginfo] target(s) in 0.54s
     Running target/debug/deps/integration_tests-61f5d8d60ccbcc19
     
running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_tests-d5df7484b111e79e

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests integration_tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The three sections of output include the unit tests, the integration test, and
the doc tests. The first section for the unit tests is the same as we’ve been
seeing: one line for each unit test (one named `internal` that we added in
"Testing a private function") and then a summary line for the unit tests.

The integration tests section starts with the line `Running target/debug/deps/integration_tests-d5df7484b111e79e` (the hash at the end of
your output will be different). Next, there is a line for each test function in
that integration test and a summary line for the results of the integration
test just before the `Doc-tests integration_tests` section starts.

Similarly to how adding more unit test functions adds more result lines to the
unit tests section, adding more test functions to the integration test file
adds more result lines to this integration test file’s section. Each
integration test file has its own section, so if we add more files in the
*tests* directory, there will be more integration test sections.

We can still run a particular integration test function by specifying the test
function’s name as an argument to `cargo test`. To run all the tests in a
particular integration test file, use the `--test` argument of `cargo test`
followed by the name of the file (`cargo test --test integration_test`):

```text
running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

This command runs only the tests in the *tests/integration_test.rs* file.
