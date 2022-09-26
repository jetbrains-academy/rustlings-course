## Switch the Lamp

In this task, we need a single lamp and several external switchers, which are able to operate on the lamp
like in the following scenario:

* create a lamp
* create a switcher for the lamp 
* create another switcher for the same lamp 
* operate on different switchers and make sure that the lamp is on and then off again

Unfortunately, Rust doesn't allow us to have several mutable references to the lamp. If you try to do that 
in `main.rs` you'll see that immediately.

Your goal is to edit the `lamp` and `switchers` modules so that `main` is compiled and runnable. 
One way to do that is to introduce *interior mutability* in `Lamp` via [`std::cell::Cell` type](https://doc.rust-lang.org/std/cell/struct.Cell.html). 

You're not supposed to modify code in `main.rs`. It should work as it is.


<div class="hint">
Keep deleting <code>mut</code> modifiers along the way. For Rust everything should be immutable so that it allows us to have several references to the same value.
</div>

<div class="hint">Consult the <a href="https://doc.rust-lang.org/std/cell/struct.Cell.html">docs</a> to figure out how to access and mutate a value in <code>Cell</code>.</div>
