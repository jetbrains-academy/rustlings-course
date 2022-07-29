### In Function Definitions

When defining a function that uses generics, we place the generics in the
signature of the function where we would usually specify the data types of the
parameters and return value. Doing so makes our code more flexible and provides
more functionality to callers of our function while preventing code duplication.

Continuing with our `largest` function, the listing below shows two functions that
both find the largest value in a slice.


```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
```

#### Two functions that differ only in their names and the types in their signatures

The `largest_i32` function is the one we extracted in the last code snippet in the previous section that finds
the largest `i32` in a slice. The `largest_char` function finds the largest
`char` in a slice. The function bodies have the same code, so let’s eliminate
the duplication by introducing a generic type parameter in a single function.

To parameterize the types in the new function we’ll define, we need to name the
type parameter, just as we do for the value parameters to a function. You can
use any identifier as a type parameter name. But we’ll use `T` because, by
convention, parameter names in Rust are short, often just a letter, and Rust’s
type-naming convention is CamelCase. Short for “type,” `T` is the default
choice of most Rust programmers.

When we use a parameter in the body of the function, we have to declare the
parameter name in the signature, so the compiler knows what that name means.
Similarly, when we use a type parameter name in a function signature, we have
to declare the type parameter name before we use it. To define the generic
`largest` function, place type name declarations inside angle brackets, `<>`,
between the name of the function and the parameter list, like this:

```rust,ignore
fn largest<T>(list: &[T]) -> &T {
```

We read this definition as: the function `largest` is generic over some type
`T`. This function has one parameter named `list`, which is a slice of values
of type `T`. The `largest` function will return a reference to a value of the
same type `T`.

The code snippet below shows the combined `largest` function definition using the generic
data type in its signature. The listing also shows how we can call the function
with either a slice of `i32` values or `char` values. Note that this code won’t
compile yet, but we’ll fix it later in this chapter.


```rust,ignore,does_not_compile
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
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

#### A definition of the `largest` function that uses generic type parameters but doesn’t compile yet

If we compile this code right now, we’ll get this error:

```console
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

The note mentions `std::cmp::PartialOrd`, which is a *trait*. We’ll talk about
traits in the next section. For now, this error states that the body of
`largest` won’t work for all possible types that `T` could be. Because we want
to compare values of type `T` in the body, we can only use types whose values
can be ordered. To enable comparisons, the standard library has the
`std::cmp::PartialOrd` trait that you can implement on types (see Appendix C
for more on this trait). You’ll learn how to specify that a generic type has a
particular trait in the “Traits as Parameters” part of the next lesson, but let’s first explore other ways of using generic type
parameters.
