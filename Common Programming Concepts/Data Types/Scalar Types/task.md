## Data Types

Every value in Rust is of a certain _data type_, which tells Rust what kind of data is being specified so it knows how to work with that data. We’ll look at two data type subsets: scalar and compound.

Keep in mind that Rust is a _statically typed language_, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it. In cases when many types are possible, such as when we convert a `String` to a numeric type using `parse` we must add a type annotation, like this:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

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

#### Integer Types

An _integer_ is a number without a fractional component. We used one integer type in Lesson 2, the `u32` type. This type declaration indicates that the value it’s associated with should be an unsigned integer (signed integer types start with `i`, instead of `u`) that takes up 32 bits of space. Table 3-1 shows the built-in integer types in Rust. Each variant in the Signed and Unsigned columns (for example, i16) can be used to declare the type of an integer value.

| Length  | Signed | Unsigned |
|---------|--------|----------|
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

##### Table 3-1: Integer Types in Rust

Each variant can be either signed or unsigned and has an explicit size. _Signed_ and _unsigned_ refer to whether it’s possible for the number to be negative or positive—in other words, whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a sign (unsigned). It’s like writing numbers on paper: when the sign matters, a number is shown with a plus sign or a minus sign; however, when it’s safe to assume the number is positive, it’s shown with no sign. Signed numbers are stored using [two’s complement](https://en.wikipedia.org/wiki/Two%27s_complement) representation.

Each signed variant can store numbers from -($2^{n-1}$) to $2^{n - 1}$-1 inclusive, where _n_ is the number of bits that variant uses. So an `i8` can store numbers from -($2^7$) to $2^7$-1, which equals -128 to 127. Unsigned variants can store numbers from 0 to $2^n$-1, so a `u8` can store numbers from 0 to $2^8$-1, which equals 0 to 255.

Additionally, the `isize` and `usize` types depend on the kind of computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

You can write integer literals in any of the forms shown in Table 3-2. Note that all number literals except the byte literal allow a type suffix, such as `57u8`, and `_` as a visual separator, such as `1_000`.

|Number literals|	Example  |
|---------------|------------|
|Decimal        |  98_222    |
|Hex 	        |   0xff     |
|Octal 	        |   0o77     |
|Binary 	    | 0b1111_0000|
|Byte (u8 only) |	b'A'     |

##### Table 3-2: Integer Literals in Rust

So how do you know which type of integer to use? If you’re unsure, Rust’s defaults are generally good choices, and integer types default to `i32`: this type is generally the fastest, even on 64-bit systems. The primary situation in which you’d use `isize` or `usize` is when indexing some sort of collection.

#### Integer Overflow

  Let’s say you have a variable of type `u8` that can hold values between 0 and 255. If you try to change the variable to a value outside of that range, such as 256, _integer overflow_ will occur. Rust has some interesting rules involving this behavior. When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to _panic_ at runtime if this behavior occurs. Rust uses the term panicking when a program exits with an error; we’ll discuss panics in more depth in the [“Unrecoverable Errors with panic!”](https://doc.rust-lang.org/stable/book/ch09-01-unrecoverable-errors-with-panic.html) section in Chapter 9 of the Rust Programming Language Book.

 When you’re compiling in release mode with the `--release` flag, Rust does _not_ include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs _two’s complement wrapping_. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a `u8`, 256 becomes 0, 257 becomes 1, and so on. The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have. Relying on integer overflow’s wrapping behavior is considered an error. If you want to wrap explicitly, you can use the standard library type `Wrapping`.

#### Floating-Point Types

Rust also has two primitive types for `floating-point numbers`, which are numbers with decimal points. Rust’s floating-point types are `f32` and `f64`, which are 32 bits and 64 bits in size, respectively. The default type is `f64` because on modern CPUs it’s roughly the same speed as `f32` but is capable of more precision.

Here’s an example that shows floating-point numbers in action:

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

Floating-point numbers are represented according to the IEEE-754 standard. The `f32` type is a single-precision float, and `f64` has double precision.

#### Numeric Operations

Rust supports the basic mathematical operations you’d expect for all of the number types: addition, subtraction, multiplication, division, and remainder. The following code shows how you’d use each one in a `let` statement:

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
}
```

Each expression in these statements uses a mathematical operator and evaluates to a single value, which is then bound to a variable. Appendix B contains a list of all operators that Rust provides.

#### The Boolean Type

As in most other programming languages, a Boolean type in Rust has two possible values: `true` and `false`. Booleans are one byte in size. The Boolean type in Rust is specified using `bool`. For example:

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

The main way to use Boolean values is through conditionals, such as an `if` expression. We’ll cover how `if` expressions work in Rust in the [“Control Flow”](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#control-flow) section.

#### The Character Type

So far we’ve worked only with numbers, but Rust supports letters too. Rust’s `char` type is the language’s most primitive alphabetic type, and the following code shows one way to use it. (Note that `char` literals are specified with single quotes, as opposed to string literals, which use double quotes.)

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart = '❤';
}
```

Rust’s `char` type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid `char` values in Rust. Unicode Scalar Values range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive. However, a “character” isn’t really a concept in Unicode, so your human intuition for what a “character” is may not match up with what a `char` is in Rust. We’ll discuss this topic in detail in [“Storing UTF-8 Encoded Text with Strings”](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings) in Chapter 8.

_You can refer to the following chapter in the Rust Programming Language Book:[Scalar Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#scalar-types)_