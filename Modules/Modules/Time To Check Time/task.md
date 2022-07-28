## Time To Check Time

You can use the `use` keyword to bring module paths from modules from anywhere
and especially from the Rust standard library into your scope.
Bring `SystemTime` and `UNIX_EPOCH`
from the `std::time` module. Bonus style points if you can do it with one line!


Make the code compile! 

<div class="hint">

`UNIX_EPOCH` and `SystemTime` are declared in the `std::time` module. Add a `use` statement
for these two to bring them into scope. You can use nested paths or the `glob`
operator to bring these two in using only one line.

</div>

