## Recoverable Errors with Result

Most errors aren’t serious enough to require the program to stop entirely. Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to. For example, if you try to open a file and that operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process.

The `Result` enum is defined as having two variants, `Ok` and `Err`, as follows:

```rust
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
```

The `T` and `E` are generic type parameters: we’ll discuss generics in more detail in the lesson "Generic Types, Traits and Lifetime". What you need to know right now is that `T` represents the type of the value that will be returned in a success case within the `Ok` variant, and `E` represents the type of the error that will be returned in a failure case within the `Err` variant. Because `Result` has these generic type parameters, we can use the `Result` type and the functions that the standard library has defined on it in many different situations where the successful value and error value we want to return may differ.

Let’s call a function that returns a `Result` value because the function could fail. In the code snippet below we try to open a file.

```rust
    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt");
    }
```

##### Opening a file

How do we know `File::open` returns a `Result`? We could look at the [standard library API documentation](https://doc.rust-lang.org/std/index.html), or we could ask the compiler! If we give `f` a type annotation that we know is _not_ the return type of the function and then try to compile the code, the compiler will tell us that the types don’t match. The error message will then tell us what the type of `f` _is_. Let’s try it! We know that the return type of `File::open` isn’t of type `u32`, so let’s change the `let f` statement to this:

```rust
    let f: u32 = File::open("hello.txt");
```

Attempting to compile now gives us the following output:

```text
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |            ---   ^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found enum `std::result::Result`
  |            |
  |            expected due to this
  |
  = note: expected type `u32`
             found enum `std::result::Result<File, std::io::Error>`
```

This tells us the return type of the `File::open` function is a `Result<T, E>`. The generic parameter `T` has been filled in here with the type of the success value, `std::fs::File`, which is a file handle. The type of `E` used in the error value is `std::io::Error`.

This return type means the call to `File::open` might succeed and return a file handle that we can read from or write to. The function call also might fail: for example, the file might not exist, or we might not have permission to access the file. The `File::open` function needs to have a way to tell us whether it succeeded or failed and at the same time give us either the file handle or error information. This information is exactly what the `Result` enum conveys.

In the case where `File::open` succeeds, the value in the variable `f` will be an instance of `Ok` that contains a file handle. In the case where it fails, the value in `f` will be an instance of `Err` that contains more information about the kind of error that happened.

We need to add to the code in the code snippet above to take different actions depending on the value `File::open` returns. The following code snippet shows one way to handle the `Result` using a basic tool, the `match` expression that we discussed in ["The Match Operator"](course://Enums/Enums and Pattern Matching/The Match Operator).

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

##### Using a match expression to handle the Result variants that might be returned

Note that, like the `Option` enum, the `Result` enum and its variants have been brought into scope by the prelude, so we don’t need to specify `Result::` before the `Ok` and `Err` variants in the `match` arms.

Here we tell Rust that when the result is `Ok`, return the inner `file` value out of the `Ok` variant, and we then assign that file handle value to the variable `f`. After the `match`, we can use the file handle for reading or writing.

The other arm of the `match` handles the case where we get an `Err` value from `File::open`. In this example, we’ve chosen to call the `panic!` macro. If there’s no file named _hello.txt_ in our current directory and we run this code, we’ll see the following output from the `panic!` macro:

```text
thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:8:23
```

As usual, this output tells us exactly what has gone wrong.


