## Try From Into

`TryFrom` is a simple and safe type conversion that may fail in a controlled way under some circumstances.
Basically, this is the same as `From`. The main difference is that this should return a `Result` type
instead of the target type itself.
You can read more about it at https://doc.rust-lang.org/std/convert/trait.TryFrom.html

Your task here is to complete this implementation
and return an `Ok` result of inner type `Color`.
You need to create an implementation for a tuple of three integers,
an array of three integers and a slice of integers.

Note that the implementation for tuple and array will be checked at compile time,
but the slice implementation needs to check the slice length!
Also note that correct RGB color values must be integers in the 0..=255 range.

<div class="hint">Follow the steps provided in the task.
You can also use this <a href="https://doc.rust-lang.org/std/convert/trait.TryFrom.html">example</a>.</div>

**Note**: in the original rustlings course this exercise uses `Box<dyn error::Error>` and tests for an `Err` with an error
instead of a string, but here we will discuss errors in the next chapter.