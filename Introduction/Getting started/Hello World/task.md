## Hello, World!

Now, let’s write your first Rust program. It’s traditional when learning a new language to write a little program that prints the text `Hello, world!` to the screen, so we’ll do the same here!

You can see the **Editor** window in the left. This is your sandbox and you can test your code here.
The `main.rs` file has the following lines:

```rust
fn main() {
    // put your code here to launch it
}
``` 
Now let's replace the line inside the main function with the code:

```rust
    println!("Hello, world!");
```
##### Example: A program that prints Hello, world!

To run this code click on **Run** icon at the left-upper corner of the editor. Click **Check** button in the **Task Description** window to run the tests checking the correctness of the solution provided.

If `Hello, world!` did print, congratulations! You’ve officially written a Rust program. That makes you a Rust programmer—welcome!

### Anatomy of a Rust Program

Let’s review in detail what just happened in your Hello, world! program. Here’s the first piece of the puzzle:

```rust

fn main() {

}
```

These lines define a function in Rust. The `main` function is special: it is always the first code that runs in every executable Rust program. The first line declares a function named `main` that has no parameters and returns nothing. If there were parameters, they would go inside the parentheses, `()`.

Also, note that the function body is wrapped in curly brackets, `{}`. Rust requires these around all function bodies. It’s good style to place the opening curly bracket on the same line as the function declaration, adding one space in between.

At the time of this writing, an automatic formatter tool called `rustfmt` is under development. If you want to stick to a standard style across Rust projects, `rustfmt` will format your code in a particular style. The Rust team plans to eventually include this tool with the standard Rust distribution, like `rustc`. So depending on when you read this book, it might already be installed on your computer! Check the online documentation for more details.

Inside the `main` function is the following code:

```rust
    println!("Hello, world!");
```

This line does all the work in this little program: it prints text to the screen. There are four important details to notice here. First, Rust style is to indent with four spaces, not a tab.

Second, println! calls a Rust macro. If it called a function instead, it would be entered as `println` (without the `!`). We’ll discuss Rust macros in more detail a bit later. For now, you just need to know that using a `!` means that you’re calling a macro instead of a normal function.

Third, you see the `"Hello, world!"` string. We pass this string as an argument to `println!`, and the string is printed to the screen.

Fourth, we end the line with a semicolon (`;`), which indicates that this expression is over and the next one is ready to begin. Most lines of Rust code end with a semicolon.

_You can refer to the following chapter in the Rust Programming Language book: [Hello, World!](https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html)_