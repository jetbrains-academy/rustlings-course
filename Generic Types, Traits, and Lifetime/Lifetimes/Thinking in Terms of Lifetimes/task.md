### Thinking in Terms of Lifetimes

The way in which you need to specify lifetime parameters depends on what your
function is doing. For example, if we changed the implementation of the
`longest` function to always return the first parameter rather than the longest
string slice, we wouldn’t need to specify a lifetime on the `y` parameter. The
following code will compile:


```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

In this example, we’ve specified a lifetime parameter `'a` for the parameter
`x` and the return type, but not for the parameter `y`, because the lifetime of
`y` does not have any relationship with the lifetime of `x` or the return value.

When returning a reference from a function, the lifetime parameter for the
return type needs to match the lifetime parameter for one of the parameters. If
the reference returned does *not* refer to one of the parameters, it must refer
to a value created within this function, which would be a dangling reference
because the value will go out of scope at the end of the function. Consider
this attempted implementation of the `longest` function that won’t compile:


```rust,ignore,does_not_compile
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

Here, even though we’ve specified a lifetime parameter `'a` for the return
type, this implementation will fail to compile because the return value
lifetime is not related to the lifetime of the parameters at all. Here is the
error message we get:

```console
error[E0515]: cannot return value referencing local variable `result`
  --> src/main.rs:11:5
   |
11 |     result.as_str()
   |     ------^^^^^^^^^
   |     |
   |     returns a value referencing data owned by the current function
   |     `result` is borrowed here
```

The problem is that `result` goes out of scope and gets cleaned up at the end
of the `longest` function. We’re also trying to return a reference to `result`
from the function. There is no way we can specify lifetime parameters that
would change the dangling reference, and Rust won’t let us create a dangling
reference. In this case, the best fix would be to return an owned data type
rather than a reference so the calling function is then responsible for
cleaning up the value.

Ultimately, lifetime syntax is about connecting the lifetimes of various
parameters and return values of functions. Once they’re connected, Rust has
enough information to allow memory-safe operations and disallow operations that
would create dangling pointers or otherwise violate memory safety.
