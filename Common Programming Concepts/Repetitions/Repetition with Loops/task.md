## Repetition with Loops

It’s often useful to execute a block of code more than once. For this task, Rust provides several _loops_. A loop runs through the code inside the loop body to the end and then starts immediately back at the beginning. 

Rust has three kinds of loops: `loop`, `while`, and `for`. Let’s try each one.

### Repeating Code with loop

The `loop` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.

As an example, change the src/main.rs file to look like this:

```rust
fn main() {
     loop {
         println!("again!");
     }
 }
```

When we run this program, we’ll see `again!` printed over and over continuously
until we stop the program manually. Most terminals support a keyboard shortcut,
<span class="keystroke">ctrl-c</span>, to interrupt a program that is stuck in
a continual loop:

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```
The symbol `^C` represents where you pressed <span class="keystroke">ctrl-c
</span>. You may or may not see the word `again!` printed after the `^C`,
depending on where the code was in the loop when it received the interrupt
signal.

Fortunately, Rust provides another, more reliable way to break out of a loop. You can place the `break` keyword within the loop to tell the program when to stop executing the loop.

### Returning Values from Loops

One of the uses of a `loop` is to retry an operation you know might fail, such as checking whether a thread has completed its job. However, you might need to pass the result of that operation to the rest of your code. To do this, you can add the value you want returned after the `break` expression you use to stop the loop; that value will be returned out of the loop so you can use it, as shown here:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

Before the loop, we declare a variable named `counter` and initialize it to `0`. Then we declare a variable named `result` to hold the value returned from the loop. On every iteration of the loop, we add `1` to the `counter` variable, and then check whether the counter is equal to `10`. When it is, we use the `break` keyword with the value `counter * 2`. After the loop, we use a semicolon to end the statement that assigns the value to `result`. Finally, we print the value in `result`, which in this case is 20.




_You can refer to the following chapter in the Rust Programming Language Book: [Repetition with Loops](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#repetition-with-loops)_