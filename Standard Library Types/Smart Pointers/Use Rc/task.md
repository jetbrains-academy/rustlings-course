## Use `Rc`

In this exercise, we want to express the concept of multiple owners via the `Rc<T>` type.
This is a model of our solar system - there is a `Sun` type and multiple `Planet`s.
The `Planet`s take ownership of the sun, indicating that they revolve around the sun.

Make this code compile by using the proper `Rc` primitives to express that the sun has multiple owners.

<div class="hint">
This is a straightforward exercise to use the <code>Rc<T></code> type. Each <code>Planet</code> has
ownership of the <code>Sun</code>, and uses <code>Rc::clone()</code> to increment the reference count of the <code>Sun</code>.
After using <code>drop()</code> to move the Planets out of scope individually, the reference count goes down.
In the end the sun only has one reference again, to itself. <a href="https://doc.rust-lang.org/book/ch15-04-rc.html">See more in The Book</a>.


</div>