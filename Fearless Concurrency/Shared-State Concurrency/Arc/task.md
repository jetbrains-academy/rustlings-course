## Use `Arc`

Make this code compile by filling in a value for `shared_numbers` and creating an initial binding for `child_numbers` somewhere.
Try not to create any copies of the `numbers` Vec!

Check out the chapter [Shared-State Concurrency](https://doc.rust-lang.org/book/2018-edition/ch16-03-shared-state.html) of the Rust Book.

<div class="hint">

  Make `shared_numbers` be an `Arc` from the numbers vector.
  Then, in order to avoid creating a copy of `numbers`, you'll need to create `child_numbers` inside the loop but still in the main thread.

  `child_numbers` should be a clone of the Arc of the numbers instead of a thread-local copy of the numbers.
</div>

<div class="hint">This is a simple exercise if you understand the underlying concepts, but if this
is too much of a struggle, consider reading (or completing) the "Fearless Concurrency" section first. Alternatively, read through all of <a href="https://doc.rust-lang.org/stable/book/ch16-00-concurrency.html">Chapter 16</a> in the Book.
</div>