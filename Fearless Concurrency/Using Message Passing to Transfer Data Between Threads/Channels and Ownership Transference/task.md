### Channels and Ownership Transference

The ownership rules play a vital role in message sending because they help you write safe, concurrent code. Preventing errors in concurrent programming is the advantage of thinking about ownership throughout your Rust programs. Let’s do an experiment to show how channels and ownership work together to prevent problems: we’ll try to use a `val` value in the spawned thread _after_ we’ve sent it down the channel. Try compiling the code in the next example to see why this code isn’t allowed:

```rust
    use std::thread;
    use std::sync::mpsc;

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
            println!("val is {}", val);
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }
```

##### Attempting to use val after we’ve sent it down the channel

Here, we try to print `val` after we’ve sent it down the channel via `tx.send`. Allowing this would be a bad idea: once the value has been sent to another thread, that thread could modify or drop it before we try to use the value again. Potentially, the other thread’s modifications could cause errors or unexpected results due to inconsistent or nonexistent data. However, Rust gives us an error if we try to compile the code in the example below:

```text
    error[E0382]: borrow of moved value: `val`
      --> src/main.rs:10:31
       |
    8  |         let val = String::from("hi");
       |             --- move occurs because `val` has type `String`, which does not implement the `Copy` trait
    9  |         tx.send(val).unwrap();
       |                 --- value moved here
    10 |         println!("val is {}", val);
       |                               ^^^ value borrowed here after move

```

Our concurrency mistake has caused a compile time error. The `send` function takes ownership of its parameter, and when the value is moved, the receiver takes ownership of it. This stops us from accidentally using the value again after sending it; the ownership system checks that everything is okay.
