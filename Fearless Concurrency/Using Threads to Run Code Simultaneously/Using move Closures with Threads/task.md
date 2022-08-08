### Using move Closures with Threads

The `move` closure is often used alongside `thread::spawn` because it allows you to use data from one thread in another thread.

In Chapter 13, we mentioned we can use the `move` keyword before the parameter list of a closure to force the closure to take ownership of the values it uses in the environment. This technique is especially useful when creating new threads in order to transfer ownership of values from one thread to another.

Notice in the snippet about creating a thread that the closure we pass to `thread::spawn` takes no arguments: we’re not using any data from the main thread in the spawned thread’s code. To use data from the main thread in the spawned thread, the spawned thread’s closure must capture the values it needs. The code snippet below shows an attempt to create a vector in the main thread and use it in the spawned thread. However, this won’t yet work, as you’ll see in a moment.

```rust
    use std::thread;

    fn main() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(|| {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }
```

##### Attempting to use a vector created by the main thread in another thread

The closure uses `v`, so it will capture `v` and make it part of the closure’s environment. Because `thread::spawn` runs this closure in a new thread, we should be able to access `v` inside that new thread. But when we compile this example, we get the following error:

```text
    error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
     --> src/main.rs:6:32
      |
    6 |     let handle = thread::spawn(|| {
      |                                ^^ may outlive borrowed value `v`
    7 |         println!("Here's a vector: {:?}", v);
      |                                           - `v` is borrowed here
      |
    note: function requires argument type to outlive `'static`
     --> src/main.rs:6:18
      |
    6 |       let handle = thread::spawn(|| {
      |  __________________^
    7 | |         println!("Here's a vector: {:?}", v);
    8 | |     });
      | |______^
    help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
      |
    6 |     let handle = thread::spawn(move || {
      |                                ^^^^^^^
```

Rust _infers_ how to capture `v`, and because `println!` only needs a reference to `v`, the closure tries to borrow `v`. However, there’s a problem: Rust can’t tell how long the spawned thread will run, so it doesn’t know if the reference to `v` will always be valid.

The next example provides a scenario that’s more likely to have a reference to `v` that won’t be valid:

```rust
    use std::thread;

    fn main() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(|| {
            println!("Here's a vector: {:?}", v);
        });

        drop(v); // oh no!

        handle.join().unwrap();
    }
```

##### A thread with a closure that attempts to capture a reference to `v` from a main thread that drops v

If we were allowed to run this code, there’s a possibility the spawned thread would be immediately put in the background without running at all. The spawned thread has a reference to `v` inside, but the main thread immediately drops `v`, using the `drop` function we discussed in Chapter 15\. Then, when the spawned thread starts to execute, `v` is no longer valid, so a reference to it is also invalid. Oh no!

To fix the compiler error in the example about vector passed between threads, we can use the error message’s advice:

```text
    help: to force the closure to take ownership of `v` (and any other referenced
    variables), use the `move` keyword
      |
    6 |     let handle = thread::spawn(move || {
      |                                ^^^^^^^
```

By adding the `move` keyword before the closure, we force the closure to take ownership of the values it’s using rather than allowing Rust to infer that it should borrow the values. The modification to that code shown in the code snippet below will compile and run as we intend:

```rust
    use std::thread;

    fn main() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }
```

##### Using the move keyword to force a closure to take ownership of the values it uses

What would happen to the code in the snippet about the thread with a closure where the main thread called `drop` if we use a `move` closure? Would `move` fix that case? Unfortunately, no; we would get a different error because what this code is trying to do isn’t allowed for a different reason. If we added `move` to the closure, we would move `v` into the closure’s environment, and we could no longer call `drop` on it in the main thread. We would get this compiler error instead:

```text
    error[E0382]: use of moved value: `v`
      --> src/main.rs:10:10
       |
    4  |     let v = vec![1, 2, 3];
       |         - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
    5  | 
    6  |     let handle = thread::spawn(move || {
       |                                ------- value moved into closure here
    7  |         println!("Here's a vector: {:?}", v);
       |                                           - variable moved due to use in closure
    ...
    10 |     drop(v); // oh no!
       |          ^ value used here after move
```

Rust’s ownership rules have saved us again! We got an error from the code in the code about passing a vector between threads because Rust was being conservative and only borrowing `v` for the thread, which meant the main thread could theoretically invalidate the spawned thread’s reference. By telling Rust to move ownership of `v` to the spawned thread, we’re guaranteeing Rust that the main thread won’t use `v` anymore. If we change the example about using a closure in the same way, we’re then violating the ownership rules when we try to use `v` in the main thread. The `move` keyword overrides Rust’s conservative default of borrowing; it doesn’t let us violate the ownership rules.

With a basic understanding of threads and the thread API, let’s look at what we can _do_ with threads.

You can refer to the following chapter in the Rust Programming Language Book:
[Using Threads to Run Code Simultaneously](https://doc.rust-lang.org/book/ch16-01-threads.html#using-threads-to-run-code-simultaneously)