### Refactoring with Tuples

The code snippet below shows another version of our program that uses tuples.

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

#### Specifying the width and height of the rectangle with a tuple

In one way, this program is better. Tuples let us add a bit of structure, and
we’re now passing just one argument. But in another way, this version is less
clear: tuples don’t name their elements, so our calculation has become more
confusing because we have to index into the parts of the tuple.

It doesn’t matter if we mix up width and height for the area calculation, but
if we want to draw the rectangle on the screen, it would matter! We would have
to keep in mind that `width` is the tuple index `0` and `height` is the tuple
index `1`. If someone else worked on this code, they would have to figure this
out and keep it in mind as well. It would be easy to forget or mix up these
values and cause errors because we haven’t conveyed the meaning of our data in
our code.