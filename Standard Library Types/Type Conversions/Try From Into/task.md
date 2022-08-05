## Implement TryFrom

`TryFrom` is a simple and safe type conversion that may fail in a controlled way under some circumstances.
Basically, this is the same as `From`. The main difference is that this should return a `Result` type
instead of the target type itself.
You can read more about it at https://doc.rust-lang.org/std/convert/trait.TryFrom.html

Your task here is to complete this implementation
and return an `Ok` result of inner type `Color`.
You need to create an implementation for a tuple of three integers,
an array of three integers, and a slice of integers.

Note that the implementation for tuple and array will be checked at compile time,
but the slice implementation needs to check the slice length!
Also note that correct RGB color values must be integers in the 0..=255 range.

<div class="hint">Follow the steps provided in the task.
You can also use this <a href="https://doc.rust-lang.org/std/convert/trait.TryFrom.html">example</a>.</div>

<div class="hint">Hint: Is there an implementation of <code>TryFrom</code> in the standard library that
can both do the required integer conversion and check the range of the input?</div>

<div class="hint">Look at the test cases to see which error variants to return.</div>

<div class="hint">You can use the <code>map_err</code> or <code>or</code> methods of <code>Result</code> to
convert errors.</div>

<div class="hint">If you would like to propagate errors by using the <code>?</code>
operator in your solution, you might want to look at this <a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/reenter_question_mark.html">article</a>.</div>

**Challenge**: Can you make the `TryFrom` implementations generic over many integer types?