## Use `Cow`

This exercise explores the `Cow`, or Clone-On-Write type.
`Cow` is a clone-on-write smart pointer.
It can enclose and provide immutable access to borrowed data, and clone the data lazily when mutation or ownership is required.
The type is designed to work with general borrowed data via the `Borrow` trait.

Look at the `abs_all` function first: it mutates the given slice if it contains negative elements. 

Your goal in each of the four cases (`case1`, `case2`, `case3`, and `case4`) is to decide whether the results
of the calls to the `Cow::from` and `abs_all` functions are either `Cow::Borrowed` or `Cow::Owned`. 

<div class="hint">
Make sure that you understand what exactly is given to <code>Cow::from</code> in each case. 
Is ownership transferred in each of them?
</div>

<div class="hint">
What about a vector in <code>case3</code> and <code>case4</code>, is it borrowed or owned after after the <code>Cow::from</code>?
</div>

<div class="hint">
Since the vector in <code>case4</code> is already owned, the <code>Cow</code> type doesn't need to clone it
even though we need to mutate it.
</div>

<div class="hint">Checkout the <a href="https://doc.rust-lang.org/std/borrow/enum.Cow.html">documentation
on the <code>Cow</code> type</a>.</div>