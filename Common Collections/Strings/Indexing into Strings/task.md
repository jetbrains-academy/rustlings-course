### Indexing into Strings

In many other programming languages, accessing individual characters in a string by referencing them by index is a valid and common operation. However, if you try to access parts of a `String` using indexing syntax in Rust, you’ll get an error. Consider the invalid code in below.

```rust
    let s1 = String::from("hello");
    let h = s1[0];
```

##### Attempting to use indexing syntax with a String</span>

This code will result in the following error:

```text
error[E0277]: the type `String` cannot be indexed by `{integer}`
 --> src/main.rs:3:13
  |
3 |     let h = s1[0];
  |             ^^^^^ `String` cannot be indexed by `{integer}`
  |
  = help: the trait `Index<{integer}>` is not implemented for `String`
```

The error and the note tell the story: Rust strings don’t support indexing. But why not? To answer that question, we need to discuss how Rust stores strings in memory.
You can read about the details in [The Book](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#indexing-into-strings).
