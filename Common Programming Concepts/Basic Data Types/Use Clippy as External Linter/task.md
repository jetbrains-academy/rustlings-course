## Use Clippy as External Linter

The `Clippy` tool is a collection of lints to analyze your code
so you can catch common mistakes and improve your Rust code. 
To use `Clippy` as an external linter, follow the instructions for [the earlier task](course://Introduction/Getting started/External Linter) but 
this time select **Clippy** instead of **Cargo Check**.

Check clippy's suggestions and apply them to solve the exercise. 
Note that the **Check** button doesn't really check anything in this task. 
Use your own judgement here!


<div class="hint">
Rust stores the highest precision version of any long or inifinite precision
mathematical constants in the <a href="https://doc.rust-lang.org/stable/std/f32/consts/index.html">Rust standard library</a>.


We may be tempted to use our own approximations for certain mathematical constants,
but clippy recognizes those imprecise mathematical constants as a source of
potential error.
See the suggestions of the `clippy` warning in compile output and use the
appropriate replacement constant from `std::f32::consts`.
</div>