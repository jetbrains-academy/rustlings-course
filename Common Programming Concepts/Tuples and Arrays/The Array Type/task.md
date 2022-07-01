## The Array Type

Another way to have a collection of multiple values is with an _array_. Unlike a tuple, every element of an array must have the same type. Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length, like tuples.

In Rust, the values going into an array are written as a comma-separated list inside square brackets:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Arrays are useful when you want your data allocated on the stack rather than the heap (we will discuss the stack and the heap more in Chapter 4) or when you want to ensure you always have a fixed number of elements. An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size. If you’re unsure whether to use an array or a vector, you should probably use a vector. Chapter 8 discusses vectors in more detail.

An example of when you might want to use an array rather than a vector is in a program that needs to know the names of the months of the year. It’s very unlikely that such a program will need to add or remove months, so you can use an array because you know it will always contain 12 elements:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

You would write an array’s type by using square brackets, and within the brackets include the type of each element, a semicolon, and then the number of elements in the array, like so:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Here, `i32` is the type of each element. After the semicolon, the number `5` indicates the array contains five elements.

Writing an array’s type this way looks similar to an alternative syntax for initializing an array: if you want to create an array that contains the same value for each element, you can specify the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:

```rust
let a = [3; 5];
```

The array named `a` will contain `5` elements that will all be set to the value `3` initially. This is the same as writing `let a = [3, 3, 3, 3, 3];` but in a more concise way.

#### Accessing Array Elements

An array is a single chunk of memory allocated on the stack. You can access elements of an array using indexing, like this:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

In this example, the variable named `first` will get the value `1`, because that is the value at index `[0]` in the array. The variable named `second` will get the value `2` from index `[1]` in the array.

#### Invalid Array Element Access

What happens if you try to access an element of an array that is past the end of the array? Say you change the example to the following code, which will compile but exit with an error when it runs:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
```

Running this code using cargo run produces the following result:

```text
   Finished dev [unoptimized + debuginfo] target(s) in 0.05s
   Running `target/debug/Test_Rust_Project`
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:5:19
```

The compilation didn’t produce any errors, but the program resulted in a _runtime_ error and didn’t exit successfully. When you attempt to access an element using indexing, Rust will check that the index you’ve specified is less than the array length. If the index is greater than or equal to the array length, Rust will panic.

This is the first example of Rust’s safety principles in action. In many low-level languages, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed. Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing. Chapter 9 discusses more of Rust’s error handling.

_You can refer to the following chapter in the Rust Programming Language Book:[Compound Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#compound-types)_