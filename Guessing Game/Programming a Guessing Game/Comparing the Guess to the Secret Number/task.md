Now that we have user input and a random number, we can compare them. That step is shown in Listing 2-4. Note that this code won’t compile quite yet, as we will explain.

```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // ---snip---

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

##### Listing 2-4: Handling the possible return values of comparing two numbers

The first new bit here is another `use` statement, bringing a type called `std::cmp::Ordering` into scope from the standard library. Like `Result`, `Ordering` is another enum, but the variants for `Ordering` are `Less`, `Greater`, and `Equal`. These are the three outcomes that are possible when you compare two values.

Then we add five new lines at the bottom that use the `Ordering` type. The `cmp` method compares two values and can be called on anything that can be compared. It takes a reference to whatever you want to compare with: here it’s comparing the `guess` to the `secret_number`. Then it returns a variant of the `Ordering` enum we brought into scope with the `use` statement. We use a `match` expression to decide what to do next based on which variant of `Ordering` was returned from the call to `cmp` with the values in `guess` and `secret_number`.

A `match` expression is made up of _arms_. An arm consists of a _pattern_ and the code that should be run if the value given to the beginning of the `match` expression fits that arm’s pattern. Rust takes the value given to `match` and looks through each arm’s pattern in turn. The `match` construct and patterns are powerful features in Rust that let you express a variety of situations your code might encounter and make sure that you handle them all. 

Let’s walk through an example of what would happen with the `match` expression used here. Say that the user has guessed 50 and the randomly generated secret number this time is 38. When the code compares 50 to 38, the `cmp` method will return `Ordering::Greater`, because 50 is greater than 38. The `match` expression gets the `Ordering::Greater` value and starts checking each arm’s pattern. It looks at the first arm’s pattern, `Ordering::Less`, and sees that the value `Ordering::Greater` does not match `Ordering::Less`, so it ignores the code in that arm and moves to the next arm. The next arm’s pattern, `Ordering::Greater`, does match `Ordering::Greater`! The associated code in that arm will execute and print `Too big!` to the screen. The match expression ends because it has no need to look at the last arm in this scenario.

However, the code in Listing 2-4 won’t compile yet. Let’s try it:

```text
error[E0308]: mismatched types
  --> src/main.rs:21:21
   |
21 |     match guess.cmp(&secret_number) {
   |                     ^^^^^^^^^^^^^^ expected struct `std::string::String`, found integer
   |
   = note: expected type `&std::string::String`
              found type `&{integer}`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: Could not compile `processing_a_guess`
```

The core of the error states that there are _mismatched types_. Rust has a strong, static type system. However, it also has type inference. When we wrote `let mut guess = String::new()`, Rust was able to infer that `guess` should be a `String` and didn’t make us write the type. The `secret_number`, on the other hand, is a number type. A few number types can have a value between 1 and 100: `i32`, a 32-bit number; `u32`, an unsigned 32-bit number; `i64`, a 64-bit number; as well as others. Rust defaults to an `i32`, which is the type of `secret_number` unless you add type information elsewhere that would cause Rust to infer a different numerical type. The reason for the error is that Rust cannot compare a string and a number type.

Ultimately, we want to convert the `String` the program reads as input into a real number type so we can compare it numerically to the secret number. We can do that by adding the following two lines to the `main` function body:

```rust
// --snip--

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

The two new lines are:

```rust
let guess: u32 = guess.trim().parse()
    .expect("Please type a number!");
```

We create a variable named `guess`. But wait, doesn’t the program already have a variable named `guess`? It does, but Rust allows us to _shadow_ the previous value of guess with a new one. This feature is often used in situations in which you want to convert a value from one type to another type. Shadowing lets us reuse the `guess` variable name rather than forcing us to create two unique variables, such as `guess_str` and `guess` for example. (Section 3 covers shadowing in more detail.)

We bind `guess` to the expression `guess.trim().parse()`. The `guess` in the expression refers to the original `guess` that was a String with the input in it. The `trim` method on a String instance will eliminate any whitespace at the beginning and end. Although `u32` can contain only numerical characters, the user must press enter to satisfy `read_line`. When the user presses enter, a newline character is added to the string. For example, if the user types 5 and presses enter, `guess` looks like this: `5\n`. The `\n` represents “newline,” the result of pressing enter. The `trim` method eliminates `\n`, resulting in just `5`.

The `parse` [method on strings](https://doc.rust-lang.org/stable/std/primitive.str.html#method.parse) parses a string into some kind of number. Because this method can parse a variety of number types, we need to tell Rust the exact number type we want by using `let guess: u32`. The colon `(:)` after `guess` tells Rust we’ll annotate the variable’s type. Rust has a few built-in number types; the `u32` seen here is an unsigned, 32-bit integer. It’s a good default choice for a small positive number. You’ll learn about other number types in Section 3. Additionally, the `u32` annotation in this example program and the comparison with `secret_number` means that Rust will infer that `secret_number` should be a `u32` as well. So now the comparison will be between two values of the same type!

The call to `parse` could easily cause an error. If, for example, the string contained `A★%`, there would be no way to convert that to a number. Because it might fail, the `parse` method returns a `Result` type, much as the `read_line` method does (discussed earlier in [“Handling Potential Failure with the Result Type”](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-the-result-type)). We’ll treat this `Result` the same way by using the `expect` method again. If `parse` returns an `Err` `Result` variant because it couldn’t create a number from the string, the `expect` call will crash the game and print the message we give it. If `parse` can successfully convert the string to a number, it will return the `Ok` variant of `Result`, and `expect` will return the number that we want from the `Ok` value.

Let’s run the program now!

```text
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/processing_a_guess`
Guess the number!
The secret number is: 93
Please input your guess.
99
You guessed: 99
Too big!
```

Nice! Even though spaces were added before the guess, the program still figured out that the user guessed 76. Run the program a few times to verify the different behavior with different kinds of input: guess the number correctly, guess a number that is too high, and guess a number that is too low.

We have most of the game working now, but the user can make only one guess. Let’s change that by adding a loop!

_You can refer to the following chapter in the Rust Programming Language Book:
[Generating a Secret Number](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html#generating-a-secret-number)_