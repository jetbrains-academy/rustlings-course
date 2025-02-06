## Print Months Once Again

Something strange is going on here. The program is compiled without any problems. We have `fn print_months` which borrows an array and prints its content. That's fine. What about `fn print_months_reversed`? Doesn't it take an ownership over the `months` array? Why can we use the same `months` again after that then? Well, no, ownership isn't moved here. The value of an array is copied instead. If you run the program, you'll see that immediately. 

Refactor this program to avoid array copying by switching to a mutable reference in `fn print_months_reversed` and fix the compiler errors.

<div class="hint">
  Remember, as we discussed in <a href="course://Common Programming Concepts/Tuples and Arrays/The Array Type">The Array Type</a> task, arrays are allocated on the stack.
</div>

<div class="hint">
The first step would be to change the type of argument in <code>fn print_months_reversed</code>.
</div>

<div class="hint">
How should we call the function that borrows its argument mutably?
</div>