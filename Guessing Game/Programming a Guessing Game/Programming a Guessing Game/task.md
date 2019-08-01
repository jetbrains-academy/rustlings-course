Let’s jump into Rust by working through a hands-on project together! This chapter introduces you to a few common Rust concepts by showing you how to use them in a real program. You’ll learn about `let`, `match`, methods, associated functions, using external crates, and more! The following lessons will explore these ideas in more detail. In this lesson, you’ll practice the fundamentals.

We’ll implement a classic beginner programming problem: a guessing game. Here’s how it works: the program will generate a random integer between 1 and 100. It will then prompt the player to enter a guess. After a guess is entered, the program will indicate whether the guess is too low or too high. If the guess is correct, the game will print a congratulatory message and exit.

### Getting a Guess

The first part of the guessing game program will ask for user input, process that input, and check that the input is in the expected form. To start, we’ll allow the player to input a guess.

```rust
use std::io;

fn main() {
    println!("Guess the number!");
}
```
##### Code that asks for a guess from the user

To obtain user input and then print the result as output, we need to bring the `io` (input/output) library into scope. The `io` library comes from the standard library (which is known as `std`):

```rust
use std::io;
```

By default, Rust brings only a few types into the scope of every program in _[the prelude](https://doc.rust-lang.org/stable/std/prelude/index.html)_. If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a `use` statement. Using the `std::io` library provides you with a number of useful features, including the ability to accept user input.

As you saw in Introduction Lesson, the main function is the entry point into the program:

```rust
fn main() {
```

The `fn` syntax declares a new function, the parentheses, `()`, indicate there are no parameters, and the curly bracket, `{`, starts the body of the function.

As you also learned in Lesson 1, `println!` is a macro that prints a string to the screen:

```rust
println!("Guess the number!");
```

This code is printing a prompt stating what the game is. Let's add one more message to ask the user for a guess. Fill in the placeholder in the _src/mains.rs_ to print two lines: "Guess the number!" and "Please input your guess."

_You can refer to the following chapter in the Rust Programming Language Book: [Processing a Guess](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html#processing-a-guess)_