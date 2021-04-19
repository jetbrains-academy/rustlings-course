## Variables and Mutability

Let's talk about simple variables.

```rust
let x = 5;
```

This is a `let` statement, which is used to create a *variable*. Here’s another example:

```rust
let foo = bar;
```

This line creates a new variable named `foo` and binds it to the value of the `bar` variable.

In Rust, by default variables are immutable. This is one of many nudges Rust gives you to write your code in a way that takes advantage of the safety and easy concurrency that Rust offers. However, you still have the option to make your variables mutable. Let’s explore how and why Rust encourages you to favor immutability and why sometimes you might want to opt out.

When a variable is immutable, once a value is bound to a name, you can’t change that value. Take a look at the main.rs file, its code won’t compile just yet:

```rust
fn main() {
       let x = 5;
       println!("The value of x is: {}", x);
       x = 6;
       println!("The value of x is: {}", x);
   }
```


Run the program by clicking **Run** button.

You should receive an error message, as shown in this output:

```text
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: make this binding mutable: `mut x`
3 |     println!("The value of x is: {}", x);
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
```

This example shows how the compiler helps you find errors in your programs. Even though compiler errors can be frustrating, they only mean your program isn’t safely doing what you want it to do yet; they do not mean that you’re not a good programmer! Experienced Rustaceans still get compiler errors.

The error message indicates that the cause of the error is that you `cannot assign twice to immutable variable x`, because you tried to assign a second value to the immutable x variable.

It’s important that we get compile-time errors when we attempt to change a value that we previously designated as immutable because this very situation can lead to bugs. If one part of our code operates on the assumption that a value will never change and another part of our code changes that value, it’s possible that the first part of the code won’t do what it was designed to do. The cause of this kind of bug can be difficult to track down after the fact, especially when the second piece of code changes the value only sometimes.

In Rust, the compiler guarantees that when you state that a value won’t change, it really won’t change. That means that when you’re reading and writing code, you don’t have to keep track of how and where a value might change. Your code is thus easier to reason through.

But mutability can be very useful. Variables are immutable only by default; you can make them mutable by adding `mut` in front of the variable name. In addition to allowing this value to change, `mut` conveys intent to future readers of the code by indicating that other parts of the code will be changing this variable value.

For example, let’s change _src/main.rs_ to the following:

```rust

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

When we run the program now, we get this:

```text
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/variables`
The value of x is: 5
The value of x is: 6
```

We’re allowed to change the value that `x` binds to from `5` to `6` when mut is used. In some cases, you’ll want to make a variable mutable because it makes the code more convenient to write than if it had only immutable variables.

There are multiple trade-offs to consider in addition to the prevention of bugs. For example, in cases where you’re using large data structures, mutating an instance in place may be faster than copying and returning newly allocated instances. With smaller data structures, creating new instances and writing in a more functional programming style may be easier to think through, so lower performance might be a worthwhile penalty for gaining that clarity.

_You can refer to the following chapter in the Rust Programming Language book: [Variables and Mutability](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html)_

Now let's go ahead to the practice task.