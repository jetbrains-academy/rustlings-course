### Lifetime Annotations in Method Definitions

When we implement methods on a struct with lifetimes, we use the same syntax as
that of generic type parameters shown in the listing "A method that uses different generic types
from its struct’s definition" in the section "Generic Data Types". Where we declare and
use the lifetime parameters depends on whether they’re related to the struct
fields or the method parameters and return values.

Lifetime names for struct fields always need to be declared after the `impl`
keyword and then used after the struct’s name, because those lifetimes are part
of the struct’s type.

In method signatures inside the `impl` block, references might be tied to the
lifetime of references in the struct’s fields, or they might be independent. In
addition, the lifetime elision rules often make it so that lifetime annotations
aren’t necessary in method signatures. Let’s look at some examples using the
struct named `ImportantExcerpt` that we defined earlier in this section.

First, we’ll use a method named `level` whose only parameter is a reference to
`self` and whose return value is an `i32`, which is not a reference to anything:

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

The lifetime parameter declaration after `impl` and its use after the type name
are required, but we’re not required to annotate the lifetime of the reference
to `self` because of the first elision rule.

Here is an example where the third lifetime elision rule applies:

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

There are two input lifetimes, so Rust applies the first lifetime elision rule
and gives both `&self` and `announcement` their own lifetimes. Then, because
one of the parameters is `&self`, the return type gets the lifetime of `&self`,
and all lifetimes have been accounted for.

