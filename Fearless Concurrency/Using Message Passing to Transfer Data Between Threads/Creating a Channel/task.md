### Creating a Channel

First, in the example below, we’ll create a channel but not do anything with it. Note that this won’t compile yet because Rust can’t tell what type of values we want to send over the channel.

```rust
    use std::sync::mpsc;

    fn main() {
        let (tx, rx) = mpsc::channel();
    }
```

##### Creating a channel and assigning the two halves to tx and rx

We create a new channel using the `mpsc::channel` function; `mpsc` stands for _multiple producer, single consumer_. In short, the way Rust’s standard library implements channels means a channel can have multiple _sending_ ends that produce values but only one _receiving_ end that consumes those values. Imagine multiple streams flowing together into one big river: everything sent down any of the streams will end up in one river at the end. We’ll start with a single producer for now, but we’ll add multiple producers when we get this example working.

The `mpsc::channel` function returns a tuple, the first element of which is the sending end and the second element is the receiving end. The abbreviations `tx` and `rx` are traditionally used in many fields for _transmitter_ and _receiver_ respectively, so we name our variables as such to indicate each end. We’re using a `let` statement with a pattern that destructures the tuples; we’ll discuss the use of patterns in `let` statements and destructuring in Chapter 18\. Using a `let` statement this way is a convenient approach to extract the pieces of the tuple returned by `mpsc::channel`.
