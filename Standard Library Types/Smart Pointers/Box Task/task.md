## Use a Box

At compile time, Rust needs to know how much space a type takes up. This becomes problematic
for recursive types, where a value can have as part of itself another value of the same type.
To get around the issue, we can use a `Box` - a smart pointer used to store data on the heap,
which also allows us to wrap a recursive type.

The recursive type we're implementing in this exercise is the `cons list` - a data structure
frequently found in functional programming languages. Each item in a cons list contains two
elements: the value of the current item and the next item. The last item is a value called `Nil`.

**Step 1**: use a `Box` in the enum definition to make the code compile

**Step 2**: create both empty and non-empty cons lists by replacing `unimplemented!()`

Check out the hints if you're stuck! :) 

<div class="hint">Step 1:
The compiler's message should help: since we cannot store the value of the actual type
when working with recursive types, we need to store a reference (pointer) to its value.
We should, therefore, place our <code>List</code> inside a <code>Box</code>. More details in the book here:
<a href="https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes">https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes</a>.
</div>

<div class="hint">
Step 2:
Creating an empty list should be fairly straightforward.
For a non-empty list, keep in mind that we want to use our Cons "list builder".
Although the current list is one of integers (i32), feel free to change the definition
and try other types!</div>