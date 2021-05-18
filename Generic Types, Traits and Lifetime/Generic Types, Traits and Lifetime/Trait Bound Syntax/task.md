## Trait Bound Syntax

An imaginary magical school has a new report card generation system written in Rust!
Currently, the system only supports creating report cards where the student's grade
is represented numerically (e.g. 1.0 -> 5.5).
However, the school also issues alphabetical grades (A+ -> F-) and needs
to be able to print both types of report cards!

Make the necessary code changes in the struct `ReportCard` and the `impl` block
to support alphabetical report cards. 

<div class="hint">To find the best solution to this challenge you're going to need to think back to your
knowledge of traits, specifically Trait Bound Syntax. You may also need this: <code>use std::fmt::Display;</code></div>

<div class="hint">This is definitely harder than the last two exercises! You need to think about not only making the
<code>ReportCard</code> struct generic, but also the correct property - you will need to change the implementation
of the struct slightly too... you can do it!</div>