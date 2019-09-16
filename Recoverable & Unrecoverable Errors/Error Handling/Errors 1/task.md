## Returning Result

This function refuses to generate text to be printed on a nametag if you pass it an empty string.
It'd be nicer if it explained what the problem was, instead of just sometimes returning `None`.

<div class="hint">
  `Err` is one of the variants of `Result`, so what the 2nd test is saying is that `generate_nametag_text` should return a `Result` instead of an `Option`.

  To make this change, you'll need to:
  - update the return type in the function signature to be a Result that could be the variants `Ok(String)` and `Err(String)`
  - change the body of the function to return `Ok(stuff)` where it currently returns `Some(stuff)`
  - change the body of the function to return `Err(error message)` where it currently returns `None`.
</div>
