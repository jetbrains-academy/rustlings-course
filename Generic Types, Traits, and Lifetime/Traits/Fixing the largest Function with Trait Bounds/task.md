### Fixing the `largest` Function with Trait Bounds

Now that you know how to specify the behavior you want to use using the generic
type parameter’s bounds, let’s return to the [definition of
the `largest` function](course://Generic+Types,+Traits,+and+Lifetime/Generic+Data+Types/In+Function+Definitions) that uses a generic type parameter! Last time we tried
to run that code, we received this error:

```text
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ^^^^^^^^^^^^^^^^^^^^^^
```

In the body of `largest` we wanted to compare two values of type `T` using the
greater than (`>`) operator. Because that operator is defined as a default
method on the standard library trait `std::cmp::PartialOrd`, we need to specify
`PartialOrd` in the trait bounds for `T` so the `largest` function can work on
slices of any type that we can compare. We don’t need to bring `PartialOrd`
into scope because it’s in the prelude. Change the signature of `largest` to
look like this:

```rust,ignore
fn largest<T: PartialOrd>(list: &[T]) -> T {
```

This time when we compile the code, we get a different set of errors:

```console
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0508]: cannot move out of type `[T]`, a non-copy slice
 --> src/main.rs:2:23
  |
2 |     let mut largest = list[0];
  |                       ^^^^^^^
  |                       |
  |                       cannot move out of here
  |                       move occurs because `list[_]` has type `T`, which does not implement the `Copy` trait
  |                       help: consider borrowing here: `&list[0]`

error[E0507]: cannot move out of a shared reference
 --> src/main.rs:4:18
  |
4 |     for &item in list {
  |         -----    ^^^^
  |         ||
  |         |data moved here
  |         |move occurs because `item` has type `T`, which does not implement the `Copy` trait
  |         help: consider removing the `&`: `item`

Some errors have detailed explanations: E0507, E0508.
For more information about an error, try `rustc --explain E0507`.
error: could not compile `chapter10` due to 2 previous errors
```

The key line in this error is `cannot move out of type [T], a non-copy slice`.
With our non-generic versions of the `largest` function, we were only trying to
find the largest `i32` or `char`. As discussed in the “Stack-Only Data:
Copy” subsection in the [Clone and Copy](course://Understanding Ownership/What is ownership/Clone and Copy) task, types like
`i32` and `char` that have a known size can be stored on the stack, so they
implement the `Copy` trait. However, when we made the `largest` function generic,
it became possible for the `list` parameter to have types in it that don’t
implement the `Copy` trait. Consequently, we wouldn’t be able to move the
value out of `list[0]` and into the `largest` variable, resulting in this
error.

To call this code with only those types that implement the `Copy` trait, we can
add `Copy` to the trait bounds of `T`! The code snippet below shows the complete code of
a generic `largest` function that will compile as long as the types of the
values in the slice that we pass into the function implement the `PartialOrd`
*and* `Copy` traits, like `i32` and `char` do.


```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

#### A working definition of the `largest` function that works on any generic type that implements the `PartialOrd` and `Copy` traits</span>

If we don’t want to restrict the `largest` function to the types that implement
the `Copy` trait, we could specify that `T` has the trait bound `Clone` instead
of `Copy`. Then we could clone each value in the slice when we want the
`largest` function to have ownership. Using the `clone` function means we’re
potentially making more heap allocations in the case of types that own heap
data like `String`, and heap allocations can be slow if we’re working with
large amounts of data.

Another way we could implement `largest` is for the function to return a
reference to a `T` value in the slice. If we change the return type to `&T`
instead of `T`, thereby changing the body of the function to return a
reference, we wouldn’t need the `Clone` or `Copy` trait bounds and we could
avoid heap allocations. Try implementing these alternate solutions on your own!
