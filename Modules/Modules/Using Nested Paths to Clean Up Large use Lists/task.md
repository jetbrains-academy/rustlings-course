### Using Nested Paths to Clean Up Large use Lists

If we’re using multiple items defined in the same crate or same module, listing each item on its own line can take up a lot of vertical space in our files. For example, these two `use` statements bring items from `std` into scope:

```rust
    use std::cmp::Ordering;
    use std::io;
    // ---snip---
```

Instead, we can use nested paths to bring the same items into scope in one line. We do this by specifying the common part of the path followed by two colons and a list of the parts of the paths that differ in curly brackets, as shown below:

```rust
    use std::{cmp::Ordering, io};
    // ---snip---
```

##### Specifying a nested path to bring multiple items with the same prefix into scope

In bigger programs, bringing many items into scope from the same crate or module using nested paths can significantly reduce the number of separate needed `use` statements!

We can use a nested path at any level in a path, which is useful when combining two `use` statements that share a subpath. For example, the snippet below shows two `use` statements: one that brings `std::io` into scope and one that brings `std::io::Write` into scope.

```rust
    use std::io;
    use std::io::Write;
```

##### Two use statements where one is a subpath of the other

The common part of these two paths is `std::io`, and that’s the complete first path. To merge these two paths into one `use` statement, we can use `self` in the nested path, as shown in the example below.

```rust
    use std::io::{self, Write};
```

##### Combining the paths into one use statement

This line brings `std::io` and `std::io::Write` into scope.
