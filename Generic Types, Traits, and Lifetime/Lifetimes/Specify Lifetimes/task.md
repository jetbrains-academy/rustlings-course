## Specify Lifetimes

The Rust compiler needs to know how to check whether supplied references are
valid so that it can let the programmer know if a reference is at risk
of going out of scope before it is used. Remember, references are borrows
and do not own their own data. What if their owner goes out of scope?

<div class="hint">
Let the compiler guide you. 

Also, take a look at [the book](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html) if you need help.
</div>