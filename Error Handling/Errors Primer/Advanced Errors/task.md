## Advanced Errors

Remember back in the previous exercise, we had multiple mapping functions so that we
could translate lower-level errors into our custom error type using
`map_err()`? What if we could use the `?` operator directly instead?

Make this code compile! Complete the code so that an appropriate error is returned in each of the cases in `main()`.
Pay attention to comments and check out hints if you're stuck.

<div class="hint">
The parsing
code is now in an implementation of the <code>FromStr</code> trait. Note that the
parsing code uses <code>?</code> directly, without any calls to <code>map_err()</code>. There is
one partial implementation of the <code>From</code> trait example that you should
complete.
</div>

<div class="hint">
Details: The <code>?</code> operator calls <code>From::from()</code> on the error type to convert
it to the error type of the return type of the surrounding function.
</div>
<div class="hint">
You will need to write another implementation of <code>From</code> that has a
different input type.
</div>