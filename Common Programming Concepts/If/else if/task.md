### Handling Multiple Conditions with else if

You can have multiple conditions by combining `if` and `else` in an `else if` expression. For example:

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

This program has four possible paths it can take. After running it, you should see the following output:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
number is divisible by 3
```

When this program executes, it checks each `if` expression in turn and executes the first body for which the condition holds true. Note that even though 6 is divisible by 2, we don’t see the output 'number is divisible by 2', nor do we see the 'number is not divisible by 4, 3, or 2' text from the `else` block. That’s because Rust only executes the block for the first true condition, and once it finds one, it doesn’t even check the rest.

Using too many `else if` expressions can clutter your code, so if you have more than one, you might want to refactor your code. [Chapter 6](https://doc.rust-lang.org/stable/book/ch06-00-enums.html) describes a powerful Rust branching construct called `match` for these cases.

### Using if in a let Statement

Because `if` is an expression, we can use it on the right side of a `let` statement, as in Listing 3-2:

```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```
##### Example of assigning the result of an if expression to a variable

The `number` variable will be bound to a value based on the outcome of the `if` expression. Run this code to see what happens:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30 secs
     Running `target/debug/branches`
The value of number is: 5
```

Remember that blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions. In this case, the value of the whole `if` expression depends on which block of code executes. This means the values that have the potential to be results from each arm of the `if` must be the same type; in Listing 3-2, the results of both the `if` arm and the `else` arm were `i32` integers. If the types are mismatched, as in the following example, we’ll get an error:

```rust
fn main() {
    let condition = true;

    let number = if condition {
        5
    } else {
        "six"
    };

    println!("The value of number is: {}", number);
}
```

When we try to compile this code, we’ll get an error. The `if` and `else` arms have value types that are incompatible, and Rust indicates exactly where to find the problem in the program:

```text
error[E0308]: if and else have incompatible types
 --> src/main.rs:4:18
  |
4 |       let number = if condition {
  |  __________________^
5 | |         5
6 | |     } else {
7 | |         "six"
8 | |     };
  | |_____^ expected integral variable, found &str
  |
  = note: expected type `{integer}`
             found type `&str`
```

The expression in the `if` block evaluates to an integer, and the expression in the `else` block evaluates to a string. This won’t work because variables must have a single type. Rust needs to know at compile time what type the 'number' variable is, definitively, so it can verify at compile time that its type is valid everywhere we use 'number'. Rust wouldn’t be able to do that if the type of 'number' was only determined at runtime; the compiler would be more complex and would make fewer guarantees about the code if it had to keep track of multiple hypothetical types for any variable.

_You can refer to the following chapters in the Rust Programming Language Book: [Handling Multiple Conditions with else if](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#handling-multiple-conditions-with-else-if) and [Using if in a let Statement](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#using-if-in-a-let-statement)_