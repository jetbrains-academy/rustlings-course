## Option

This example panics because the second time it calls `pop`, the `vec` is empty, so `pop` returns `None`, and `unwrap` panics if it's called on `None`.
Handle this in a more graceful way than calling `unwrap`!

<div class="hint">
  Try using a `match` statement where the arms are `Some(thing)` and `None`.
  Or set a default value to print out if you get `None` by using the function `unwrap_or`.
  Or use an `if let` statement on the result of `pop()` to both destructure a `Some` value and only print out something if we have a value!
</div>
