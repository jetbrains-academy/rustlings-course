## Returning Result

This function refuses to generate text to be printed on a nametag if you pass it an empty string.
It'd be nicer if it explained what the problem was, instead of just sometimes returning `None`.

<div class="hint">
  <code>Err</code> is one of the variants of <code>Result</code>, so what the 2nd test is saying is that <code>generate_nametag_text</code> should return a <code>Result</code> instead of an <code>Option</code>.

To make this change, you'll need to:
- update the return type in the function signature to be a Result<String, String> that
  could be the variants `Ok(String)` and `Err(String)`
- change the body of the function to return `Ok(stuff)` where it currently
  returns `Some(stuff)`
- change the body of the function to return `Err(error message)` where it
  currently returns `None`
- change the first test to expect `Ok(stuff)` where it currently expects
  `Some(stuff)`.
</div>
