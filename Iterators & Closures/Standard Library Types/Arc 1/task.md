## Arc

Make this code compile by filling in a value for `shared_numbers` and creating an initial binding for `child_numbers` somewhere.
Try not to create any copies of the `numbers` Vec!

<div class="hint">
  Make `shared_numbers` be an `Arc` from the numbers vector.
  Then, in order to avoid creating a copy of `numbers`, you'll need to create `child_numbers` inside the loop but still in the main thread.

  `child_numbers` should be a clone of the Arc of the numbers instead of a thread-local copy of the numbers.
</div>
