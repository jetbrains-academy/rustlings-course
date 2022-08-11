## Error Trait

This exercise demonstrates a few traits that are useful for custom error
types to implement, especially so that other code can consume the custom
error type more usefully.

Make this compile, and make the tests pass!
Check out the hints if you're stuck.

Steps:
1. Implement a missing trait so that `main()` will compile.
2. Complete the partial implementation of `From` for `ParseClimateError`.
3. Handle the missing error cases in the `FromStr` implementation for `Climate`.
4. Complete the partial implementation of `Display` for `ParseClimateError`.

Parser for `Climate`:
1. Split the input string into 3 fields: `city`, `year`, `temp`.
2. Return an error if the string is empty or has the wrong number of fields.
3. Return an error if the city name is empty.
4. Parse the year as a `u32` and return an error if that fails.
5. Parse the temp as a `f32` and return an error if that fails.
6. Return an `Ok` value containing the completed `Climate` value.

Add or complete the missing steps.
Don't forget to read comments in the code.

<div class="hint">This exercise demonstrates a few traits that are useful for custom error
types to implement. These traits make it easier for other code to consume
the custom error type.</div>

<div class="hint">You will have to
supply a missing trait implementation, and complete a few incomplete ones.</div>

<div class="hint">
You may find these pages to be helpful references:
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/define_error_type.html">1</a>,
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/boxing_errors.html">2</a>, 
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/wrap_error.html">3</a>.
</div>

<div class="hint">
What trait must our error type have for <code>main()</code> to return the return
type that it returns?
</div>

<div class="hint">It's not necessary to implement any methods inside the missing
trait. (Some methods have default implementations that are supplied by the
trait.)</div>

<div class="hint">Consult the tests to determine which error variants (and which
error message text) to produce for certain error conditions.</div>

**Challenge**: There is one test that is marked `#[ignore]`. Can you supply the
missing code that will make it pass? You may want to consult the standard
library documentation for a certain trait for more hints.