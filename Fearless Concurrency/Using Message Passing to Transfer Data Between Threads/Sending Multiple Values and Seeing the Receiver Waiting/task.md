### Sending Multiple Values and Seeing the Receiver Waiting

The code in the snippet about receiving "hi" in the main thread compiled and ran, but it didn’t clearly show us that two separate threads were talking to each other over the channel. In the following snippet we’ve made some modifications that will prove that code is running concurrently: the spawned thread will now send multiple messages and pause for a second between each message.

```rust
    use std::thread;
    use std::sync::mpsc;
    use std::time::Duration;

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }
```

##### Sending multiple messages and pausing between each

This time, the spawned thread has a vector of strings that we want to send to the main thread. We iterate over them, sending each individually, and pause between each by calling the `thread::sleep` function with a `Duration` value of 1 second.

In the main thread, we’re not calling the `recv` function explicitly anymore: instead, we’re treating `rx` as an iterator. For each value received, we’re printing it. When the channel is closed, iteration will end.

When running the code in the last example, you should see the following output with a 1-second pause in between each line:

```text
    Got: hi
    Got: from
    Got: the
    Got: thread
```

Because we don’t have any code that pauses or delays in the `for` loop in the main thread, we can tell that the main thread is waiting to receive values from the spawned thread.
