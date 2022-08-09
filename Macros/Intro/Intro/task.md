## Macros

We’ve used macros like `println!` throughout this book, but we haven’t fully explored what a macro is and how it works. The term _macro_ refers to a family of features in Rust: _declarative_ macros with `macro_rules!` and three kinds of _procedural_ macros:

*   Custom `#[derive]` macros that specify code added with the `derive` attribute used on structs and enums
*   Attribute-like macros that define custom attributes usable on any item
*   Function-like macros that look like function calls but operate on the tokens specified as their argument

We’ll talk about each of these in turn, but first, let’s look at why we even need macros when we already have functions.

