### Performance of Code Using Generics

You might be wondering whether there is a runtime cost when you’re using
generic type parameters. The good news is that Rust implements generics in such
a way that your code doesn’t run any slower using generic types than it would
with concrete types.

Rust accomplishes this by performing monomorphization of the code that is using
generics at compile time. *Monomorphization* is the process of turning generic
code into specific code by filling in the concrete types that are used when
compiled.

In this process, the compiler does the opposite of the steps we used to create
the generic function in the third code snippet in this section: the compiler looks at all the places
where generic code is called and generates code for the concrete types the
generic code is called with.

Let’s look at how this works with an example that uses the standard library’s
`Option<T>` enum:

```rust
let integer = Some(5);
let float = Some(5.0);
```

When Rust compiles this code, it performs monomorphization. During that
process, the compiler reads the values that have been used in `Option<T>`
instances and identifies two kinds of `Option<T>`: one is `i32` and the other
is `f64`. As such, it expands the generic definition of `Option<T>` into
`Option_i32` and `Option_f64`, thereby replacing the generic definition with
the specific ones.

The monomorphized version of the code looks like the following. The generic
`Option<T>` is replaced with the specific definitions created by the compiler:


```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

Because Rust compiles generic code into code that specifies the type in each
instance, we pay no runtime cost for using generics. When the code runs, it
performs just as it would if we had duplicated each definition by hand. The
process of monomorphization makes Rust’s generics extremely efficient at
runtime.

