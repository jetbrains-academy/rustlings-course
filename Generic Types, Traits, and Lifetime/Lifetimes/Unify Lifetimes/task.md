## Unify Lifetimes

So if the compiler is just validating the references passed
to the annotated parameters and the return type, what do
we need to change?

<div class="hint">
Remember that the generic lifetime <code>'a</code> will get the concrete 
lifetime that is equal to the smaller of the lifetimes of <code>x</code> and <code>y</code>.
</div>

<div class="hint">
You can take at least two paths to achieve the desired result while keeping the inner block:

1. Move the `string2` declaration to make it live as long as `string1` (how is `result` declared?)
2. Move `println!` into the inner block.

</div>


