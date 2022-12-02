## Want some ice cream?

We have the `maybe_ice_cream` function that returns how much ice cream there is left in the fridge.
If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
all, so there'll be no more left :(

Your task is to complete the implementation of the `maybe_ice_cream` function and fix the test after it.

To learn about Option<T>, check out these links:

- [Option Enum Format](https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#in-enum-definitions)
- [Option Module Documentation](https://doc.rust-lang.org/std/option/)
- [Option Enum Documentation](https://doc.rust-lang.org/std/option/enum.Option.html)


<div class="hint">Options can have a <code>Some</code> value, with an inner value, or a <code>None</code> value, without an inner value.
There's multiple ways to get at the inner value, you can use <code>unwrap</code>, or pattern match. Unwrapping
is the easiest, but how do you do it safely so that it doesn't panic in your face later?</div>
