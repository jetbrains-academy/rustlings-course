### Methods for Iterating Over Strings

Fortunately, you can access elements in a string in other ways.

If you need to perform operations on individual Unicode scalar values, the best way to do so is to use the `chars` method. Calling `chars` on “नमस्ते” separates out and returns six values of type `char`, and you can iterate over the result to access each element:

```rust
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
```

This code will print the following:

```text
    न
    म
    स
    ्
    त
    े
```

The `bytes` method returns each raw byte, which might be appropriate for your domain:

```rust
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
```

This code will print the 18 bytes that make up this `String`:

```text
    224
    164
    // --snip--
    165
    135
```

But be sure to remember that valid Unicode scalar values may be made up of more than 1 byte.

Getting grapheme clusters from strings is complex, so this functionality is not provided by the standard library. Crates are available on [crates.io](https://crates.io) if this is the functionality you need.
