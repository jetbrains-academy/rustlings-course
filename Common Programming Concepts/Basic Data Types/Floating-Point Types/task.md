## Floating-Point Types

Rust also has two primitive types for `floating-point numbers`, which are numbers with decimal points. Rust’s floating-point types are `f32` and `f64`, which are 32 bits and 64 bits in size, respectively. The default type is `f64` because on modern CPUs it’s roughly the same speed as `f32` but is capable of more precision.

Here’s an example that shows floating-point numbers in action:

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    let z = 4.0f32; // f32
}
```

Floating-point numbers are represented according to the IEEE-754 standard. The `f32` type is a single-precision float, and `f64` has double precision.