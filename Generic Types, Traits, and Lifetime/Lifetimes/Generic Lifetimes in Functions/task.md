### Generic Lifetimes in Functions

Let’s write a function that returns the longer of two string slices. This
function will take two string slices and return a string slice. After we’ve
implemented the `longest` function, the code below should print `The
longest string is abcd`.


```rust,ignore
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

#### A `main` function that calls the `longest` function to find the longer of two string slices

Note that we want the function to take string slices, which are references,
because we don’t want the `longest` function to take ownership of its
parameters. Refer to the “String Slices as
Parameters” section in "Understanding Ownership"
for more discussion about why the parameters we use in the code snippet above are the
ones we want.

If we try to implement the `longest` function as shown below, it
won’t compile.


```rust,ignore,does_not_compile
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

#### An implementation of the `longest` function that returns the longer of two string slices but does not yet compile

Instead, we get the following error that talks about lifetimes:

```console
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ^^^^    ^^^^^^^     ^^^^^^^     ^^^
```

The help text reveals that the return type needs a generic lifetime parameter
on it because Rust can’t tell whether the reference being returned refers to
`x` or `y`. Actually, we don’t know either, because the `if` block in the body
of this function returns a reference to `x` and the `else` block returns a
reference to `y`!

When we’re defining this function, we don’t know the concrete values that will
be passed into this function, so we don’t know whether the `if` case or the
`else` case will execute. We also don’t know the concrete lifetimes of the
references that will be passed in, so we can’t look at the scopes as we did in
the second and third code snippets of this section to determine whether the reference we return will
always be valid. The borrow checker can’t determine this either, because it
doesn’t know how the lifetimes of `x` and `y` relate to the lifetime of the
return value. To fix this error, we’ll add generic lifetime parameters that
define the relationship between the references so the borrow checker can
perform its analysis.