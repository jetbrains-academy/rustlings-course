## Outside of Module

Make me compile, without taking the macro out of the module!

<div class="hint">
  In order to use a macro outside of its module, you need to do something special to the module to lift the macro out into its parent.

  The same trick also works on `extern crate` statements for crates that have exported macros, if you've seen any of those around.
</div>
