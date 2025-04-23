### Variable Scope

We’ve walked through an example of a Rust program already in the previous section. Now that we’re past basic syntax, we won’t include all the `fn main() {` code in examples, so if you’re following along, you’ll have to put the following examples inside a `main` function manually. As a result, our examples will be a bit more concise, letting us focus on the actual details rather than boilerplate code.

As a first example of ownership, we’ll look at the _scope_ of some variables. A scope is the range within a program for which an item is valid. Let’s say we have a variable that looks like this:

```rust
let s = "hello"
```

The variable `s` refers to a string literal, where the value of the string is hardcoded into the text of our program. The variable is valid from the point at which it’s declared until the end of the current _scope_. The code snippet below has comments annotating where the variable `s` is valid.

```rust
{                      // s is not valid here, it’s not yet declared
let s = "hello";   // s is valid from this point forward

// do stuff with s
}                      // this scope is now over, and s is no longer valid
```

##### A variable and the scope in which it is valid

In other words, there are two important points in time here:

*   When `s` comes _into scope_, it is valid.
*   It remains valid until it goes _out of scope_.

At this point, the relationship between scopes and when variables are valid is similar to that in other programming languages. Now we’ll build on top of this understanding by introducing the `String` type.
