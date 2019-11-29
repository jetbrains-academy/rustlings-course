## Extended Matching Result

This is a program that is trying to use a completed version of the `total_cost` function from the previous exercise.
It's not working though -- we can't use the `?` operator in the `main()` function! Why not?

<div class="hint">
  Since the `?` operator returns an `Err` early if the thing it's trying to do fails, you can only use the `?` operator in functions that have a `Result` as their return type.

  Hence the error that you get if you run this code is:

  ```
  error[E0277]: the `?` operator can only be used in a function that returns `Result` (or another type that implements `std::ops::Try`)
  ```

  So we have to use another way of handling a `Result` within `main`.

  Decide what we should do if `pretend_user_input` has a string value that does not parse to an integer, and implement that instead of using the `?` operator.
</div>
