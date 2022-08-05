#### Atomic Reference Counting with `Arc<T>`

Fortunately, `Arc<T>` *is* a type like `Rc<T>` that is safe to use in
concurrent situations. The *a* stands for *atomic*, meaning it’s an *atomically
reference counted* type. Atomics are an additional kind of concurrency
primitive that we won’t cover in detail here: see the standard library
documentation for [`std::sync::atomic`] for more details. At this point, you just
need to know that atomics work like primitive types but are safe to share
across threads.

[`std::sync::atomic`]: https://doc.rust-lang.org/std/sync/atomic/

You might then wonder why all primitive types aren’t atomic and why standard
library types aren’t implemented to use `Arc<T>` by default. The reason is that
thread safety comes with a performance penalty that you only want to pay when
you really need to. If you’re just performing operations on values within a
single thread, your code can run faster if it doesn’t have to enforce the
guarantees atomics provide.

Let’s return to our example: `Arc<T>` and `Rc<T>` have the same API, so we fix
our program by changing the `use` line, the call to `new`, and the call to
`clone`. The code below will finally compile and run:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

##### Using an Arc<T> to wrap the Mutex<T> to be able to share ownership across multiple threads

This code will print the following:

```text
    Result: 10
```

We did it! We counted from 0 to 10, which may not seem very impressive, but it did teach us a lot about `Mutex<T>` and thread safety. You could also use this program’s structure to do more complicated operations than just incrementing a counter. Using this strategy, you can divide a calculation into independent parts, split those parts across threads, and then use a `Mutex<T>` to have each thread update the final result with its part.
