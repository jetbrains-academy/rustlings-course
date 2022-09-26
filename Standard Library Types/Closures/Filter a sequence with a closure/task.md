## Task: Filter a Sequence with a Closure

You have an array with `i32` elements, a `cap`, and a counter `left_behind`. Your task
is to complete the closure given as a parameter to the `filter` method. The closure
should keep only elements strictly smaller than `cap`. It should also count all the elements
left behind.

<div class="hint">
Read the docs about the <code>filter</code> method to understand what 
those <code>&&</code> in the closure argument are about.
</div>

<div class="hint">
You may want to use the fact that the right-hand side of <code> condition || condition </code> is
evaluated only if the left-hand side is <code>false</code>.
</div>

<div class="hint">
Use <code>{...; condition}</code> to do something while evaluating some logical 
expression.
</div>
