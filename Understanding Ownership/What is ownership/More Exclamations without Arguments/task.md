## More Exclamations without Arguments

Refactor this code so that instead of having `Hello` and creating the string in `fn main`, we create it within `fn with_exclamation` and transfer the freshly created string from `fn with_exclamation` to its caller.

<div class="hint">
  Stop reading whenever you feel like you have enough direction :)
  Or try doing one step and then fixing the compiler errors that result!

So the procedure is the following:
- get rid of the first line in `main` that creates the new string
- since `Hello` doesn't exist now, we can't pass it to `with_exclamation`
- since we don't want to pass anything to `with_exclamation`, its signature should reflect that it does not take any arguments
- since we're not creating a new string in `main` anymore, we need to create a new string in `with_exclamation`, similarly to the way we did in `main`
</div>
