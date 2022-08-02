## Play with `assert!`

Tests are important to ensure that your code does what you think it should do.

This test has a problem with it — make the test compile! Make the test pass!
Make the test fail!

<div class="hint">
  You don't even need to write any code to test — you can just test values and run that, even though you wouldn't do that in real life.
  <code>assert!</code> is a macro that needs an argument.
  Depending on the value of the argument, <code>assert!</code> will do nothing (in which case the test will pass) or <code>assert!</code> will panic (in which case the test will fail).
  So try giving different values  to <code>assert!</code> and see which ones compile, which ones pass, and which ones fail.
</div>
