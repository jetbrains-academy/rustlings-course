## Numeric Operations

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

Each expression in these statements uses a mathematical operator and evaluates to a single value, which is then bound to a variable. [Appendix B](https://doc.rust-lang.org/stable/book/appendix-02-operators.html) contains a list of all operators that Rust provides.