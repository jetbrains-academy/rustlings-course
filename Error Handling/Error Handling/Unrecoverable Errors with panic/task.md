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

