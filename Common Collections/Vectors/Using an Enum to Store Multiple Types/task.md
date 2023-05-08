### Using an Enum to Store Multiple Types

At the beginning of this chapter, we said that vectors can only store values
that are the same type. This can be inconvenient; there are definitely use
cases for needing to store a list of items of different types. Fortunately, the
variants of an enum are defined under the same enum type, so when we need to
store elements of a different type in a vector, we can define and use an enum!

For example, say we want to get values from a row in a spreadsheet in which
some of the columns in the row contain integers, some floating-point numbers,
and some strings. We can define an enum whose variants will hold the different
value types, and then all the enum variants will be considered the same type:
that of the enum. Then we can create a vector that holds that enum and so,
ultimately, holds different types. We’ve demonstrated this below.

```rust
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```

#### Defining an enum to store values of different types in one vector

Rust needs to know what types will be in the vector at compile time so it knows
exactly how much memory on the heap will be needed to store each element. A
secondary advantage is that we can be explicit about what types are allowed in
this vector. If Rust allowed a vector to hold any type, there would be a chance
that one or more of the types would cause errors with the operations performed
on the elements of the vector. Using an enum plus a `match` expression means
that Rust will ensure at compile time that every possible case is handled, as
discussed in the chapter "Enums".

When you’re writing a program, if you don’t know the exhaustive set of types
the program will get at runtime to store in a vector, the enum technique won’t
work. Instead, you can use a trait object, which we’ll cover in Chapter 17 in the [Rust book][book].

Now that we’ve discussed some of the most common ways to use vectors, be sure
to review [the API documentation][vec-api] for all the many useful methods defined on
`Vec<T>` by the standard library. For example, in addition to `push`, a `pop`
method removes and returns the last element. 



[vec-api]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[book]: https://doc.rust-lang.org/stable/book/
