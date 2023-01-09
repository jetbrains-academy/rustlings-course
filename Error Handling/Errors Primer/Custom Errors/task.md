## Custom Errors

Using catch-all error types like `Box<dyn error::Error>` isn't recommended
for library code, where callers might want to make decisions based on the
error content, instead of printing it out or propagating it further. Here,
we define a custom error type to make it possible for callers to decide
what to do next when our function returns an error.

Make the tests pass! Read the hints if you are stuck.

<div class="hint">This exercise uses a completed version of <code>PositiveNonzeroInteger</code> from
<a href="course://Error Handling/Errors Primer/Positive Nonzero Integer"> the previous task</a>.</div>

<div class="hint">You could create another function inside <code>impl ParsePosNonzeroError</code>.

For example, it'd be useful to build `ParsePosNonzeroError` from `ParseIntError` that can be
returned by the `parse` function.
</div>

<div class="hint">Below the line that a comment asks you to change, there is an example of using
the <code>map_err()</code> method on a <code>Result</code> to transform one type of error into
another. Try using something similar on the <code>Result</code> from <code>parse()</code>. You
might use the <code>?</code> operator to return early from the function, or you might
use a <code>match</code> expression, or maybe there's another way!</div>


<div class="hint">Read more about <code>map_err()</code> in the <code>std::result</code> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err">documentation</a>.
</div>