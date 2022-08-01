## How to Write Tests

Tests are Rust functions that verify that the non-test code is functioning in the expected manner. The bodies of test functions typically perform these three actions:

1.  Set up any needed data or state.
2.  Run the code you want to test.
3.  Assert the results are what you expect.

Let’s look at the features Rust provides specifically for writing tests that take these actions, which include the `test` attribute, a few macros, and the `should_panic` attribute.


_You can refer to the following chapter in the Rust Programming Language Book: [Writing Tests](https://doc.rust-lang.org/stable/book/ch11-01-writing-tests.html)_
