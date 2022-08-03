## Submodules in Integration Tests

As you add more integration tests, you might want to make more than one file in
the *tests* directory to help organize them; for example, you can group the
test functions by the functionality they’re testing. As mentioned earlier, each
file in the *tests* directory is compiled as its own separate crate.

Treating each integration test file as its own crate is useful to create
separate scopes that are more like the way end users will be using your crate.
However, this means files in the *tests* directory don’t share the same
behavior as files in *src* do, as you learned in "Modules and Macros/Modules" regarding how to
separate code into modules and files.

The different behavior of files in the *tests* directory is most noticeable
when you have a set of helper functions that would be useful in multiple
integration test files and you try to follow the steps in the “Separating
Modules into Different Files”
section of "Modules and Macros/Modules" to extract them into a common module. For example, if we
create *tests/common.rs* and place a function named `setup` in it, we can add
some code to `setup` that we want to call from multiple test functions in
multiple test files:


```rust
pub fn setup() {
    // setup code specific to your library's tests would go here
}
```

When we run the tests again, we’ll see a new section in the test output for the
*common.rs* file, even though this file doesn’t contain any test functions nor
did we call the `setup` function from anywhere:

```text
   Compiling test_organization v0.1.0
    Finished test [unoptimized + debuginfo] target(s) in 0.81s
     Running target/debug/deps/test_organization-61f5d8d60ccbcc19

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/common-b5e4eefa9d201089

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-5843d720c5feeb7a

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests test_organization

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Having `common` appear in the test results with `running 0 tests` displayed for
it is not what we wanted. We just wanted to share some code with the other
integration test files. You will learn how to avoid having `common` appear in the test output and to organize the tests properly in the next section.

To avoid having `common` appear in the test output, instead of creating
*tests/common.rs*, we’ll create *tests/common/mod.rs*. This is an alternate
naming convention that Rust also understands. Naming the file this way tells
Rust not to treat the `common` module as an integration test file. When we move
the `setup` function code into *tests/common/mod.rs* and delete the
*tests/common.rs* file, the section in the test output will no longer appear.
Files in subdirectories of the *tests* directory don’t get compiled as separate
crates or have sections in the test output.

After we’ve created *tests/common/mod.rs*, we can use it from any of the
integration test files as a module. Here’s an example of calling the `setup`
function from the `it_adds_two` test in *tests/integration_test.rs*:


```rust,ignore
use test_organization_part_2;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_organization_part_2::add_two(2));
}
```

Note that the `mod common;` declaration is the same as the module declaration
we demonstrated in the listing "Declaring the front_of_house module whose body will be in _src/front_of_house.rs" in the section "Separating Modules into Different Files" in "Modules". Then in the test function, we can call the
`common::setup()` function.

Output of `cargo test` after creating *tests/common/mod.rs* and calling the `setup`
function from the `it_adds_two` test in *tests/integration_test.rs*:

```text
Compiling submodules v0.1.0 
    Finished test [unoptimized + debuginfo] target(s) in 0.50s
     Running target/debug/deps/submodules-c44b35b673c8053d

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-31048908068047a2

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests submodules

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

##### Integration Tests for Binary Crates

If our project is a binary crate that only contains a *src/main.rs* file and
doesn’t have a *src/lib.rs* file, we can’t create integration tests in the
*tests* directory and bring functions defined in the *src/main.rs* file into
scope with a `use` statement. Only library crates expose functions that other
crates can use; binary crates are meant to be run on their own.

This is one of the reasons Rust projects that provide a binary have a
straightforward *src/main.rs* file that calls logic that lives in the
*src/lib.rs* file. Using that structure, integration tests *can* test the
library crate with `use` to make the important functionality available.
If the important functionality works, the small amount of code in the
*src/main.rs* file will work as well, and that small amount of code doesn’t
need to be tested.

