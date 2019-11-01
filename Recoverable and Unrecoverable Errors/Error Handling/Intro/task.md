# Error Handling

Rust’s commitment to reliability extends to error handling. Errors are a fact of life in software, so Rust has a number of features for handling situations in which something goes wrong. In many cases, Rust requires you to acknowledge the possibility of an error and take some action before your code will compile. This requirement makes your program more robust by ensuring that you’ll discover errors and handle them appropriately before you’ve deployed your code to production!

Rust groups errors into two major categories: _recoverable_ and _unrecoverable_ errors. For a recoverable error, such as a file not found error, it’s reasonable to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.

Most languages don’t distinguish between these two kinds of errors and handle both in the same way, using mechanisms such as exceptions. Rust doesn’t have exceptions. Instead, it has the type `Result<T, E>` for recoverable errors and the `panic!` macro that stops execution when the program encounters an unrecoverable error. This chapter covers calling `panic!` first and then talks about returning `Result<T, E>` values. Additionally, we’ll explore considerations when deciding whether to try to recover from an error or to stop execution.

## Unrecoverable Errors with panic!

Sometimes, bad things happen in your code, and there’s nothing you can do about it. In these cases, Rust has the `panic!` macro. When the `panic!` macro executes, your program will print a failure message, unwind and clean up the stack, and then quit. This most commonly occurs when a bug of some kind has been detected and it’s not clear to the programmer how to handle the error.


> ### Unwinding the Stack or Aborting in Response to a Panic
>
> By default, when a panic occurs, the program starts _unwinding_, which means Rust walks back up the stack and cleans up the data from each function it encounters. But this walking back and cleanup is a lot of work. The alternative is to immediately _abort_, which ends the program without cleaning up. Memory that the program was using will then need to be cleaned up by the operating system. If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting upon a panic by adding `panic = 'abort'` to the appropriate `[profile]` sections in your _Cargo.toml_ file. For example, if you want to abort on panic in release mode, add this:
> 
>     [profile.release]
>     panic = 'abort'

Let’s try calling `panic!` in a simple program:

```rust
    fn main() {
        panic!("crash and burn");
    }
```

When you run the program, you’ll see something like this:

```text
  Compiling test_rust_project v0.1.0
      Finished dev [unoptimized + debuginfo] target(s) in 0.42s
       Running `target/debug/test_rust_project`
  thread 'main' panicked at 'crash and burn', src/main.rs:2:5
```

The call to `panic!` causes the error message contained in the last two lines. The first line shows our panic message and the place in our source code where the panic occurred: _src/main.rs:2:5_ indicates that it’s the second line, fifth character of our _src/main.rs_ file.

In this case, the line indicated is part of our code, and if we go to that line, we see the `panic!` macro call. In other cases, the `panic!` call might be in code that our code calls, and the filename and line number reported by the error message will be someone else’s code where the `panic!` macro is called, not the line of our code that eventually led to the `panic!` call. We can use the backtrace of the functions the `panic!` call came from to figure out the part of our code that is causing the problem. We’ll discuss what a backtrace is in more detail next.

### Using a `panic!` Backtrace

Let’s look at another example to see what it’s like when a `panic!` call comes from a library because of a bug in our code instead of from our code calling the macro directly. The code snippet below has some code that attempts to access an element by index in a vector.

```rust
    fn main() {
        let v = vec![1, 2, 3];

        v[99];
    }
```

##### Attempting to access an element beyond the end of a vector, which will cause a call to panic!

Here, we’re attempting to access the 100th element of our vector (which is at index 99 because indexing starts at zero), but it has only 3 elements. In this situation, Rust will panic. Using `[]` is supposed to return an element, but if you pass an invalid index, there’s no element that Rust could return here that would be correct.

Other languages, like C, will attempt to give you exactly what you asked for in this situation, even though it isn’t what you want: you’ll get whatever is at the location in memory that would correspond to that element in the vector, even though the memory doesn’t belong to the vector. This is called a _buffer overread_ and can lead to security vulnerabilities if an attacker is able to manipulate the index in such a way as to read data they shouldn’t be allowed to that is stored after the array.

