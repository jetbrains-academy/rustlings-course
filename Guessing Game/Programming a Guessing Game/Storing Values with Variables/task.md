Next, we’ll create a place to store the user input, like this:

```rust
let mut guess = String::new();
```

Now the program is getting interesting! There’s a lot going on in this little line. Notice that this is a `let` statement, which is used to create a _variable_. Here’s another example:

```rust
let foo = bar;
```

This line creates a new variable named `foo` and binds it to the value of the `bar` variable. In Rust, variables are immutable by default. We’ll be discussing this concept in detail in the [“Variables and Mutability”](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html#variables-and-mutability) lesson. The following example shows how to use `mut` before the variable name to make a variable mutable:

```rust
let foo = 5; // immutable
let mut bar = 5; // mutable
```

>Note: The // syntax starts a comment that continues until the end of the line. Rust ignores everything in comments, which are discussed in more detail in Section 3.

Let’s return to the guessing game program. You now know that `let mut guess` will introduce a mutable variable named `guess`. On the other side of the equal sign `(=)` is the value that `guess` is bound to, which is the result of calling `String::new`, a function that returns a new instance of a `String`. `String` is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.

The `::` syntax in the `::new` line indicates that `new` is an _associated function_ of the `String` type. An associated function is implemented on a type, in this case `String`, rather than on a particular instance of a String. Some languages call this a _static method_.

This `new` function creates a new, empty string. You’ll find a `new` function on many types, because it’s a common name for a function that makes a new value of some kind.

To summarize, the `let mut guess = String::new();` line has created a mutable variable that is currently bound to a new, empty instance of a `String`. Whew!

Recall that we included the input/output functionality from the standard library with `use std::io;` on the first line of the program. Now we’ll call the `stdin` function from the io module:

```rust
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```

If we hadn’t listed the use `std::io` line at the beginning of the program, we could have written this function call as `std::io::stdin`. The `stdin` function returns an instance of `std::io::Stdin`, which is a type that represents a handle to the standard input for your terminal.

The next part of the code, `.read_line(&mut guess)`, calls the `read_line` method on the standard input handle to get input from the user. We’re also passing one argument to `read_line`: `&mut guess`.

The job of `read_line` is to take whatever the user types into standard input and place that into a string, so it takes that string as an argument. The string argument needs to be mutable so the method can change the string’s content by adding the user input.

The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references. You don’t need to know a lot of those details to finish this program. For now, all you need to know is that like variables, references are immutable by default. Hence, you need to write `&mut guess` rather than `&guess` to make it mutable.

### Handling Potential Failure with the Result Type

We’re not quite done with this line of code. Although what we’ve discussed so far is a single line of text, it’s only the first part of the single logical line of code. The second part is this method:

```rust
.expect("Failed to read line");
```

When you call a method with the `.foo()` syntax, it’s often wise to introduce a newline and other whitespace to help break up long lines. We could have written this code as:

```rust
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

However, one long line is difficult to read, so it’s best to divide it: two lines for two method calls. Now let’s discuss what this line does.

As mentioned earlier, `read_line` puts what the user types into the string we’re passing it, but it also returns a value—in this case, an `io::Result`. Rust has a number of types named `Result` in its standard library: a generic `Result` as well as specific versions for submodules, such as `io::Result`.

The `Result` types are _[enumerations](https://doc.rust-lang.org/stable/book/ch06-00-enums.html)_, often referred to as _enums_. An enumeration is a type that can have a fixed set of values, and those values are called the enum’s _variants_.

For `Result`, the variants are `Ok` or `Err`. The `Ok` variant indicates the operation was successful, and inside `Ok` is the successfully generated value. The `Err` variant means the operation failed, and `Err` contains information about how or why the operation failed.

The purpose of these `Result` types is to encode error-handling information. Values of the `Result` type, like values of any type, have methods defined on them. An instance of `io::Result` has an `expect` [method](https://doc.rust-lang.org/stable/std/result/enum.Result.html#method.expect) that you can call. If this instance of `io::Result` is an `Err` value, `expect` will cause the program to crash and display the message that you passed as an argument to `expect`. If the `read_line` method returns an `Err`, it would likely be the result of an error coming from the underlying operating system. If this instance of `io::Result` is an `Ok` value, `expect` will take the return value that `Ok` is holding and return just that value to you so you can use it. In this case, that value is the number of bytes in what the user entered into standard input.

If you don’t call `expect`, the program will compile, but you’ll get a warning:

```text
Compiling processing_a_guess v0.1.0 (/private/var/folders/jz/nfzs7rnd5zl_7r7x3t_8yvdh0000gp/T/rustlings-course/Programming a Guessing Game/task)
warning: unused `std::result::Result` that must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(unused_must_use)] on by default
   = note: this `Result` may be an `Err` variant, which should be handled
```

Rust warns that you haven’t used the `Result` value returned from `read_line`, indicating that the program hasn’t handled a possible error.

The right way to suppress the warning is to actually write error handling, but because you just want to crash this program when a problem occurs, you can use `expect`.

### Printing Values with println! Placeholders

Aside from the closing curly brackets, there’s only one more line to discuss in the code added so far, which is the following:

```rust
println!("You guessed: {}", guess);
```

This line prints the string we saved the user’s input in. The set of curly brackets, `{}`, is a placeholder: think of `{}` as little crab pincers that hold a value in place. You can print more than one value using curly brackets: the first set of curly brackets holds the first value listed after the format string, the second set holds the second value, and so on. Printing multiple values in one call to `println!` would look like this:

```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

This code would print `x = 5 and y = 10`.

### Testing the First Part

Let’s test the first part of the guessing game. Check the code by clicking **Check** button in Task Description window and run it by clicking the green play button in the left gutter:

```text
   Compiling processing_a_guess v0.1.0 (/private/var/folders/jz/nfzs7rnd5zl_7r7x3t_8yvdh0000gp/T/rustlings-course/Programming a Guessing Game/task)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/processing_a_guess`
Guess the number!
Please input your guess.
6
You guessed: 6
```

At this point, the first part of the game is done: we’re getting input from the keyboard and then printing it.

You can refer to the following chapters in the Rust Programming Language Book:_[Storing Values with Variables](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html#storing-values-with-variables) and [Handling Potential Failure with the Result Type](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-the-result-type)_