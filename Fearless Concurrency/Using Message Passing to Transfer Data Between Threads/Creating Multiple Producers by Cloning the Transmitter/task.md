### Creating Multiple Producers by Cloning the Transmitter

Earlier we mentioned that `mpsc` was an acronym for _multiple producer, single consumer_. Let’s put `mpsc` to use and expand the code in the previous snippet to create multiple threads that all send values to the same receiver. We can do so by cloning the transmitting half of the channel, as shown below:

```rust
    // --snip--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // --snip--
```

##### Sending multiple messages from multiple producers

This time, before we create the first spawned thread, we call `clone` on the sending end of the channel. This will give us a new sending handle we can pass to the first spawned thread. We pass the original sending end of the channel to a second spawned thread. This gives us two threads, each sending different messages to the receiving end of the channel.

When you run the code, your output should look something like this:

```text
    Got: hi
    Got: more
    Got: from
    Got: messages
    Got: for
    Got: the
    Got: thread
    Got: you
```

You might see the values in another order; it depends on your system. This is what makes concurrency interesting as well as difficult. If you experiment with `thread::sleep`, giving it various values in the different threads, each run will be more nondeterministic and create different output each time.

Now that we’ve looked at how channels work, let’s look at a different method of concurrency.

You can refer to the following chapter in the Rust Programming Language Book: _[Using Message Passing to Transfer Data Between Threads](https://doc.rust-lang.org/book/ch16-02-message-passing.html#using-message-passing-to-transfer-data-between-threads)_