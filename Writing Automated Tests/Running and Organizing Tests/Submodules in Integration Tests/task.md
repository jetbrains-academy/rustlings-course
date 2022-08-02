## Submodules in Integration Tests

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

