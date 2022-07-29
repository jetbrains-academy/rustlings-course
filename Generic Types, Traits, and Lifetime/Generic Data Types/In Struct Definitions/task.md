### In Struct Definitions

We can also define structs to use a generic type parameter in one or more
fields using the `<>` syntax. The code below shows how to define a `Point<T>`
struct to hold `x` and `y` coordinate values of any type.


```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

#### A `Point<T>` struct that holds `x` and `y` values of type `T`

The syntax for using generics in struct definitions is similar to that used in
function definitions. First, we declare the name of the type parameter inside
angle brackets just after the name of the struct. Then we can use the generic
type in the struct definition where we would otherwise specify concrete data
types.

Note that because we’ve used only one generic type to define `Point<T>`, this
definition says that the `Point<T>` struct is generic over some type `T`, and
the fields `x` and `y` are *both* that same type, whatever that type may be. If
we create an instance of a `Point<T>` that has values of different types, as it was done below, our code won’t compile.


```rust,ignore,does_not_compile
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}
```

#### The fields `x` and `y` must be the same type because both have the same generic data type `T`.

In this example, when we assign the integer value 5 to `x`, we let the
compiler know that the generic type `T` will be an integer for this instance of
`Point<T>`. Then when we specify 4.0 for `y`, which we’ve defined to have the
same type as `x`, we’ll get a type mismatch error like this:

```console
error[E0308]: mismatched types
 --> src/main.rs:7:38
  |
7 |     let wont_work = Point { x: 5, y: 4.0 };
  |                                      ^^^ expected integer, found floating-point number
```

To define a `Point` struct where `x` and `y` are both generics but could have
different types, we can use multiple generic type parameters. For example, in
the listing below, we can change the definition of `Point` to be generic over types
`T` and `U` where `x` is of type `T` and `y` is of type `U`.


```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

#### A `Point<T, U>` generic over two types so that `x` and `y` can be values of different types

Now all the instances of `Point` shown are allowed! You can use as many generic
type parameters in a definition as you want, but using more than a few makes
your code hard to read. When you need lots of generic types in your code, it
could indicate that your code needs restructuring into smaller pieces.
