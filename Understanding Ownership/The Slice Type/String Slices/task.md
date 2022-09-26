## String Slices

A _string slice_ is a reference to part of a `String`, and it looks like this:

```rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```

This is similar to taking a reference to the whole `String` but with the extra `[0..5]` bit. Rather than a reference to the entire `String`, it’s a reference to a portion of the `String`.

We can create slices using a range within brackets by specifying `[starting_index..ending_index]`, where `starting_index` is the first position in the slice and `ending_index` is the last position in the slice + 1. Internally, the slice data structure stores the starting position and the length of the slice, which corresponds to `ending_index` minus `starting_index`. So in the case of `let world = &s[6..11];`, `world` would be a slice that contains a pointer to the 7th byte (counting from 1) of `s` with a length value of 5.

Figure 6 shows this in a diagram.

<img alt="world containing a pointer to the 6th byte of String s and a length 5" src="https://doc.rust-lang.org/stable/book/img/trpl04-06.svg" class="center" style="width: 50%;">

##### Figure 6: String slice referring to part of a String

With Rust’s `..` range syntax, if you want to start at the first index (zero), you can drop the value before the two periods. In other words, these are equal:

```rust
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];
```

By the same token, if your slice includes the last byte of the `String`, you can drop the trailing number. That means these are equal:

```rust
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];
```

You can also drop both values to take a slice of the entire string. So these are equal:

```rust
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];
```

> Note: String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error. For the purposes of introducing string slices, we are assuming ASCII only in this section; a more thorough discussion of UTF-8 handling is in the [“Storing UTF-8 Encoded Text with Strings”](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings) section of the Chapter "Common Collections".

With all this information in mind, let’s rewrite `first_word` to return a slice. The type that signifies “string slice” is written as `&str`:

```rust
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
```

We get the index for the end of the word in the same way as we did in the first code snippet, by looking for the first occurrence of a space. When we find a space, we return a string slice using the start of the string and the index of the space as the starting and ending indices.

Now when we call `first_word`, we get back a single value that is tied to the underlying data. The value is made up of a reference to the starting point of the slice and the number of elements in the slice.

Returning a slice would also work for a `second_word` function:

```rust
    fn second_word(s: &String) -> &str {
```

We now have a straightforward API that’s much harder to mess up, because the compiler will ensure the references into the `String` remain valid. Remember the bug in the program in the "Storing the result from calling the first_word function and then changing the String contents" example, when we got the index to the end of the first word but then cleared the string so our index was invalid? That code was logically incorrect but didn’t show any immediate errors. The problems would show up later if we kept trying to use the first word index with an emptied string. Slices make this bug impossible and let us know we have a problem with our code much sooner. Using the slice version of `first_word` will throw a compile-time error:

```rust
    fn main() {
        let mut s = String::from("hello world");

        let word = first_word(&s);

        s.clear(); // error!

        println!("the first word is: {}", word);
    }
```

Here’s the compiler error:

```text
    error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
      --> src/main.rs:18:5
       |
    16 |     let word = first_word(&s);
       |                           -- immutable borrow occurs here
    17 |
    18 |     s.clear(); // error!
       |     ^^^^^^^^^ mutable borrow occurs here
    19 |
    20 |     println!("the first word is: {}", word);
       |                                       ---- immutable borrow later used here
```

Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference. Because `clear` needs to truncate the `String`, it needs to get a mutable reference. Rust disallows this, and compilation fails. Not only has Rust made our API easier to use, but it has also eliminated an entire class of errors at compile time!

