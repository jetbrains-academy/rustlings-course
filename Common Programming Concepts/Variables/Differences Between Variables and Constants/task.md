### Differences Between Variables and Constants

Being unable to change the value of a variable might have reminded you of another programming concept that most other languages have: constants. Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

First, you aren’t allowed to use `mut` with constants. Constants aren’t just immutable by default—they’re always immutable.

You declare constants using the `const` keyword instead of the `let` keyword, and the type of the value must be annotated. We’re about to cover types and type annotations in the next section, [Data Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#data-types), so don’t worry about the details right now. Just know that you must always annotate the type.

Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.

The last difference is that constants may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime.

Here’s an example of a constant declaration where the constant’s name is MAX_POINTS and its value is set to 100,000. (Rust’s naming convention for constants is to use all uppercase with underscores between words, and underscores can be inserted in numeric literals to improve readability):

```
const MAX_POINTS: u32 = 100_000;
```


Constants are valid for the entire time a program runs, within the scope they were declared in, making them a useful choice for values in your application domain that multiple parts of the program might need to know about, such as the maximum number of points any player of a game is allowed to earn or the speed of light.

Naming hardcoded values used throughout your program as constants is useful in conveying the meaning of that value to future maintainers of the code. It also helps to have only one place in your code you would need to change if the hardcoded value needed to be updated in the future.

_You can refer to the following chapter in the Rust Programming Language Book: [Differences Between Variables and Constants](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html#differences-between-variables-and-constants)_