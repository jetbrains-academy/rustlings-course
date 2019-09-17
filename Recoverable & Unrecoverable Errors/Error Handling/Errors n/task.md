## Multiple Errors

This is a bigger error exercise than the previous ones!
You can do it! :)

Edit the `read_and_validate` function so that it compiles and passes the tests... so many things could go wrong!

- Reading from stdin could produce an io::Error
- Parsing the input could produce a num::ParseIntError
- Validating the input could produce a CreationError (defined below)

How can we lump these errors into one general error?
That is, what type goes where the question marks are, and how do we return that type from the body of read_and_validate?

<div class="hint">
  To figure out what type should go where the ??? is, take a look at the test helper function `test_with_str`, since it returns whatever `read_and_validate` returns and`test_with_str` has its signature fully specified.
</div>

<div class="hint">
  There are three places in `read_and_validate` that we call a function that returns a `Result` (that is, the functions might fail).

  Apply the `?` operator on those calls so that we return immediately from `read_and_validate` if those function calls fail.
</div>

<div class="hint">
  Under the hood, the `?` operator calls `From::from` on the error value to convert it to a boxed trait object, a Box<error::Error>, which is polymorphic -- that means that lots of different kinds of errors can be returned from the same function because all errors act the same since they all implement the `error::Error` trait.

  Check out this section of the book:
  https://doc.rust-lang.org/stable/book/second-edition/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
</div>

<div class="hint">
  Note that because the `?` operator returns the *unwrapped* value in the `Ok` case, if we want to return a `Result` from `read_and_validate` for *its* success case, we'll have to rewrap a value that we got from the return value of a `?`ed call in an `Ok` -- this will look like `Ok(something)`.
</div>

<div class="hint">
  `Result`s must be "used", that is, you'll get a warning if you don't handle a `Result` that you get in your function.

  Read more about that in the `std::result` module docs:
  https://doc.rust-lang.org/std/result/#results-must-be-used
</div>
