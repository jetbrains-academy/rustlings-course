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
To figure out what type should go where the ??? is, take a look
at the test helper function <code>test_with_str</code>, since it returns whatever
<code>read_and_validate</code> returns and <code>test_with_str</code> has its signature fully
specified.
</div>

<div class="hint">
There are three places in <code>read_and_validate</code> that we call a
function that returns a <code>Result</code> (that is, the functions might fail).

Apply the <code>?</code> operator on those calls so that we return immediately from
<code>read_and_validate</code> if those function calls fail.
</div>

<div class="hint">
Under the hood, the <code>?</code> operator calls <code>From::from</code>
on the error value to convert it to a boxed trait object, a <code>Box&lt;dyn error::Error&gt;</code>,
which is polymorphic –– that means that lots of different kinds of errors
can be returned from the same function because all errors act the same
since they all implement the <code>error::Error</code> trait.

Check out this section of the book:
<a href="">https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator </a>
</div>

<div class="hint">
Note that because the <code>?</code> operator returns
the *unwrapped* value in the `Ok` case, if we want to return a <code>Result</code> from
<code>read_and_validate</code> for *its* success case, we'll have to rewrap a value
that we got from the return value of a <code>?</code>ed call in an <code>Ok</code> –– this will
look like <code>Ok(something)</code>.</div>

<div class="hint">
<code>Result</code>s must be "used", that is, you'll
get a warning if you don't handle a <code>Result</code> that you get in your
function. 

Read more about that in the <code>std::result</code> module docs:
<a  href="">https://doc.rust-lang.org/std/result/#results-must-be-used </a>
</div>
