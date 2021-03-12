## Matching Result

Say we're writing a game where you can buy items with tokens.
All items cost 5 tokens, and whenever you purchase items there is a processing fee of 1 token.
A player of the game will type in how many items they want to buy, and the `total_cost` function will calculate the total number of tokens.
Since the player typed in the quantity, though, we get it as a string -- and they might have typed anything, not just numbers!

Right now, this function isn't handling the error case at all (and isn't handling the success case properly either).
What we want to do is: if we call the `parse` function on a string that is not a number, that function will return a `ParseIntError`, and in that case, we want to immediately return that error from our function and not try to multiply and add.

There are at least two ways to implement this that are both correct -- but one is a lot shorter!
Scroll down for hints to both ways.

<div class="hint">
  One way to handle this is using a <code>match</code>match statement on <code>item_quantity.parse::&lt;i32&gt()</code> where the cases are <code>Ok(something)</code> and <code>Err(something)</code>.
  This pattern is very common in Rust, though, so there's a <code>?</code> operator that does pretty much what you would make that match statement do for you!
  Take a look at  <a href="https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator">this section of the Error Handling chapter</a>
  and give it a try!
</div>
