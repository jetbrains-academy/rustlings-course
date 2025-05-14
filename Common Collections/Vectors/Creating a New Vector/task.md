### Creating a New Vector

To create a new, empty vector, we can call the `Vec::new` function, as shown in
the code below.

```rust
    let v: Vec<i32> = Vec::new();
```

#### Creating a new, empty vector to hold values of type i32

Note that we added a type annotation here. Because we aren’t inserting any
values into this vector, Rust doesn’t know what kind of elements we intend to
store. This is an important point. Vectors are implemented using generics;
we’ll cover how to use generics with your own types in the [Generic Types, Traits, and Lifetime](course://Generic%20Types,%20Traits,%20and%20Lifetime) section. For now,
know that the `Vec<T>` type provided by the standard library can hold any type,
and when a specific vector holds a specific type, the type is specified within
angle brackets. In the code above, we’ve told Rust that the `Vec<T>` in `v` will
hold elements of the `i32` type.

In more realistic code, Rust can often infer the type of value you want to
store once you insert values, so you rarely need to do this type annotation.
It’s more common to create a `Vec<T>` that has initial values, and Rust
provides the `vec!` macro for convenience. The macro will create a new vector
that holds the values you give it. The listing below creates a new `Vec<i32>` that
holds the values `1`, `2`, and `3`. The integer type is `i32` because that’s
the default integer type, as we discussed in the "Basic Data Types" lesson of the [Common Programming Concepts](course://Common Programming Concepts) section.

```rust
    let v = vec![1, 2, 3];
```

#### Creating a new vector containing values

Because we’ve given initial `i32` values, Rust can infer that the type of `v`
is `Vec<i32>`, and the type annotation isn’t necessary. Next, we’ll look at how
to modify a vector.
