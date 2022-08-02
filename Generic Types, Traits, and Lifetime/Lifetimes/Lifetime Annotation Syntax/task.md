### Lifetime Annotation Syntax

Lifetime annotations don’t change how long any of the references live. Just
as functions can accept any type when the signature specifies a generic type
parameter, functions can accept references with any lifetime by specifying a
generic lifetime parameter. Lifetime annotations describe the relationships of
the lifetimes of multiple references to each other without affecting the
lifetimes.

Lifetime annotations have a slightly unusual syntax: the names of lifetime
parameters must start with an apostrophe (`'`) and are usually all lowercase and
very short, like generic types. Most people use the name `'a`. We place
lifetime parameter annotations after the `&` of a reference, using a space to
separate the annotation from the reference’s type.

Here are some examples: a reference to an `i32` without a lifetime parameter, a
reference to an `i32` that has a lifetime parameter named `'a`, and a mutable
reference to an `i32` that also has the lifetime `'a`.

```rust,ignore
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

One lifetime annotation by itself doesn’t have much meaning, because the
annotations are meant to tell Rust how generic lifetime parameters of multiple
references relate to each other. For example, let’s say we have a function with
the parameter `first` that is a reference to an `i32` with lifetime `'a`. The
function also has another parameter named `second` that is another reference to
an `i32` that also has the lifetime `'a`. The lifetime annotations indicate
that the references `first` and `second` must both live as long as that generic
lifetime.