
### Matching with `Option<T>`

In the previous section, we wanted to get the inner `T` value out of the `Some`
case when using `Option<T>`; we can also handle `Option<T>` using `match` as we
did with the `Coin` enum! Instead of comparing coins, we’ll compare the
variants of `Option<T>`, but the way that the `match` expression works remains
the same.

Let’s say we want to write a function that takes an `Option<i32>` and, if
there’s a value inside, adds 1 to that value. If there isn’t a value inside,
the function should return the `None` value and not attempt to perform any
operations.

This function is very easy to write, thanks to `match`, and will look like
the code below.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

#### A function that uses a `match` expression on an `Option<i32>`

Let’s examine the first execution of `plus_one` in more detail. When we call
`plus_one(five)`, the variable `x` in the body of `plus_one` will have the
value `Some(5)`. We then compare that against each match arm.

```rust,ignore
None => None,
```

The `Some(5)` value doesn’t match the pattern `None`, so we continue to the
next arm.

```rust,ignore
Some(i) => Some(i + 1),
```

Does `Some(5)` match `Some(i)`? Why, yes it does! We have the same variant. The
`i` binds to the value contained in `Some`, so `i` takes the value `5`. The
code in the match arm is then executed, so we add 1 to the value of `i` and
create a new `Some` value with our total `6` inside.

Now let’s consider the second call of `plus_one` in Listing 6-5, where `x` is
`None`. We enter the `match` and compare to the first arm.

```rust,ignore
None => None,
```

It matches! There’s no value to add to, so the program stops and returns the
`None` value on the right side of `=>`. Because the first arm matched, no other
arms are compared.

Combining `match` and enums is useful in many situations. You’ll see this
pattern a lot in Rust code: `match` against an enum, bind a variable to the
data inside, and then execute code based on it. It’s a bit tricky at first, but
once you get used to it, you’ll wish you had it in all languages. It’s
consistently a user favorite.
