### Matching on Different Errors

The code in the next example will `panic!` no matter why `File::open` failed. What we want to do instead is take different actions for different failure reasons: if `File::open` failed because the file doesn’t exist, we want to create the file and return the handle to the new file. If `File::open` failed for any other reason—for example, because we didn’t have permission to open the file—we still want the code to `panic!` in the same way as it did in the previous snippet. Look at the following code, which adds an inner `match` expression.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
```

##### Handling different kinds of errors in different ways

The type of the value that `File::open` returns inside the `Err` variant is `io::Error`, which is a struct provided by the standard library. This struct has a method `kind` that we can call to get an `io::ErrorKind` value. The enum `io::ErrorKind` is provided by the standard library and has variants representing the different kinds of errors that might result from an `io` operation. The variant we want to use is `ErrorKind::NotFound`, which indicates the file we’re trying to open doesn’t exist yet. So we match on `f`, but we also have an inner match on `error.kind()`.

The condition we want to check in the inner match is whether the value returned by `error.kind()` is the `NotFound` variant of the `ErrorKind` enum. If it is, we try to create the file with `File::create`. However, because `File::create` could also fail, we need a second arm in the inner `match` expression. When the file can’t be created, a different error message is printed. The second arm of the outer `match` stays the same, so the program panics on any error besides the missing file error.

That’s a lot of `match`! The `match` expression is very useful but also very much a primitive. In ["Standard Library Types/Closures"](course://Standard Library Types/Closures), you’ll learn about closures; the `Result<T, E>` type has many methods that accept a closure and are implemented using `match` expressions. Using those methods will make your code more concise. A more seasoned Rustacean might write this code instead of the previous one:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

Although this code has the same behavior as the previous one, it doesn’t contain any `match` expressions and is cleaner to read. Come back to this example after you’ve read ["Standard Library Types/Closures"](course://Standard Library Types/Closures), and look up the `unwrap_or_else` method in the standard library documentation. Many more of these methods can clean up huge nested `match` expressions when you’re dealing with errors.
