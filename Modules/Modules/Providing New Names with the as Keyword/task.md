### Providing New Names with the `as` Keyword

There’s another solution to the problem of bringing two types of the same name into the same scope with `use`: after the path, we can specify `as` and a new local name, or alias, for the type. The next example shows another way to write the code for importing by renaming one of the two `Result` types using `as`.

```rust
    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function1() -> Result {
    }

    fn function2() -> IoResult<()> {
    }
```

##### Renaming a type when it’s brought into scope with the as keyword

In the second `use` statement, we chose the new name `IoResult` for the `std::io::Result` type, which won’t conflict with the `Result` from `std::fmt` that we’ve also brought into scope. Both of the above examples are considered idiomatic, so the choice is up to you!