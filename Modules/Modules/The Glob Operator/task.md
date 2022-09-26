### The Glob Operator

If we want to bring _all_ public items defined in a path into scope, we can specify that path followed by `*`, the glob operator:

```rust
    use std::collections::*;
```

This `use` statement brings all public items defined in `std::collections` into the current scope. Be careful when using the glob operator! Glob can make it harder to tell what names are in scope and where a name used in your program was defined.

The glob operator is often used when testing to bring everything under test into the `tests` module; we’ll talk about that in the section [“How to Write Tests”](https://doc.rust-lang.org/stable/book/ch11-01-writing-tests.html#how-to-write-tests) in Chapter 11. The glob operator is also sometimes used as part of the prelude pattern: see [the standard library documentation](https://doc.rust-lang.org/std/prelude/index.html#other-preludes) for more information on that pattern.


_You can refer to the following chapter in the Rust Programming Language Book: [Bringing Paths into Scope with the use Keyword](https://doc.rust-lang.org/stable/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#bringing-paths-into-scope-with-the-use-keyword)_