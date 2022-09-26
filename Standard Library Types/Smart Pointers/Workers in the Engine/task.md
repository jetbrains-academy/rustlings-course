## Workers in the Engine

Make me compile!

Though it would be a good idea to read the explanations first.

`Engine` contains a vector of `Worker`s. Initially this vector is empty, but we can easily add any number of workers. Once the engine has enough workers, we can run it. The engine runs its workers. Workers do their work and write to the `log`. What is this `log` and where does it come from? Well, it's just a vector of `String` in the engine. Do you see the problem?

Workers require writable access to the log, but Rust would never allow us to have several mutable references to the log. Throwing `mut` here and there wouldn't work, unfortunately. We need something called [interior mutability](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#interior-mutability-a-mutable-borrow-to-an-immutable-value) and [`RefCell`s](https://doc.rust-lang.org/std/cell/struct.RefCell.html) to give workers temporary writable access to the engine's log. 

There is one more problem. The `Worker` struct keeps a reference to a log. We know that references in `struct`s require specifying lifetimes, but don't try to follow the compiler suggestions. They won't lead you anywhere. The real problem is that workers and the engine have different lifetimes. It'd be impossible to persuade the borrow checker that everything is fine by specifying lifetimes. Instead, we should use [Rc&lt;T> smart pointers](https://doc.rust-lang.org/book/ch15-04-rc.html).

Edit the `worker` and `engine` modules so that the program compiles, runs, and passes the tests. Don't edit `main.rs`. It should work as it is.

<div class="hint">
Instead of following the compiler suggestions, your first step could be to change the type of the log to <code>Rc&lt;RefCell&lt;Vec&lt;String>>></code> (smart pointer to <code>RefCell</code> with a vector of strings) in both <code>worker</code> and <code>engine</code> modules. Then you will need to fix the rest of the code.
</div>

<div class="hint">
You may need the following methods:

- `Rc::new()`, `Rc::clone()`;
- `RefCell::new()`, `RefCell::borrow()`, and `RefCell::borrow_mut()`.
</div>

<div class="hint">
Be careful not to destroy the log after printing it. Make sure that you refer to log entries by references when you print them in <code>Engine::print_log()</code>.
</div>