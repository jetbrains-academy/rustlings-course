### Lifetime Annotations in Struct Definitions

So far, we’ve only defined structs to hold owned types. It’s possible for
structs to hold references, but in that case we would need to add a lifetime
annotation on every reference in the struct’s definition. The listing below has a
struct named `ImportantExcerpt` that holds a string slice.


```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

#### A struct that holds a reference, so its definition needs a lifetime annotation

This struct has one field, `part`, that holds a string slice, which is a
reference. As with generic data types, we declare the name of the generic
lifetime parameter inside angle brackets after the name of the struct so we can
use the lifetime parameter in the body of the struct definition. This
annotation means an instance of `ImportantExcerpt` can’t outlive the reference
it holds in its `part` field.

The `main` function here creates an instance of the `ImportantExcerpt` struct
that holds a reference to the first sentence of the `String` owned by the
variable `novel`. The data in `novel` exists before the `ImportantExcerpt`
instance is created. In addition, `novel` doesn’t go out of scope until after
the `ImportantExcerpt` goes out of scope, so the reference in the
`ImportantExcerpt` instance is valid.