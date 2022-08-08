#### Sharing a Mutex<T> Between Multiple Threads

Now, let’s try to share a value between multiple threads using `Mutex<T>`. We’ll spin up 10 threads and have them each increment a counter value by 1, so the counter goes from 0 to 10\. The next example will have a compiler error, and we’ll use that error to learn more about using `Mutex<T>` and how Rust helps us use it correctly. The following code has our starting example:

```rust
    use std::sync::Mutex;
    use std::thread;

    fn main() {
        let counter = Mutex::new(0);
        let mut handles = vec![];

        for _ in 0..10 {
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

##### Ten threads each increment a counter guarded by a Mutex<T>

We create a `counter` variable to hold an `i32` inside a `Mutex<T>`, as we did in the first example. Next, we create 10 threads by iterating over a range of numbers. We use `thread::spawn` and give all the threads the same closure, one that moves the counter into the thread, acquires a lock on the `Mutex<T>` by calling the `lock` method, and then adds 1 to the value in the mutex. When a thread finishes running its closure, `num` will go out of scope and release the lock so another thread can acquire it.

In the main thread, we collect all the join handles. Then we call `join` on each handle to make sure all the threads finish. At that point, the main thread will acquire the lock and print the result of this program.

We hinted that this example wouldn’t compile. Now let’s find out why!

```text
error[E0382]: use of moved value: `counter`
  --> src/main.rs:9:36
   |
5  |     let counter = Mutex::new(0);
   |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
...
9  |         let handle = thread::spawn(move || {
   |                                    ^^^^^^^ value moved into closure here, in previous iteration of loop
10 |             let mut num = counter.lock().unwrap();
   |                           ------- use occurs due to use in closure
```

The error message states that the `counter` value was moved in the previous
iteration of the loop. So Rust is telling us that we can’t move the ownership
of lock `counter` into multiple threads. Let’s fix the compiler error with a
multiple-ownership method that is discussed in [Chapter 15](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) of the Rust Book.
