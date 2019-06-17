## Functions

Functions are pervasive in Rust code. You’ve already seen one of the most important functions in the language: the `main` function, which is the entry point of many programs. You’ve also seen the `fn` keyword, which allows you to declare new functions.

Rust code uses _snake case_ as the conventional style for function and variable names. In snake case, all letters are lowercase and underscores separate words. Here’s a program that contains an example function definition:

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

Function definitions in Rust start with `fn` and have a set of parentheses after the function name. The curly brackets tell the compiler where the function body begins and ends.

We can call any function we’ve defined by entering its name followed by a set of parentheses. Because `another_function` is defined in the program, it can be called from inside the `main` function. Note that we defined `another_function` _after_ the `main` function in the source code; we could have defined it before as well. Rust doesn’t care where you define your functions, only that they’re defined somewhere.

Let’s run the code from listing above to explore functions further. Place the another_function example in src/main.rs and run it. You should see the following output:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28 secs
     Running `target/debug/functions`
Hello, world!
Another function.
```

The lines execute in the order in which they appear in the `main` function. First, the “Hello, world!” message prints, and then `another_function` is called and its message is printed.

### Function Parameters
   
   Functions can also be defined to have _parameters_, which are special variables that are part of a function’s signature. When a function has parameters, you can provide it with concrete values for those parameters. Technically, the concrete values are called _arguments_, but in casual conversation, people tend to use the words _parameter_ and _argument_ interchangeably for either the variables in a function’s definition or the concrete values passed in when you call a function.
   
   The following rewritten version of `another_function` shows what parameters look like in Rust:
   
```rust
   fn main() {
       another_function(5);
   }
   
   fn another_function(x: i32) {
       println!("The value of x is: {}", x);
   }
```   

   Try running this program; you should get the following output:
   
```text
   $ cargo run
      Compiling functions v0.1.0 (file:///projects/functions)
       Finished dev [unoptimized + debuginfo] target(s) in 1.21 secs
        Running `target/debug/functions`
   The value of x is: 5
```
   
   The declaration of `another_function` has one parameter named `x`. The type of `x` is specified as `i32`. When `5` is passed to `another_function`, the `println!` macro puts `5` where the pair of curly brackets were in the format string.
   
   In function signatures, you _must_ declare the type of each parameter. This is a deliberate decision in Rust’s design: requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what you mean.
   
   When you want a function to have multiple parameters, separate the parameter declarations with commas, like this:
   
```rust
   fn main() {
       another_function(5, 6);
   }
   
   fn another_function(x: i32, y: i32) {
       println!("The value of x is: {}", x);
       println!("The value of y is: {}", y);
   }
```   
   This example creates a function with two parameters, both of which are `i32` types. The function then prints the values in both of its parameters. Note that function parameters don’t all need to be the same type, they just happen to be in this example.
   
   Let’s try running this code. Replace the program currently in your functions project’s src/main.rs file with the preceding example and run it:
   
```text
   $ cargo run
      Compiling functions v0.1.0 (file:///projects/functions)
       Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
        Running `target/debug/functions`
   The value of x is: 5
   The value of y is: 6
```

   Because we called the function with `5` as the value for `x` and `6` is passed as the value for `y`, the two strings are printed with these values.

_You can refer to the following chapter in the Rust Programming Language Book: [How Functions Work](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html)_