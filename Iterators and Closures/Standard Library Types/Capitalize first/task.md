## Capitalize First

In this module, you'll learn some of the unique advantages that iterators can offer.

Step 1. Complete the `capitalize_first` function to pass the first two tests. It has to return "Hello" when the input is "hello" and an empty string when the input is an empty string.

Step 2. Apply the `capitalize_first` function to a vector of strings.
Ensure that it returns a vector of strings as well.

Step 3. Apply the `capitalize_first` function again to a list.
Try to ensure it returns a single string.

As always, there are hints!

<div class="hint">You need to call something on `first` before it can be collected
Currently its type is `char`. Have a look at the methods that are available on that type:
<a href="https://doc.rust-lang.org/std/primitive.char.html">https://doc.rust-lang.org/std/primitive.char.html</a>.</div>

<div class="hint">First you'll need to turn the Vec into an iterator
Then you'll need to apply your function unto each item in the vector
P.s. Don't forget to <code>collect()</code> at the end!</div>

<div class="hint">This is very similar to the previous test. The only real change is that you will need to
alter the type that collect is coerced into. For a bonus you could try doing this with a
turbofish.</div>