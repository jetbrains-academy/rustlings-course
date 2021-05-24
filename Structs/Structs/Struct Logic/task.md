## Struct Logic

Structs contain data, but can also have logic. In this exercise we have
defined the `Package` struct, and we want to test some logic attached to it.
Make the code compile and the tests pass!

<div class="hint">
The new method needs to panic if the weight is physically impossible :) How do we do that in Rust?
</div>

<div class="hint">
For is_international: What makes a package international? Seems related to the places it goes through, right?
</div>

<div class="hint">
For calculate_transport_fees: Bigger is more expensive usually, we don't have size, but something may fit the bill here :)
</div>

<div class="hint">
Have a look in <a href="https://doc.rust-lang.org/book/ch05-03-method-syntax.html">The Book</a> to find out more about method implementations. 
</div>
