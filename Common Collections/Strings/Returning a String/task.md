## Returning a String

Make me compile without changing the function signature!

<div class="hint">
  The `current_favorite_color` function is currently returning a string slice with the `'static` lifetime.
  We know this because the data of the string lives in our code itself -- it doesn't come from a file or user input or another program -- so it will live as long as our program lives.
  But it is still a string slice.
  There's one way to create a `String` by converting a string slice covered in the Strings chapter of the book, and another way that uses the `From` trait.
</div>
