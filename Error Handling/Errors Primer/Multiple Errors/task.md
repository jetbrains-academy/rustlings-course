## Multiple Errors

This program uses a completed version of the code from the previous task.
It won't compile right now! Why? Use the hints if you're stuck.

<div class="hint">
There are two different possible <code>Result</code> types produced within
<code>main()</code>, which are propagated using <code>?</code> operators. How do we declare a
return type from <code>main()</code> that allows both?</div>

<div class="hint">Another hint: under the hood, the <code>?</code> operator calls <code>From::from</code>
on the error value to convert it to a boxed trait object, a
<code>Box&lt;dyn error::Error></code>, which is polymorphic &mdash; that means that lots of
different kinds of errors can be returned from the same function because
all errors act the same since they all implement the <code>error::Error</code> trait.
Check out the section "A Shortcut for Propagating Errors: the ? Operator" in this <a href="course://Error Handling/Error Handling/Propagating Errors Limitations">task</a>.
</div>

<div class="hint">This exercise uses some concepts that we won't get to until later in the
course, like <code>Box</code> and the <code>From</code> trait. It's not important to understand
them in detail right now, but you can read ahead if you like.
Read more about boxing errors:
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/boxing_errors.html">here</a>.</div>