## Count Progress

Rustling progress is modelled using a hash map. The name of the exercise is
the key and the progress is the value. Two counting functions were created
to count the number of exercises with a given progress. These counting
functions use imperative style for loops. Recreate this counting
functionality using iterators.


Make the code compile and the tests pass.

<div class="hint">Step 1:
The documentation for the <code>std::iter::Iterator</code> trait contains numerous methods
that would be helpful here.
</div>

<div class="hint">
Step 2:
Return 0 from <code>count_stack</code> to make the code compile in order to test count.
</div>

<div class="hint">The <code>stack</code> variable in <code>count_stack</code> is a slice of HashMaps. It needs to be
converted into an iterator in order to use the iterator methods.
</div>

<div class="hint">The <code>fold</code> method can be useful in the <code>count_stack</code> function.</div>