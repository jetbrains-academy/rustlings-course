## Generic Types, Traits, and Lifetimes

Every programming language has tools for effectively handling the duplication
of concepts. In Rust, one such tool is *generics*. Generics are abstract
stand-ins for concrete types or other properties. When we’re writing code, we
can express the behavior of generics or how they relate to other generics
without knowing what will be in their place when compiling and running the code.

Similar to the way a function takes parameters with unknown values to run the
same code on multiple concrete values, functions can take parameters of some
generic type instead of a concrete type, like `i32` or `String`. In fact, we’ve
already used generics in the chapter "Enums" with `Option<T>`, in "Common Collections" with `Vec<T>`
and `HashMap<K, V>`, and in "Recoverable and Unrecoverable Errors" with `Result<T, E>`. In this chapter, you’ll
explore how to define your own types, functions, and methods with generics!

First, we’ll review how to extract a function to reduce code duplication. Next,
we’ll use the same technique to make a generic function from two functions that
differ only in the types of their parameters. We’ll also explain how to use
generic types in struct and enum definitions.

Then you’ll learn how to use *traits* to define behavior in a generic way. You
can combine traits with generic types to constrain a generic type to only
those types that have a particular behavior, as opposed to just any type.

Finally, we’ll discuss *lifetimes*, a variety of generics that give the
compiler information about how references relate to each other. Lifetimes allow
us to borrow values in many situations while still enabling the compiler to
check that the references are valid.

