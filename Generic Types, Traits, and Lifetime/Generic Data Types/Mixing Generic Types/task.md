### Mixing Generic Types

Generic type parameters in a struct definition aren’t always the same as those
you use in that struct’s method signatures. For example, the listing below defines
the method `mixup` on the `Point<T, U>` struct from earlier in this section. The method
takes another `Point` as a parameter, which might have different types from the
`self` `Point` we’re calling `mixup` on. The method creates a new `Point`
instance with the `x` value from the `self` `Point` (of type `T`) and the `y`
value from the passed-in `Point` (of type `W`).


```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

#### A method that uses different generic types from its struct’s definition

In `main`, we’ve defined a `Point` that has an `i32` for `x` (with value `5`)
and an `f64` for `y` (with value `10.4`). The `p2` variable is a `Point` struct
that has a string slice for `x` (with value `"Hello"`) and a `char` for `y`
(with value `c`). Calling `mixup` on `p1` with the argument `p2` gives us `p3`,
which will have an `i32` for `x`, because `x` came from `p1`. The `p3` variable
will have a `char` for `y`, because `y` came from `p2`. The `println!` macro
call will print `p3.x = 5, p3.y = c`.

The purpose of this example is to demonstrate a situation in which some generic
parameters are declared with `impl` and some are declared with the method
definition. Here, the generic parameters `T` and `U` are declared after `impl`,
because they go with the struct definition. The generic parameters `V` and `W`
are declared after `fn mixup`, because they’re only relevant to the method.
