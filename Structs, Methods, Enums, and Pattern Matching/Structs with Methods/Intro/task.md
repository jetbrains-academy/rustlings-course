## Method Syntax

*Methods* are similar to functions: they’re declared with the `fn` keyword and
their name, they can have parameters and a return value, and they contain some
code that is run when they’re called from somewhere else. However, methods are
different from functions in that they’re defined within the context of a struct
(or an enum or a trait object, which are covered in the [Enums](course://Structs,+Methods,+Enums,+and+Pattern+Matching/Enums) lesson and in [Chapter 18][ch18] of the Rust Book,
respectively), and their first parameter is always `self`, which represents the
instance of the struct the method is being called on.

[ch18]: https://doc.rust-lang.org/book/ch18-00-oop.html