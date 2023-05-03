### Iterating over the Values in a Vector

If we want to access each element in a vector in turn, we can iterate through
all of the elements rather than use indices to access one at a time. The code
below shows how to use a `for` loop to get immutable references to each element
in a vector of `i32` values and print them.

```rust
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
```

#### Printing each element in a vector by iterating over the elements using a for loop

We can also iterate over mutable references to each element in a mutable vector
in order to make changes to all the elements. The `for` loop in the listing below
will add `50` to each element.

```rust
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
```

#### Iterating over mutable references to elements in a vector

To change the value that the mutable reference refers to, we have to use the
dereference operator (`*`) to get to the value in `i` before we can use the
`+=` operator. You can read more about the dereference operator in the
“Following the Pointer to the Value with the Dereference Operator”
section of Chapter 15 in the [Rust book][book].

[book]: https://doc.rust-lang.org/stable/book/ch15-02-deref.html?highlight=dereference#following-the-pointer-to-the-value
