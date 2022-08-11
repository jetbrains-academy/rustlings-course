### Using Tuple Structs without Named Fields to Create Different Types

You can also define structs that look similar to tuples, called *tuple
structs*. Tuple structs have the added meaning the struct name provides but
don’t have names associated with their fields; rather, they just have the types
of the fields. Tuple structs are useful when you want to give the whole tuple a
name and make the tuple be a different type from other tuples, and naming each
field as in a regular struct would be verbose or redundant.

To define a tuple struct, start with the `struct` keyword and the struct name
followed by the types in the tuple. For example, here are definitions and
usages of two tuple structs named `Color` and `Point`:

```rust
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
```

Note that the `black` and `origin` values are different types, because they’re
instances of different tuple structs. Each struct you define is its own type,
even though the fields within the struct have the same types. For example, a
function that takes a parameter of type `Color` cannot take a `Point` as an
argument, even though both types are made up of three `i32` values. Otherwise,
tuple struct instances behave like tuples: you can destructure them into their
individual pieces, you can use a `.` followed by the index to access an
individual value, and so on.

### Unit-Like Structs Without Any Fields

You can also define structs that don’t have any fields! These are called
*unit-like structs* because they behave similarly to `()`, the unit type.
Unit-like structs can be useful in situations in which you need to implement a
trait on some type but don’t have any data that you want to store in the type
itself. We’ll discuss traits [later](course://Generic+Types,+Traits,+and Lifetime/Traits/Traits).
