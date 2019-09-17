# Fearless Concurrency

Handling concurrent programming safely and efficiently is another of Rust’s major goals. _Concurrent programming_, where different parts of a program execute independently, and _parallel programming_, where different parts of a program execute at the same time, are becoming increasingly important as more computers take advantage of their multiple processors. Historically, programming in these contexts has been difficult and error prone: Rust hopes to change that.

Initially, the Rust team thought that ensuring memory safety and preventing concurrency problems were two separate challenges to be solved with different methods. Over time, the team discovered that the ownership and type systems are a powerful set of tools to help manage memory safety _and_ concurrency problems! By leveraging ownership and type checking, many concurrency errors are compile-time errors in Rust rather than runtime errors. Therefore, rather than making you spend lots of time trying to reproduce the exact circumstances under which a runtime concurrency bug occurs, incorrect code will refuse to compile and present an error explaining the problem. As a result, you can fix your code while you’re working on it rather than potentially after it has been shipped to production. We’ve nicknamed this aspect of Rust _fearless_ _concurrency_. Fearless concurrency allows you to write code that is free of subtle bugs and is easy to refactor without introducing new bugs.

> Note: For simplicity’s sake, we’ll refer to many of the problems as _concurrent_ rather than being more precise by saying _concurrent and/or parallel_. If this book were about concurrency and/or parallelism, we’d be more specific. For this chapter, please mentally substitute _concurrent and/or parallel_ whenever we use _concurrent_.

Many languages are dogmatic about the solutions they offer for handling concurrent problems. For example, Erlang has elegant functionality for message-passing concurrency but has only obscure ways to share state between threads. Supporting only a subset of possible solutions is a reasonable strategy for higher-level languages, because a higher-level language promises benefits from giving up some control to gain abstractions. However, lower-level languages are expected to provide the solution with the best performance in any given situation and have fewer abstractions over the hardware. Therefore, Rust offers a variety of tools for modeling problems in whatever way is appropriate for your situation and requirements.

Here are the topics we’ll cover in this chapter:

*   How to create threads to run multiple pieces of code at the same time
*   _Message-passing_ concurrency, where channels send messages between threads
*   _Shared-state_ concurrency, where multiple threads have access to some piece of data
*   The `Sync` and `Send` traits, which extend Rust’s concurrency guarantees to user-defined types as well as types provided by the standard library

## Using Threads to Run Code Simultaneously

In most current operating systems, an executed program’s code is run in a _process_, and the operating system manages multiple processes at once. Within your program, you can also have independent parts that run simultaneously. The features that run these independent parts are called _threads_.

Splitting the computation in your program into multiple threads can improve performance because the program does multiple tasks at the same time, but it also adds complexity. Because threads can run simultaneously, there’s no inherent guarantee about the order in which parts of your code on different threads will run. This can lead to problems, such as:

*   Race conditions, where threads are accessing data or resources in an inconsistent order
*   Deadlocks, where two threads are waiting for each other to finish using a resource the other thread has, preventing both threads from continuing
*   Bugs that happen only in certain situations and are hard to reproduce and fix reliably

Rust attempts to mitigate the negative effects of using threads, but programming in a multithreaded context still takes careful thought and requires a code structure that is different from that in programs running in a single thread.

Programming languages implement threads in a few different ways. Many operating systems provide an API for creating new threads. This model where a language calls the operating system APIs to create threads is sometimes called _1:1_, meaning one operating system thread per one language thread.

Many programming languages provide their own special implementation of threads. Programming language-provided threads are known as _green_ threads, and languages that use these green threads will execute them in the context of a different number of operating system threads. For this reason, the green-threaded model is called the _M:N_ model: there are `M` green threads per `N` operating system threads, where `M` and `N` are not necessarily the same number.

Each model has its own advantages and trade-offs, and the trade-off most important to Rust is runtime support. _Runtime_ is a confusing term and can have different meanings in different contexts.

In this context, by _runtime_ we mean code that is included by the language in every binary. This code can be large or small depending on the language, but every non-assembly language will have some amount of runtime code. For that reason, colloquially when people say a language has “no runtime,” they often mean “small runtime.” Smaller runtimes have fewer features but have the advantage of resulting in smaller binaries, which make it easier to combine the language with other languages in more contexts. Although many languages are okay with increasing the runtime size in exchange for more features, Rust needs to have nearly no runtime and cannot compromise on being able to call into C to maintain performance.

The green-threading M:N model requires a larger language runtime to manage threads. As such, the Rust standard library only provides an implementation of 1:1 threading. Because Rust is such a low-level language, there are crates that implement M:N threading if you would rather trade overhead for aspects such as more control over which threads run when and lower costs of context switching, for example.

Now that we’ve defined threads in Rust, let’s explore how to use the thread-related API provided by the standard library.

### Creating a New Thread with spawn

To create a new thread, we call the `thread::spawn` function and pass it a closure (we talked about closures in Chapter 13) containing the code we want to run in the new thread. The example in the code snippet below prints some text from a main thread and other text from a new thread:

```rust
    use std::thread;
    use std::time::Duration;

    fn main() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
```

##### Creating a new thread to print one thing while the main thread prints something else

