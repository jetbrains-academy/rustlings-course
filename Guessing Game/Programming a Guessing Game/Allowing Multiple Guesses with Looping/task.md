The 'loop' keyword creates an infinite loop. We’ll add that now to give users more chances at guessing the number:

```rust
// --snip--

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
```

As you can see, we’ve moved everything into a loop from the guess input prompt onward. Be sure to indent the lines inside the loop another four spaces each and run the program again. Notice that there is a new problem because the program is doing exactly what we told it to do: ask for another guess forever! It doesn’t seem like the user can quit!

The user could always interrupt the program by using the keyboard shortcut ⌘F2 or Ctrl F2. But there’s another way to escape this insatiable monster, as mentioned in the `parse` discussion in [“Comparing the Guess to the Secret Number”](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number): if the user enters a non-number answer, the program will crash. The user can take advantage of that in order to quit, as shown here:

```text
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/processing_a_guess`
Guess the number!
The secret number is: 83
Please input your guess.
11
You guessed: 11
Too small!
Please input your guess.
92
You guessed: 92
Too big!
Please input your guess.
quit
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/libcore/result.rs:997:5
Process finished with exit code 101
```

Typing `quit` actually quits the game, but so will any other non-number input. However, this is suboptimal to say the least. We want the game to automatically stop when the correct number is guessed.

### Quitting After a Correct Guess

Let’s program the game to quit when the user wins by adding a `break` statement:

```rust
// --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

Adding the `break` line after `You win!` makes the program exit the loop when the user guesses the secret number correctly. Exiting the loop also means exiting the program, because the loop is the last part of `main`.

### Handling Invalid Input

To further refine the game’s behavior, rather than crashing the program when the user inputs a non-number, let’s make the game ignore a non-number so the user can continue guessing. We can do that by altering the line where `guess` is converted from a `String` to a `u32`, as shown in Listing 2-5.

```rust
// --snip--

io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

println!("You guessed: {}", guess);

// --snip--
```

##### Listing 2-5: Ignoring a non-number guess and asking for another guess instead of crashing the program

Switching from an `expect` call to a `match` expression is how you generally move from crashing on an error to handling the error. Remember that `parse` returns a `Result` type and `Result` is an enum that has the variants `Ok` or `Err`. We’re using a `match` expression here, as we did with the `Ordering` result of the `cmp` method.

If `parse` is able to successfully turn the string into a number, it will return an `Ok` value that contains the resulting number. That `Ok` value will match the first arm’s pattern, and the `match` expression will just return the `num` value that `parse` produced and put inside the `Ok` value. That number will end up right where we want it in the new `guess` variable we’re creating.

If `parse` is not able to turn the string into a number, it will return an `Err` value that contains more information about the error. The `Err` value does not match the `Ok(num)` pattern in the first match arm, but it does match the `Err(_)` pattern in the second arm. The underscore,`_`, is a catchall value; in this example, we’re saying we want to match all `Err` values, no matter what information they have inside them. So the program will execute the second arm’s code, `continue`, which tells the program to go to the next iteration of the `loop` and ask for another guess. So, effectively, the program ignores all errors that `parse` might encounter!

Now everything in the program should work as expected. Let’s try it:

```text
    Finished dev [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/processing_a_guess`
Guess the number!
The secret number is: 42
Please input your guess.
33
You guessed: 33
Too small!
Please input your guess.
51
You guessed: 51
Too big!
Please input your guess.
42
You guessed: 42
You win!

Process finished with exit code 0
```

Awesome! With one tiny final tweak, we will finish the guessing game. Recall that the program is still printing the secret number. That worked well for testing, but it ruins the game. Let’s delete the println! that outputs the secret number. Listing 2-6 shows the final code.

```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

##### Listing 2-6: Complete guessing game code

### Summary

At this point, you’ve successfully built the guessing game. Congratulations!

This project was a hands-on way to introduce you to many new Rust concepts: `let`, `match`, methods, associated functions, the use of external crates, and more. In the next few chapters, you’ll learn about these concepts in more detail. Section 3 covers concepts that most programming languages have, such as variables, data types, and functions, and shows how to use them in Rust. Section 4 explores ownership, a feature that makes Rust different from other languages. Section 5 discusses structs and method syntax, and Section 6 explains how enums work.

_You can refer to the following chapter in the Rust Programming Language Book: [Allowing Multiple Guesses with Looping](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#allowing-multiple-guesses-with-looping)_