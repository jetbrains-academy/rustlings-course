## The `match` Control Flow Operator

Rust has an extremely powerful control flow operator called `match` that allows
you to compare a value against a series of patterns and then execute code based
on which pattern matches. Patterns can be made up of literal values, variable
names, wildcards, and many other things; Chapter 18 of the [Rust Book][book]<!-- ignore --> covers all the different
kinds of patterns and what they do. The power of `match` comes from the
expressiveness of the patterns and the fact that the compiler confirms that all
possible cases are handled.

[book]: https://github.com/rust-lang/book/tree/master/src

Think of a `match` expression as being like a coin-sorting machine: coins slide
down a track with variously sized holes along it, and each coin falls through
the first hole it encounters that it fits into. In the same way, values go
through each pattern in a `match`, and at the first pattern the value “fits,”
the value falls into the associated code block to be used during execution.

Because we just mentioned coins, let’s use them as an example using `match`! We
can write a function that can take an unknown United States coin and, similarly 
to the counting machine, determine which coin it is and return its
value in cents, as shown here below.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

#### An enum and a `match` expression that has the variants of the enum as its patterns

Let’s break down the `match` in the `value_in_cents` function. First, we list
the `match` keyword followed by an expression, which in this case is the value
`coin`. This seems very similar to an expression used with `if`, but there’s a
big difference: with `if`, the expression needs to return a Boolean value, but
here, it can be any type. The type of `coin` in this example is the `Coin` enum
that we defined on line 1.

Next are the `match` arms. An arm has two parts: a pattern and some code. The
first arm here has a pattern that is the value `Coin::Penny` and then the `=>`
operator that separates the pattern and the code to run. The code in this case
is just the value `1`. Each arm is separated from the next with a comma.

When the `match` expression executes, it compares the resulting value against
the pattern of each arm, in order. If a pattern matches the value, the code
associated with that pattern is executed. If that pattern doesn’t match the
value, execution continues to the next arm, much as in a coin-sorting machine.
We can have as many arms as we need: in the code snippet above, our `match` has four arms.

The code associated with each arm is an expression, and the resulting value of
the expression in the matching arm is the value that gets returned for the
entire `match` expression.

Curly brackets typically aren’t used if the match arm code is short, as it is
in the code snippet above where each arm just returns a value. If you want to run multiple
lines of code in a match arm, you can use curly brackets. For example, the
following code would print “Lucky penny!” every time the method was called with
a `Coin::Penny` but would still return the last value of the block, `1`:

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```