Note that with this function, the new thread will be stopped when the main thread ends, whether or not it has finished running. The output from this program might be a little different every time, but it will look similar to the following:

```text
    hi number 1 from the main thread!
    hi number 1 from the spawned thread!
    hi number 2 from the main thread!
    hi number 2 from the spawned thread!
    hi number 3 from the main thread!
    hi number 3 from the spawned thread!
    hi number 4 from the main thread!
    hi number 4 from the spawned thread!
    hi number 5 from the spawned thread!
```

The calls to `thread::sleep` force a thread to stop its execution for a short duration, allowing a different thread to run. The threads will probably take turns, but that isn’t guaranteed: it depends on how your operating system schedules the threads. In this run, the main thread printed first, even though the print statement from the spawned thread appears first in the code. And even though we told the spawned thread to print until `i` is 9, it only got to 5 before the main thread shut down.

If you run this code and only see output from the main thread, or don’t see any overlap, try increasing the numbers in the ranges to create more opportunities for the operating system to switch between the threads.

### Waiting for All Threads to Finish Using join Handles

The code in above not only stops the spawned thread prematurely most of the time due to the main thread ending, but also can’t guarantee that the spawned thread will get to run at all. The reason is that there is no guarantee on the order in which threads run!

We can fix the problem of the spawned thread not getting to run, or not getting to run completely, by saving the return value of `thread::spawn` in a variable. The return type of `thread::spawn` is `JoinHandle`. A `JoinHandle` is an owned value that, when we call the `join` method on it, will wait for its thread to finish. The example below shows how to use the `JoinHandle` of the thread we created before and call `join` to make sure the spawned thread finishes before `main` exits:

```rust
    use std::thread;
    use std::time::Duration;

    fn main() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();
    }
```

##### Saving a JoinHandle from thread::spawn to guarantee the thread is run to completion

Calling `join` on the handle blocks the thread currently running until the thread represented by the handle terminates. _Blocking_ a thread means that thread is prevented from performing work or exiting. Because we’ve put the call to `join` after the main thread’s `for` loop, running this code should produce output similar to this:

```text
    hi number 1 from the main thread!
    hi number 2 from the main thread!
    hi number 1 from the spawned thread!
    hi number 3 from the main thread!
    hi number 2 from the spawned thread!
    hi number 4 from the main thread!
    hi number 3 from the spawned thread!
    hi number 4 from the spawned thread!
    hi number 5 from the spawned thread!
    hi number 6 from the spawned thread!
    hi number 7 from the spawned thread!
    hi number 8 from the spawned thread!
    hi number 9 from the spawned thread!
```

The two threads continue alternating, but the main thread waits because of the call to `handle.join()` and does not end until the spawned thread is finished.

But let’s see what happens when we instead move `handle.join()` before the `for` loop in `main`, like this:

```rust
    use std::thread;
    use std::time::Duration;

    fn main() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        handle.join().unwrap();

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
```

The main thread will wait for the spawned thread to finish and then run its `for` loop, so the output won’t be interleaved anymore, as shown here:

```text
    hi number 1 from the spawned thread!
    hi number 2 from the spawned thread!
    hi number 3 from the spawned thread!
    hi number 4 from the spawned thread!
    hi number 5 from the spawned thread!
    hi number 6 from the spawned thread!
    hi number 7 from the spawned thread!
    hi number 8 from the spawned thread!
    hi number 9 from the spawned thread!
    hi number 1 from the main thread!
    hi number 2 from the main thread!
    hi number 3 from the main thread!
    hi number 4 from the main thread!
```

Small details, such as where `join` is called, can affect whether or not your threads run at the same time.

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
    error[E0373]: closure may outlive the current function, but it borrows `v`,
    which is owned by the current function
     --> src/main.rs:6:32
      |
    6 |     let handle = thread::spawn(|| {
      |                                ^^ may outlive borrowed value `v`
    7 |         println!("Here's a vector: {:?}", v);
      |                                           - `v` is borrowed here
      |
    help: to force the closure to take ownership of `v` (and any other referenced
    variables), use the `move` keyword
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
    6  |     let handle = thread::spawn(move || {
       |                                ------- value moved (into closure) here
    ...
    10 |     drop(v); // oh no!
       |          ^ value used here after move
       |
       = note: move occurs because `v` has type `std::vec::Vec<i32>`, which does
       not implement the `Copy` trait
```

Rust’s ownership rules have saved us again! We got an error from the code in the code about passing a vector between threads because Rust was being conservative and only borrowing `v` for the thread, which meant the main thread could theoretically invalidate the spawned thread’s reference. By telling Rust to move ownership of `v` to the spawned thread, we’re guaranteeing Rust that the main thread won’t use `v` anymore. If we change the example about using a closure in the same way, we’re then violating the ownership rules when we try to use `v` in the main thread. The `move` keyword overrides Rust’s conservative default of borrowing; it doesn’t let us violate the ownership rules.

With a basic understanding of threads and the thread API, let’s look at what we can _do_ with threads.

You can refer to the following chapter in the Rust Programming Language Book:
[Using Threads to Run Code Simultaneously](https://doc.rust-lang.org/book/ch16-01-threads.html#using-threads-to-run-code-simultaneously)