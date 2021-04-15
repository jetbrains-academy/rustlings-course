## Iterator

This is a bigger exercise than most of the others!
You can do it!

Here is your mission, should you choose to accept it:

1. Complete the divide function to get the first four tests to pass.
2. Uncomment the last two tests and get them to pass by filling in values for `x` using `division_results`.

Check out the chapter [Iterator](https://doc.rust-lang.org/book/2018-edition/ch13-02-iterators.html) of the Rust Book and the [Iterator documentation](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.htmlj).

Scroll down for a minor hint for part 2, and scroll down further for a major hint.
Have fun :-)

<div class="hint">
  In each of the two cases in the match in main, you can create x with either a 'turbofish' or by hinting the type of x to the compiler.
  You may try both.
</div>

<div class="hint">
  Have a look at the <code>Iter</code> trait and at the explanation of its <code>collect</code> function.
  Especially interesting is the part about <code>Result</code>.
</div>
