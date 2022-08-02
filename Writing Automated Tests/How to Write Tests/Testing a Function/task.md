## Test a Function

This test isn't testing our function — make it do that in such a way that the test passes.
Then write a second test function called `is_false_when_odd` that tests whether we get the result we expect to get when we call `is_even(5)`.

<div class="hint">
  You can call a function right where you're passing arguments to <code>assert!</code> — so you could do something like:

```rust
assert!(having_fun());
```


  If you want to check that you indeed get false, you can negate the result of what you're doing using `!`, like:
  
```rust
assert!(!having_fun());
```
</div>
