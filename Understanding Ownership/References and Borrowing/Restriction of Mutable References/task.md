## Restriction of Mutable References

But mutable references have one big restriction: you can have only one mutable reference to a particular piece of data in a particular scope. This code will fail:

```rust
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
```

Here’s the error:

```text
    error[E0499]: cannot borrow `s` as mutable more than once at a time
     --> src/main.rs:5:14
      |
    4 |     let r1 = &mut s;
      |              ------ first mutable borrow occurs here
    5 |     let r2 = &mut s;
      |              ^^^^^^ second mutable borrow occurs here
    6 |
    7 |     println!("{}, {}", r1, r2);
      |                        -- first borrow later used here
```

This restriction allows for mutation but in a very controlled fashion. It’s something that new Rustaceans struggle with because most languages let you mutate whenever you’d like.

The benefit of having this restriction is that Rust can prevent data races at compile time. A _data race_ is similar to a race condition and happens when these three behaviors occur:

*   Two or more pointers access the same data at the same time.
*   At least one of the pointers is being used to write to the data.
*   There’s no mechanism being used to synchronize access to the data.

Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; Rust prevents this problem from happening because it won’t even compile code with data races!

As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not _simultaneous_ ones:

```rust
    let mut s = String::from("hello");

    {
        let r1 = &mut s;

    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
```

A similar rule exists for combining mutable and immutable references. This code results in an error:

```rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
```

Here’s the error:

```text
    error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
     --> src/main.rs:6:14
      |
    4 |     let r1 = &s; // no problem
      |              -- immutable borrow occurs here
    5 |     let r2 = &s; // no problem
    6 |     let r3 = &mut s; // BIG PROBLEM
      |              ^^^^^^ mutable borrow occurs here
    7 |
    8 |     println!("{}, {}, and {}", r1, r2, r3);
      |                                -- immutable borrow later used here
```

Whew! We _also_ cannot have a mutable reference while we have an immutable one. Users of an immutable reference don’t expect the values to suddenly change out from under them! However, multiple immutable references are okay because no one who is just reading the data has the ability to affect anyone else’s reading of the data.
