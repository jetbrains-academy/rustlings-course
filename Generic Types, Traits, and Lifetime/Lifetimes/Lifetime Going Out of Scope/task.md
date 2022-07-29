### Lifetime: Going Out of Scope

Next, let’s try an example that shows that the lifetime of the reference in
`result` must be the smaller lifetime of the two arguments. We’ll move the
declaration of the `result` variable outside the inner scope but leave the
assignment of the value to the `result` variable inside the scope with
`string2`. Then we’ll move the `println!` that uses `result` outside the inner
scope, after the inner scope has ended. The code below will not
compile.

```rust,ignore,does_not_compile
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

#### Attempting to use `result` after `string2` has gone out of scope

When we try to compile this code, we’ll get this error:

```console
error[E0597]: `string2` does not live long enough
 --> src/main.rs:6:44
  |
6 |         result = longest(string1.as_str(), string2.as_str());
  |                                            ^^^^^^^ borrowed value does not live long enough
7 |     }
  |     - `string2` dropped here while still borrowed
8 |     println!("The longest string is {}", result);
  |                                          ------ borrow later used here
```

The error shows that for `result` to be valid for the `println!` statement,
`string2` would need to be valid until the end of the outer scope. Rust knows
this because we annotated the lifetimes of the function parameters and return
values using the same lifetime parameter `'a`.

As humans, we can look at this code and see that `string1` is longer than
`string2` and therefore `result` will contain a reference to `string1`.
Because `string1` has not gone out of scope yet, a reference to `string1` will
still be valid for the `println!` statement. However, the compiler can’t see
that the reference is valid in this case. We’ve told Rust that the lifetime of
the reference returned by the `longest` function is the same as the smaller of
the lifetimes of the references passed in. Therefore, the borrow checker
disallows the code in the previous listing as possibly having an invalid reference.

Try designing more experiments that vary the values and lifetimes of the
references passed in to the `longest` function and how the returned reference
is used. Make hypotheses about whether or not your experiments will pass the
borrow checker before you compile; then check to see if you’re right!
