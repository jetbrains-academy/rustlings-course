### Ownership of Struct Data

In the `User` struct definition in the first code snippet of this section, we used the owned `String`
type rather than the `&str` string slice type. This is a deliberate choice
because we want instances of this struct to own all of its data and for that
data to be valid for as long as the entire struct is valid.

It’s possible for structs to store references to data owned by something else,
but to do so requires the use of *lifetimes*, a Rust feature that we’ll
discuss in the chapter "Generic Types, Traits and Lifetime". Lifetimes ensure that the data referenced by a struct
is valid for as long as the struct is. Let’s say you try to store a reference
in a struct without specifying lifetimes, like this, which won’t work:


```rust,ignore,does_not_compile
 struct User {
     username: &str,
     email: &str,
     sign_in_count: u64,
     active: bool,
 }

 fn main() {
     let user1 = User {
         email: "someone@example.com",
         username: "someusername123",
         active: true,
         sign_in_count: 1,
     };
 }
```

 The compiler will complain that it needs lifetime specifiers:

```console
 $ cargo run
    Compiling structs v0.1.0 (file:///projects/structs)
 error[E0106]: missing lifetime specifier
  --> src/main.rs:2:15
   |
 2 |     username: &str,
   |               ^ expected named lifetime parameter
   |
 help: consider introducing a named lifetime parameter
   |
 1 | struct User<'a> {
 2 |     username: &'a str,
   |

 error[E0106]: missing lifetime specifier
  --> src/main.rs:3:12
   |
 3 |     email: &str,
   |            ^ expected named lifetime parameter
   |
 help: consider introducing a named lifetime parameter
   |
 1 | struct User<'a> {
 2 |     username: &str,
 3 |     email: &'a str,
   |

 error: aborting due to 2 previous errors

 For more information about this error, try `rustc --explain E0106`.
 error: could not compile `structs`

 To learn more, run the command again with --verbose.
```

 In the chapter "Generic Types, Traits and Lifetime", we’ll discuss how to fix these errors, so you can store
 references in structs, but for now, we’ll fix errors like these using owned
 types like `String` instead of references like `&str`.


