## Capitalize First

In this module, you'll learn some of the unique advantages that iterators can offer.

Step 1. Complete the `capitalize_first` function to pass the first two tests. It has to return "Hello" when the input is "hello" and an empty string when the input is an empty string.

Step 2. Apply the `capitalize_first` function to a slice of string slices.
Return a vector of strings.
`["hello", "world"]` -> `["Hello", "World"]`

Step 3. Apply the `capitalize_first` function again to a slice of string slices.
Return a single string.
`["hello", " ", "world"]` -> `"Hello World"`

As always, there are hints!

<div class="hint">
The variable <code>first</code> is a <code>char</code>. It needs to be capitalized and added to the
remaining characters in <code>c</code> in order to return the correct <code>String</code>.
The remaining characters in <code>c</code> can be viewed as a string slice using the
<code>as_str</code>method.
The documentation for <code>char</code> contains many useful methods.
<a href="https://doc.rust-lang.org/std/primitive.char.html">https://doc.rust-lang.org/std/primitive.char.html</a>.</div>

<div class="hint">Create an iterator from the slice. Transform the iterated values by applying
the <code>capitalize_first</code> function. Remember to collect the iterator.</div>

<div class="hint">This is surprisingly similar to the previous solution. Collect is very powerful
and very general. Rust just needs to know the desired type.</div>