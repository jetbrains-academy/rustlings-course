## An Example: Dealing with Rectangles

To understand when we might want to use structs, let’s write a program that
calculates the area of a rectangle. We’ll start with single variables and then
refactor the program until we’re using structs instead.

Let’s make a new binary project with Cargo called *rectangles* that will take
the width and height of a rectangle specified in pixels and calculate the area
of the rectangle. The listing below shows a short program with one way of doing
exactly that in our project’s *src/main.rs*.

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

#### Calculating the area of a rectangle specified by separate width and height variables

Now, run this program using `cargo run`:

```console
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/structs`
The area of the rectangle is 1500 square pixels.
```

Even though the program above works and figures out the area of the rectangle by
calling the `area` function with each dimension, we can do better. The width
and the height are related to each other because together they describe one
rectangle.

The issue with this code is evident in the signature of `area`:

```rust
fn area(width: u32, height: u32) -> u32 {
```

The `area` function is supposed to calculate the area of one rectangle, but the
function we wrote has two parameters. The parameters are related, but that’s
not expressed anywhere in our program. It would be more readable and more
manageable to group width and height together. We’ve already discussed one way
we might do that in the “Tuple” section
of the lesson "Common Programming Concepts/Data Types": by using tuples.

