### Associated Functions

Another useful feature of `impl` blocks is that we’re allowed to define
functions within `impl` blocks that *don’t* take `self` as a parameter. These
are called *associated functions* because they’re associated with the struct.
They’re still functions, not methods, because they don’t have an instance of
the struct to work with. You’ve already used the `String::from` associated
function.

Associated functions are often used for constructors that will return a new
instance of the struct. For example, we could provide an associated function
that would have one dimension parameter and use that as both width and height,
thus making it easier to create a square `Rectangle` rather than having to
specify the same value twice:

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

To call this associated function, we use the `::` syntax with the struct name;
`let sq = Rectangle::square(3);` is an example. This function is namespaced by
the struct: the `::` syntax is used for both associated functions and
namespaces created by modules. We’ll discuss modules in the chapter "Modules".

### Multiple `impl` Blocks

Each struct is allowed to have multiple `impl` blocks. For example, the listing
"Implementing the `can_hold` method on `Rectangle` that takes another `Rectangle` instance as a parameter" is equivalent to the code shown below, which has each method
in its own `impl` block.

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

#### Rewriting the listing "Implementing the `can_hold` method on `Rectangle` that takes another `Rectangle` instance as a parameter" using multiple `impl` blocks

There’s no reason to separate these methods into multiple `impl` blocks here,
but this is valid syntax. We’ll see a case in which multiple `impl` blocks are
useful in the chapter "Generic Types, Traits and Lifetime".
