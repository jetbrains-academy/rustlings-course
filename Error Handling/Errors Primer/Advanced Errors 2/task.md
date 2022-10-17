## Advanced Errors 2

This exercise demonstrates a few approaches that are useful for custom error
types, especially so that other code can consume the custom
error type more usefully.

Make this compile, and make the tests pass!
Check out the hints if you're stuck.

Steps:
1. Make sure that `ParseClimateError` is suitable for error propagation in  `main()`.
2. Complete the partial implementation of `From` for `ParseClimateError`.
3. Handle the missing error cases in the `FromStr` implementation for `Climate` using the parsing guidelines below.
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

<div class="hint">

Let's look at the `main.rs` file. The result type of the `main` function is `Result<(), Box<dyn Error>>`. This means that we can return either `()`, or *any error*. Further in the `main` function we use error propagation:

```rust
  "Hong Kong,1999,25.7".parse::<Climate>()?
```

This code propagates the `ParseClimateError` custom error. To make it suitable for passing along as *any error*, we need to make it an `Error` as follows:
```rust
impl Error for ParseClimateError {}
```

The `Error` (`std::error::Error`) is a trait that represent basic expectations for error values. We'll discuss traits in more details [later in this course](course://Generic%20Types,%20Traits,%20and Lifetime/Traits/Traits).
</div>

<div class="hint">
It's not necessary to implement any methods inside the <code>Error</code> implementation. (Some methods have implementations that are supplied by default.)
</div>

<div class="hint">Consult the tests to determine which error variants (and which
error message text) to produce for certain error conditions.</div>

<div class="hint">
You may find these pages to be helpful references:
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/define_error_type.html">1</a>,
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/boxing_errors.html">2</a>, 
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/wrap_error.html">3</a>.
</div>


**Challenge**: There is one test that is marked `#[ignore]`. Can you supply the
missing code that will make it pass? You may want to consult the standard
library documentation for a certain trait for more hints.