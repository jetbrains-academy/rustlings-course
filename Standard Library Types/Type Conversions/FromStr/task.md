## Implement FromStr

This does practically the same thing that `TryFrom<&str>` does.
Additionally, upon implementing `FromStr`, you can use the `parse` method
on strings to generate an object of the implementor type.
You can read more about it at https://doc.rust-lang.org/std/str/trait.FromStr.html

Steps:
1. If the length of the provided string is 0, then return an error.
2. Split the given string on the commas present in it.
3. Only 2 elements should be returned from the split, otherwise return an error.
4. Extract the first element from the split operation and use it as the name.
5. Extract the other element from the split operation and parse it into a `usize` as the age
with something like `"4".parse::<usize>()`.
6. If while parsing the age, something goes wrong, an error should be returned.
   If everything goes well, return a `Result` of a `Person` object.
   
<div class="hint">The implementation of <code>FromStr</code> should return an <code>Ok</code> with a <code>Person</code> object,
or an <code>Err</code> with a string if the string is not valid. </div>

<div class="hint">This is almost like the from_into exercise, but returning errors instead
of falling back to a default value.</div>

<div class="hint">Hint: Look at the test cases to see which error variants to return.</div>

<div class="hint">Another hint: You can use the <code>map_err</code> method of <code>Result</code> with a function
or a closure to wrap the error from <code>parse::&lt;usize&gt;</code>.</div>

<div class="hint">Yet another hint: If you would like to propagate errors by using the <code>?</code>
operator in your solution, you might want to check out <a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/reenter_question_mark.html">this</a> article.</div>

