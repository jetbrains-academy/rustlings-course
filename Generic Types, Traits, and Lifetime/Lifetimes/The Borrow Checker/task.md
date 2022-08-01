### The Borrow Checker

The Rust compiler has a *borrow checker* that compares scopes to determine
whether all borrows are valid. The code listing below shows the same code as the previous one, but with annotations showing the lifetimes of the variables.

```rust,ignore,does_not_compile
{
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

#### Annotations of the lifetimes of `r` and `x`, named `'a` and `'b`, respectively

Here, we’ve annotated the lifetime of `r` with `'a` and the lifetime of `x`
with `'b`. As you can see, the inner `'b` block is much smaller than the outer
`'a` lifetime block. At compile time, Rust compares the size of the two
lifetimes and sees that `r` has a lifetime of `'a` but that it refers to memory
with a lifetime of `'b`. The program is rejected because `'b` is shorter than
`'a`: the subject of the reference doesn’t live as long as the reference.

The listing below fixes the code so it doesn’t have a dangling reference and
compiles without any errors.

```rust
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
```

#### A valid reference because the data has a longer lifetime than the reference

Here, `x` has the lifetime `'b`, which in this case is larger than `'a`. This
means `r` can reference `x` because Rust knows that the reference in `r` will
always be valid while `x` is valid.

Now that you know where the lifetimes of references are and how Rust analyzes
lifetimes to ensure references will always be valid, let’s explore generic
lifetimes of parameters and return values in the context of functions.
