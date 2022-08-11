## Deal with Division

This is a bigger exercise than most of the others!
You can do it!

Here is your mission, should you choose to accept it:

1. Complete the `divide` function to get the first four tests to pass.
2. Get the remaining tests to pass by completing the result_with_list and
`list_of_results` functions.

Check out the chapter [Iterator](https://doc.rust-lang.org/stable/book/ch13-02-iterators.html) of the Rust Book and the [Iterator documentation](https://doc.rust-lang.org/stable/std/iter/).

Scroll down for a minor hint for part 2, and scroll down further for a major hint.
Have fun :-)

<div class="hint">
  The <code>divide</code> function needs to return the correct error when even division is not
possible.
</div>

<div class="hint">
The <code>division_results</code> variable needs to be collected into a collection type.

The `result_with_list` function needs to return a single `Result` where the success
case is a vector of integers and the failure case is a `DivisionError`.

The `list_of_results` function needs to return a vector of results.
</div>
