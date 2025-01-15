## Data Types

Every value in Rust is of a certain _data type_, which tells Rust what kind of data is being specified so it knows how to work with that data. In this lesson, we’ll look at scalar types, types that represent a single value.

Keep in mind that Rust is a _statically typed language_, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it. In cases when many types are possible, such as when we converted a `String` to a numeric type using `parse`, we must add a type annotation, like this:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

<div class="hint" title="Code explanation">

  Although the example is focused on type annotation, it uses the `parse` and `expect` methods. 
  We won't pay much attention to them now, but for a general understanding, they are used for the following:
  - `parse` converts a string to another type (number in this instance) and returns either a value or `Err`;
  - `expect` allows you to set a custom error message for panic in case `parse` returns `Err`.

  For example, this code
  ```rust
  let guess: u32 = "a42".parse().expect("Not a number!");
  ```
  will cause the following error:
  ```text
  thread 'main' panicked at Common Programming Concepts/Basic Data Types/Intro/src/main.rs:2:36:
  Not a number!: ParseIntError { kind: InvalidDigit }
  ...
```
</div>


If we don’t add the type annotation here, Rust will display the following error, which means the compiler needs more information from us to know which type we want to use:

```text
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^
  |         |
  |         cannot infer type for `_`
  |         consider giving `guess` a type
```

You’ll see different type annotations for other data types.

### Scalar Types

A _scalar_ type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. You may recognize these from other programming languages. Let’s jump into how they work in Rust.