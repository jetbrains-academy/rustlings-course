## Move, Borrow, Mutate, or What

Make me compile! Don't change anything except the function's arguments.

<div class="hint">
Note that <code>fn get_last_char</code> shouldn't take ownership 
because <code>message</code> is used later in the <code>main</code> function. 
Instead, you could use reference for borrowing it there.
</div>

<div class="hint">
Note that <code>fn uppercase_and_print</code> is the last call in <code>main</code>, so you can safely move
<code>message</code> there. Hence, there is no need in passing a reference. 

Note also, that this function mutates the `message` variable. So you should define it as mutable.
</div>