To protect your program from this sort of vulnerability, if you try to read an element at an index that doesn’t exist, Rust will stop execution and refuse to continue. Let’s try it and see:

```text
   Compiling test_rust_project v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/test_rust_project`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /rustc/3c235d5600393dfe6c36eeed34042efad8d4f26e/src/libcore/slice/mod.rs:2686:10
This error points at a file we didn’t write, _libcore/slice/mod.rs_. That’s the implementation of `slice` in the Rust source code. The code that gets run when we use `[]` on our vector `v` is in _libcore/slice/mod.rs_, and that is where the `panic!` is actually happening.
```

The next note line tells us that we can set the `RUST_BACKTRACE` environment variable to get a backtrace of exactly what happened to cause the error. A _backtrace_ is a list of all the functions that have been called to get to this point. Backtraces in Rust work as they do in other languages: the key to reading the backtrace is to start from the top and read until you see files you wrote. That’s the spot where the problem originated. The lines above the lines mentioning your files are code that your code called; the lines below are code that called your code. These lines might include core Rust code, standard library code, or crates that you’re using. Let’s try getting a backtrace by setting the `RUST_BACKTRACE` environment variable to any value except 0. The code snippet below shows output similar to what you’ll see.

```text
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:59
             at src/libstd/panicking.rs:197
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:211
   4: <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::get
             at src/libstd/panicking.rs:474
   5: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:381
   6: std::panicking::try::do_call
             at src/libstd/panicking.rs:308
   7: <T as core::any::Any>::type_id
             at src/libcore/panicking.rs:85
   8: <T as core::any::Any>::type_id
             at src/libcore/panicking.rs:61
   9: <usize as core::slice::SliceIndex<[T]>>::index
             at /rustc/3c235d5600393dfe6c36eeed34042efad8d4f26e/src/libcore/slice/mod.rs:2686
  10: core::slice::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/3c235d5600393dfe6c36eeed34042efad8d4f26e/src/libcore/slice/mod.rs:2543
  11: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
             at /rustc/3c235d5600393dfe6c36eeed34042efad8d4f26e/src/liballoc/vec.rs:1677
  12: test_rust_project::main
             at src/main.rs:4
  13: std::rt::lang_start::{{closure}}
             at /rustc/3c235d5600393dfe6c36eeed34042efad8d4f26e/src/libstd/rt.rs:64
  14: std::panicking::try::do_call
             at src/libstd/rt.rs:49
             at src/libstd/panicking.rs:293
  15: panic_unwind::dwarf::eh::read_encoded_pointer
             at src/libpanic_unwind/lib.rs:87
  16: <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::get
             at src/libstd/panicking.rs:272
             at src/libstd/panic.rs:388
             at src/libstd/rt.rs:48
  17: std::rt::lang_start
             at /rustc/3c235d5600393dfe6c36eeed34042efad8d4f26e/src/libstd/rt.rs:64
  18: test_rust_project::main

Process finished with exit code 101
```

##### The backtrace generated by a call to panic! displayed when the environment variable RUST_BACKTRACE is set

That’s a lot of output! The exact output you see might be different depending on your operating system and Rust version. In order to get backtraces with this information, debug symbols must be enabled. Debug symbols are enabled by default when using `cargo build` or `cargo run` without the `--release` flag, as we have here.

In the output in the code snippet above, line 12 of the backtrace points to the line in our project that’s causing the problem: line 4 of _src/main.rs_. If we don’t want our program to panic, the location pointed to by the first line mentioning a file we wrote is where we should start investigating. In the first code snippet, where we deliberately wrote code that would panic in order to demonstrate how to use backtraces, the way to fix the panic is to not request an element at index 99 from a vector that only contains 3 items. When your code panics in the future, you’ll need to figure out what action the code is taking with what values to cause the panic and what the code should do instead.

We’ll come back to `panic!` and when we should and should not use `panic!` to handle error conditions in the [“To `panic!` or Not to `panic!`”](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#to-panic-or-not-to-panic) section later in this chapter. Next, we’ll look at how to recover from an error using `Result`.

_You can refer to the following chapter in the Rust Programming Language Book:
[Unrecoverable Errors with panic!](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unrecoverable-errors-with-panic)_